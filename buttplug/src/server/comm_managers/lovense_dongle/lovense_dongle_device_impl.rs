use crate::{
  core::{errors::ButtplugError, messages::RawReading},
  device::{
    configuration_manager::{DeviceSpecifier, ProtocolDefinition, SerialSpecifier},
    device::{
      BoundedDeviceEventBroadcaster,
      ButtplugDeviceEvent,
      ButtplugDeviceImplCreator,
      DeviceImpl,
      DeviceReadCmd,
      DeviceSubscribeCmd,
      DeviceUnsubscribeCmd,
      DeviceWriteCmd,
    },
    Endpoint,
  },
};
use async_std::{
  prelude::StreamExt,
  sync::{channel, Arc, Mutex, Receiver, Sender},
  task,
};
use async_trait::async_trait;
use broadcaster::BroadcastChannel;
use serialport::{open_with_settings, SerialPort, SerialPortInfo, SerialPortSettings};
use std::{io::ErrorKind, thread, time::Duration};

pub struct LovenseDongleDeviceImplCreator {
  specifier: DeviceSpecifier,
  port_info: SerialPortInfo,
}

impl LovenseDongleDeviceImplCreator {
  pub fn new(port_info: &SerialPortInfo) -> Self {
    Self {
      specifier: DeviceSpecifier::Serial(SerialSpecifier::new_from_name(&port_info.port_name)),
      port_info: port_info.clone(),
    }
  }
}

#[derive(Clone)]
pub struct LovenseDongleDeviceImpl {
  name: String,
  address: String,
  port_receiver: Receiver<Vec<u8>>,
  port_sender: Sender<Vec<u8>>,
  connected: bool,
  event_receiver: BoundedDeviceEventBroadcaster,
}

impl LovenseDongleDeviceImpl {
  pub fn new(
    protocol_def: ProtocolDefinition,
    port_receiver: Receiver<Vec<u8>>,
    port_sender: Sender<Vec<u8>>,
    event_receiver: BoundedDeviceEventBroadcaster,
  ) -> Self {
    Self {
      name: "Lovense Serial Device".to_owned(),
      address: "whatever".to_owned(),
      port_receiver,
      port_sender,
      connected: true,
      event_receiver,
    }
  }
}

#[async_trait]
impl DeviceImpl for LovenseDongleDeviceImpl {
  fn name(&self) -> &str {
    &self.name
  }

  fn address(&self) -> &str {
    &self.address
  }

  fn connected(&self) -> bool {
    self.connected
  }

  fn endpoints(&self) -> Vec<Endpoint> {
    vec![Endpoint::Rx, Endpoint::Tx]
  }

  async fn disconnect(&mut self) {
    self.connected = false;
  }

  fn box_clone(&self) -> Box<dyn DeviceImpl> {
    Box::new((*self).clone())
  }

  fn get_event_receiver(&self) -> BoundedDeviceEventBroadcaster {
    self.event_receiver.clone()
  }

  async fn read_value(&self, _msg: DeviceReadCmd) -> Result<RawReading, ButtplugError> {
    // TODO Should check endpoint validity and length requirements
    if self.port_receiver.is_empty() {
      Ok(RawReading::new(0, Endpoint::Rx, vec![]))
    } else {
      let mut port_receiver = self.port_receiver.clone();
      Ok(RawReading::new(
        0,
        Endpoint::Rx,
        port_receiver.next().await.unwrap(),
      ))
    }
  }

  async fn write_value(&self, msg: DeviceWriteCmd) -> Result<(), ButtplugError> {
    // TODO Should check endpoint validity
    Ok(self.port_sender.send(msg.data).await)
  }

  async fn subscribe(&self, _msg: DeviceSubscribeCmd) -> Result<(), ButtplugError> {
    // TODO Should check endpoint validity
    let mut data_receiver = self.port_receiver.clone();
    let event_sender = self.event_receiver.clone();
    task::spawn(async move {
      loop {
        match data_receiver.next().await {
          Some(data) => {
            info!("Got serial data! {:?}", data);
            event_sender
              .send(&ButtplugDeviceEvent::Notification(Endpoint::Tx, data))
              .await
              .unwrap();
          }
          None => {
            info!("Data channel closed, ending serial listener task");
            break;
          }
        }
      }
    });
    Ok(())
  }

  async fn unsubscribe(&self, _msg: DeviceUnsubscribeCmd) -> Result<(), ButtplugError> {
    unimplemented!();
  }
}
