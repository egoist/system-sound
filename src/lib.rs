#![deny(clippy::all)]

use napi::Result;
use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[cfg(target_os = "macos")]
use cocoa::base::{id, nil};
#[cfg(target_os = "macos")]
use cocoa::foundation::NSString;
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

#[napi]
pub enum SystemSoundName {
  Tink,
  Pop,
  Glass,
  Hero,
  Morse,
  Ping,
  Purr,
  Sosumi,
  Submarine,
  Blow,
  Bottle,
  Frog,
  Funk,
  Basso,
}

impl SystemSoundName {
  pub fn as_str(&self) -> &str {
    match self {
      SystemSoundName::Tink => "Tink",
      SystemSoundName::Pop => "Pop",
      SystemSoundName::Glass => "Glass",
      SystemSoundName::Hero => "Hero",
      SystemSoundName::Morse => "Morse",
      SystemSoundName::Ping => "Ping",
      SystemSoundName::Purr => "Purr",
      SystemSoundName::Sosumi => "Sosumi",
      SystemSoundName::Submarine => "Submarine",
      SystemSoundName::Blow => "Blow",
      SystemSoundName::Bottle => "Bottle",
      SystemSoundName::Frog => "Frog",
      SystemSoundName::Funk => "Funk",
      SystemSoundName::Basso => "Basso",
    }
  }
}

pub struct NSSound;

impl NSSound {
  /// Play a system sound by name
  /// Common system sounds: "Tink", "Pop", "Glass", "Hero", "Morse", "Ping", "Purr", "Sosumi", "Submarine", "Blow", "Bottle", "Frog", "Funk", "Basso"
  /// Note: Only works on macOS, no-op on other platforms
  #[cfg(target_os = "macos")]
  pub fn play(sound_name: &str) -> Result<()> {
    unsafe {
      let sound_name_ns = NSString::alloc(nil).init_str(sound_name);
      let sound: id = msg_send![class!(NSSound), soundNamed: sound_name_ns];

      if sound == nil {
        return Err(napi::Error::from_reason(format!(
          "Sound '{}' not found",
          sound_name
        )));
      }

      let _: () = msg_send![sound, play];

      Ok(())
    }
  }

  #[cfg(not(target_os = "macos"))]
  pub fn play(_sound_name: &str) -> Result<()> {
    // No-op on non-macOS platforms
    Ok(())
  }
}

#[napi]
pub fn play_system_sound(sound_name: SystemSoundName) -> Result<()> {
  NSSound::play(sound_name.as_str())
}
