use super::{ButtplugDeviceResultFuture, ButtplugProtocol, ButtplugProtocolCommandHandler};
use crate::{
  core::{
    errors::ButtplugError,
    messages::{self, ButtplugDeviceCommandMessageUnion, DeviceMessageAttributesMap},
  },
  device::{
    protocol::{generic_command_manager::GenericCommandManager, ButtplugProtocolProperties},
    DeviceImpl,
    DeviceWriteCmd,
    Endpoint,
  },
};
use std::sync::Arc;
use futures::future::BoxFuture;
use tokio::sync::Mutex;

#[derive(ButtplugProtocolProperties)]
pub struct EarHaptics {
  name: String,
  message_attributes: DeviceMessageAttributesMap,
  manager: Arc<Mutex<GenericCommandManager>>,
  stop_commands: Vec<ButtplugDeviceCommandMessageUnion>,
}

impl ButtplugProtocol for EarHaptics {
  fn new_protocol(
    name: &str,
    message_attributes: DeviceMessageAttributesMap,
  ) -> Box<dyn ButtplugProtocol>
  where
    Self: Sized,
  {
    let manager = GenericCommandManager::new(&message_attributes);

    Box::new(Self {
      name: name.to_owned(),
      message_attributes,
      stop_commands: manager.get_stop_commands(),
      manager: Arc::new(Mutex::new(manager)),
    })
  }

  fn initialize(device_impl: Arc<DeviceImpl>) -> BoxFuture<'static, Result<Option<String>, ButtplugError>>
  where
      Self: Sized, {
    Box::pin(async move {
    if device_impl.name().contains("1") {
      Ok(Some("Ear1".to_owned()))
    } else {
      Ok(Some("Ear2".to_owned()))
    }
  })
  }
}

impl ButtplugProtocolCommandHandler for EarHaptics {
  fn handle_vibrate_cmd(
    &self,
    device: Arc<DeviceImpl>,
    message: messages::VibrateCmd,
  ) -> ButtplugDeviceResultFuture {
    // Store off result before the match, so we drop the lock ASAP.
    let manager = self.manager.clone();
    Box::pin(async move {
      let result = manager.lock().await.update_vibration(&message, false)?;
      let mut fut_vec = vec![];
      if let Some(cmds) = result {
        for (index, cmd) in cmds.iter().enumerate() {
          if let Some(speed) = cmd {
            fut_vec.push(device.write_value(DeviceWriteCmd::new(
              Endpoint::Tx,
              vec![*speed as u8],
              false,
            )));
          }
        }
      }
      // TODO Just use join_all here
      for fut in fut_vec {
        // TODO Do something about possible errors here
        fut.await?;
      }
      Ok(messages::Ok::default().into())
    })
  }
}