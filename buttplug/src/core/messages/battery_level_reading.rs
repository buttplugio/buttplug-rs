// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2020 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

use super::*;
#[cfg(feature = "serialize-json")]
use serde::{Deserialize, Serialize};

#[derive(Debug, ButtplugDeviceMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize-json", derive(Serialize, Deserialize))]
pub struct BatteryLevelReading {
  #[cfg_attr(feature = "serialize-json", serde(rename = "Id"))]
  pub(super) id: u32,
  #[cfg_attr(feature = "serialize-json", serde(rename = "DeviceIndex"))]
  device_index: u32,
  #[cfg_attr(feature = "serialize-json", serde(rename = "BatteryLevel"))]
  battery_level: f64,
}

impl BatteryLevelReading {
  pub fn new(device_index: u32, battery_level: f64) -> Self {
    Self {
      id: 1,
      device_index,
      battery_level,
    }
  }

  pub fn battery_level(&self) -> f64 {
    self.battery_level
  }
}

impl ButtplugMessageValidator for BatteryLevelReading {
  fn is_valid(&self) -> Result<(), ButtplugMessageError> {
    self.is_not_system_id(self.id)?;
    self.is_in_command_range(
      self.battery_level,
      "BatteryLevelReading must be between 0.0 and 1.0".to_string(),
    )
  }
}
