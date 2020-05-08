use super::lovense_dongle_messages::{
  LovenseDongleIncomingMessage, LovenseDongleMessageFunc, LovenseDongleMessageType,
  LovenseDongleOutgoingMessage,
};
use super::LovenseDongleDeviceImplCreator;
use crate::{
  core::errors::ButtplugError,
  server::comm_managers::{
    DeviceCommunicationEvent, DeviceCommunicationManager, DeviceCommunicationManagerCreator,
  },
};
use async_std::{
  prelude::StreamExt,
  sync::{channel, Arc, Mutex, Receiver, Sender},
  task,
};
use async_trait::async_trait;
use serialport::{
  available_ports, open_with_settings, SerialPort, SerialPortSettings, SerialPortType,
};
use std::{io::ErrorKind, thread, time::Duration};

enum OutgoingLovenseData {
  Raw(String),
  Message(LovenseDongleOutgoingMessage),
}

enum IncomingLovenseData {
  Raw(String),
  Message(LovenseDongleIncomingMessage),
}

fn serial_write_thread(mut port: Box<dyn SerialPort>, mut receiver: Receiver<OutgoingLovenseData>) {
  let mut port_write = |mut data: String| {
    data += "\r\n";
    info!("{}", data);
    port.write(&data.into_bytes()).unwrap();
  };

  task::block_on(async move {
    loop {
      match receiver.next().await {
        Some(v) => match v {
          OutgoingLovenseData::Raw(s) => {
            port_write(s);
          }
          OutgoingLovenseData::Message(m) => {
            port_write(serde_json::to_string(&m).unwrap());
          }
        },
        None => break,
      }
    }
  });
  info!("EXITING LOVENSE DONGLE WRITE THREAD.");
}

fn serial_read_thread(mut port: Box<dyn SerialPort>, sender: Sender<IncomingLovenseData>) {
  let mut data: String = String::default();
  loop {
    // TODO This is probably too small
    let mut buf: [u8; 1024] = [0; 1024];
    match port.read(&mut buf) {
      Ok(len) => {
        info!("Got {} serial bytes", len);
        data += std::str::from_utf8(&buf[0..len]).unwrap();
        if data.contains("\n") {
          // We have what should be a full message.
          // Split it.
          let msg_vec: Vec<&str> = data.split("\n").collect();

          let incoming = msg_vec[0];
          task::block_on(async {
            match serde_json::from_str::<LovenseDongleIncomingMessage>(incoming) {
              Ok(m) => {
                info!("{:?}", m);
                sender.send(IncomingLovenseData::Message(m)).await;
              }
              Err(e) => {
                error!("{:?}", e);
                sender
                  .send(IncomingLovenseData::Raw(incoming.to_string()))
                  .await;
              }
            }
          });

          // Save off the extra.
          if msg_vec.len() > 1 {
            data = msg_vec[1].to_string();
          } else {
            data = String::default();
          }
        }
      }
      Err(e) => {
        if e.kind() == ErrorKind::TimedOut {
          continue;
        }
        error!("{:?}", e);
        break;
      }
    }
  }
  info!("EXITING LOVENSE DONGLE READ THREAD.");
}

pub struct LovenseDongleCommunicationManager {
  sender: Sender<DeviceCommunicationEvent>,
  //port: Arc<Mutex<Option<Box<dyn SerialPort>>>>,
  read_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
  write_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
  // We either have to make our receiver internally mutable, or make
  // EVERYTHING mut across read/write on the trait. So we'll do internal
  // mutability here for now since it already works everywhere else.
  port_receiver: Receiver<IncomingLovenseData>,
  port_sender: Sender<OutgoingLovenseData>,
}

impl DeviceCommunicationManagerCreator for LovenseDongleCommunicationManager {
  fn new(sender: Sender<DeviceCommunicationEvent>) -> Self {
    info!("Lovense dongle serial port created!");
    let (port_sender, _) = channel(256);
    let (_, port_receiver) = channel(256);
    Self {
      sender,
      //port: Arc::new(Mutex::new(None)),
      read_thread: Arc::new(Mutex::new(None)),
      write_thread: Arc::new(Mutex::new(None)),
      port_receiver,
      port_sender,
    }
  }
}

#[async_trait]
impl DeviceCommunicationManager for LovenseDongleCommunicationManager {
  async fn start_scanning(&mut self) -> Result<(), ButtplugError> {
    info!("Lovense Dongle Manager scanning ports!");
    // First off, see if we can actually find a Lovense dongle. If we already
    // have one, skip on to scanning. If we can't find one, send message to log
    // and stop scanning.
    //
    // TODO Does this block? Should it run in one of our threads?
    match available_ports() {
      Ok(ports) => {
        info!("Got {} serial ports back", ports.len());
        for p in ports {
          if let SerialPortType::UsbPort(usb_info) = p.port_type {
            // Hardcode the dongle VID/PID for now. We can't really do protocol
            // detection here because this is a comm bus to us, not a device.
            if usb_info.vid == 0x1a86 && usb_info.pid == 0x7523 {
              // We've found a dongle.
              info!("Found lovense dongle, connecting");
              let mut settings = SerialPortSettings::default();
              // Default is 8/N/1 but we'll need to set the baud rate
              settings.baud_rate = 115200;
              // Set our timeout at ~2hz. Would be nice if this was async, but oh well.
              settings.timeout = Duration::from_millis(500);
              match open_with_settings(&p.port_name, &settings) {
                Ok(dongle_port) => {
                  let (writer_sender, writer_receiver) = channel::<OutgoingLovenseData>(256);
                  let (reader_sender, reader_receiver) = channel::<IncomingLovenseData>(256);

                  let read_port = (*dongle_port).try_clone().unwrap();
                  let read_thread = thread::Builder::new()
                    .name("Serial Reader Thread".to_string())
                    .spawn(move || {
                      serial_read_thread(read_port, reader_sender);
                    })
                    .unwrap();

                  let write_port = (*dongle_port).try_clone().unwrap();
                  let write_thread = thread::Builder::new()
                    .name("Serial Writer Thread".to_string())
                    .spawn(move || {
                      serial_write_thread(write_port, writer_receiver);
                    })
                    .unwrap();
                  *(self.read_thread.lock().await) = Some(read_thread);
                  *(self.write_thread.lock().await) = Some(write_thread);
                  self.port_receiver = reader_receiver;
                  self.port_sender = writer_sender;
                  let scan_msg = LovenseDongleOutgoingMessage {
                    message_type: LovenseDongleMessageType::USB,
                    func: LovenseDongleMessageFunc::Search,
                    eager: None,
                    id: None,
                    command: None,
                  };
                  //self.port_sender.send(OutgoingLovenseData::Raw("DeviceType;".to_string())).await;
                  self
                    .port_sender
                    .send(OutgoingLovenseData::Message(scan_msg))
                    .await;
                }
                Err(e) => error!("{:?}", e),
              };
            }
          }
        }
      }
      Err(_) => {
        info!("No serial ports found");
      }
    }
    Ok(())
  }

  async fn stop_scanning(&mut self) -> Result<(), ButtplugError> {
    Ok(())
  }

  fn is_scanning(&mut self) -> bool {
    false
  }
}
