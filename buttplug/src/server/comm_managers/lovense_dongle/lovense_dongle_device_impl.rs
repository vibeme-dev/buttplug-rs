use crate::{
  core::{errors::ButtplugError, messages::RawReading},
  device::{
    configuration_manager::{DeviceSpecifier, ProtocolDefinition, BluetoothLESpecifier},
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
use super::{
  lovense_dongle_messages::{LovenseDongleIncomingMessage, LovenseDongleOutgoingMessage, LovenseDongleMessageFunc, LovenseDongleMessageType},
  lovense_dongle_comm_manager::OutgoingLovenseData
};
use async_std::{
  prelude::StreamExt,
  sync::{channel, Arc, Mutex, Receiver, Sender},
  task,
};
use async_trait::async_trait;
use broadcaster::BroadcastChannel;

pub struct LovenseDongleDeviceImplCreator {
  specifier: DeviceSpecifier,
  id: String,
  port_sender: Sender<OutgoingLovenseData>,
}

impl LovenseDongleDeviceImplCreator {
  pub fn new(id: &str, port_sender: Sender<OutgoingLovenseData>) -> Self {
    Self {
      // We know the only thing we'll ever get from a lovense dongle is a
      // lovense device. However, we don't have a way to specify that in our
      // device config file. Therefore, we just lie and act like it's a
      // bluetooth device with a name that will match the Lovense builder. Then
      // when we get the device, we can set up as we need.
      //
      // Hacky, but it works.
      specifier: DeviceSpecifier::BluetoothLE(BluetoothLESpecifier::new_from_device("LVS-SerialPortDevice")),
      id: id.to_string(),
      port_sender
    }
  }
}

#[async_trait]
impl ButtplugDeviceImplCreator for LovenseDongleDeviceImplCreator {
  fn get_specifier(&self) -> DeviceSpecifier {
    self.specifier.clone()
  }

  async fn try_create_device_impl(
    &mut self,
    protocol: ProtocolDefinition,
  ) -> Result<Box<dyn DeviceImpl>, ButtplugError> {
    info!("RUNNING DEVICE IMPL CREATION");
    let outgoing_msg = LovenseDongleOutgoingMessage {
      func: LovenseDongleMessageFunc::Connect,
      message_type: LovenseDongleMessageType:: Toy,
      id: Some(self.id.clone()),
      command: None,
      eager: None
    };
    task::block_on(async {
      self.port_sender.send(OutgoingLovenseData::Message(outgoing_msg)).await;
      task::sleep(std::time::Duration::from_millis(1000)).await;
    });

    Ok(Box::new(LovenseDongleDeviceImpl::new(&self.id, self.port_sender.clone())))
  }
}

#[derive(Clone)]
pub struct LovenseDongleDeviceImpl {
  name: String,
  address: String,
  port_receiver: Receiver<LovenseDongleIncomingMessage>,
  port_sender: Sender<OutgoingLovenseData>,
  connected: bool,
  event_receiver: BoundedDeviceEventBroadcaster,
}

impl LovenseDongleDeviceImpl {
  pub fn new(
    address: &String,
    port_sender: Sender<OutgoingLovenseData>,
  ) -> Self {
    let (_, port_receiver) = channel(256);
    Self {
      name: "Lovense Serial Device".to_owned(),
      address: address.to_string(),
      port_receiver,
      port_sender,
      connected: true,
      event_receiver: BroadcastChannel::with_cap(256),
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
    unimplemented!()
  }

  async fn write_value(&self, msg: DeviceWriteCmd) -> Result<(), ButtplugError> {
    info!("Writing to lovense device: {}", std::str::from_utf8(&msg.data).unwrap().to_string());
    let outgoing_msg = LovenseDongleOutgoingMessage {
      func: LovenseDongleMessageFunc::Command,
      message_type: LovenseDongleMessageType:: Toy,
      id: Some(self.address.clone()),
      command: Some(std::str::from_utf8(&msg.data).unwrap().to_string()),
      eager: None
    };
    self.port_sender.send(OutgoingLovenseData::Message(outgoing_msg)).await;
    Ok(())
  }

  async fn subscribe(&self, _msg: DeviceSubscribeCmd) -> Result<(), ButtplugError> {
    Ok(())
  }

  async fn unsubscribe(&self, _msg: DeviceUnsubscribeCmd) -> Result<(), ButtplugError> {
    unimplemented!();
  }
}
