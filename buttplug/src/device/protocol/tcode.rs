use super::{ButtplugProtocol, ButtplugProtocolCreator};
use crate::{
    create_buttplug_protocol,
    device::{
        configuration_manager::DeviceProtocolConfiguration,
        device::{ButtplugDeviceEvent, DeviceSubscribeCmd, DeviceUnsubscribeCmd},
    },
};
use async_std::prelude::StreamExt;
use async_trait::async_trait;

pub struct TCodeCreator {
  config: DeviceProtocolConfiguration,
}

impl TCodeCreator {
  pub fn new(config: DeviceProtocolConfiguration) -> Self {
      Self { config }
  }
}

#[async_trait]
impl ButtplugProtocolCreator for TCodeCreator {
  async fn try_create_protocol(
      &self,
      device_impl: &Box<dyn DeviceImpl>,
  ) -> Result<Box<dyn ButtplugProtocol>, ButtplugError> {
    info!("Trying to do TCode bringup!");
      device_impl
          .subscribe(DeviceSubscribeCmd::new(Endpoint::Rx).into())
          .await?;
      let identifier;
      match device_impl.get_event_receiver().next().await {
          Some(ButtplugDeviceEvent::Notification(_, n)) => {
              let type_response = std::str::from_utf8(&n).unwrap().to_owned();
              info!("TCode Response: {}", type_response);
              identifier = "/dev/ttyACM0";
          }
          Some(ButtplugDeviceEvent::Removed) => {
              return Err(ButtplugDeviceError::new(
                  "TCode Device disconnected while getting DeviceType info.",
              )
              .into());
          }
          None => {
              return Err(ButtplugDeviceError::new(
                  "Did not get TCode output in time"
              )
              .into());
          }
      };
      device_impl
          .unsubscribe(DeviceUnsubscribeCmd::new(Endpoint::Rx).into())
          .await?;

      let (names, attrs) = self.config.get_attributes(&identifier).unwrap();
      let name = names.get("en-us").unwrap();
      Ok(Box::new(TCode::new(name, attrs)))
  }
}


create_buttplug_protocol!(
    // Protocol name
    TCode,
    // Use the default protocol creator implementation. No special init needed.
    false,
    // No extra members
    (),
    // No implementations. Just see if anything even runs.
    ((VibrateCmd, {Ok(messages::Ok::default().into())}))
);
