#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

mod traits;
#[cfg(target_os = "linux")]
pub use linux::Listener;
#[cfg(target_os = "linux")]
pub use linux::keys;
#[cfg(target_os = "linux")]
pub use linux::modifiers;
#[cfg(target_os = "macos")]
pub use macos::Listener;
#[cfg(target_os = "macos")]
pub use macos::keys;
#[cfg(target_os = "macos")]
pub use macos::modifiers;
pub use traits::{HotkeyError, HotkeyListener, ListenerHotkey};
#[cfg(target_os = "windows")]
pub use windows::Listener;
#[cfg(target_os = "windows")]
pub use windows::keys;
#[cfg(target_os = "windows")]
pub use windows::modifiers;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn register_unregister_hotkey_test() {
		let mut listener = Listener::new();

		assert_eq!(listener.registered_hotkeys().len(), 0);

		let hotkey1 = ListenerHotkey::new(modifiers::ALT, keys::A);

		assert_eq!(listener.register_hotkey(hotkey1, || {}), Ok(()));

		assert_eq!(listener.registered_hotkeys()[0], hotkey1);

		assert_eq!(listener.registered_hotkeys().len(), 1);

		let hotkey2 = ListenerHotkey::new(modifiers::CONTROL, keys::B);

		assert_eq!(listener.register_hotkey(hotkey2, || {}), Ok(()));

		assert!(listener.registered_hotkeys().iter().any(|h| *h == hotkey1));

		assert!(listener.registered_hotkeys().iter().any(|h| *h == hotkey2));

		assert_eq!(listener.registered_hotkeys().len(), 2);

		assert_eq!(listener.unregister_hotkey(hotkey1), Ok(()));

		assert_eq!(listener.registered_hotkeys().len(), 1);

		assert_eq!(listener.registered_hotkeys()[0], hotkey2);

		assert_eq!(listener.unregister_hotkey(hotkey2), Ok(()));

		assert_eq!(listener.registered_hotkeys().len(), 0);

		let hotkey3 =
			ListenerHotkey::new(modifiers::CONTROL | modifiers::SUPER | modifiers::ALT, keys::P);

		assert_eq!(listener.register_hotkey(hotkey3, || {}), Ok(()));

		assert_eq!(listener.registered_hotkeys()[0], hotkey3);

		assert_eq!(listener.registered_hotkeys().len(), 1);

		assert_eq!(listener.unregister_hotkey(hotkey3), Ok(()));

		assert_eq!(listener.registered_hotkeys().len(), 0);

		assert_eq!(listener.register_hotkey(hotkey1, || {}), Ok(()));

		assert_eq!(listener.registered_hotkeys()[0], hotkey1);

		assert_eq!(listener.registered_hotkeys().len(), 1);

		assert_eq!(listener.unregister_hotkey(hotkey1), Ok(()));

		assert_eq!(listener.registered_hotkeys().len(), 0);
	}

	#[test]
	fn unregister_invalid_hotkey_test() {
		let mut listener = Listener::new();

		assert_eq!(listener.registered_hotkeys().len(), 0);

		let hotkey = ListenerHotkey::new(modifiers::ALT, keys::A);

		assert_eq!(
			listener.unregister_hotkey(hotkey),
			Err(HotkeyError::HotkeyNotRegistered(hotkey))
		);

		assert_eq!(listener.registered_hotkeys().len(), 0);
	}

	#[test]
	fn reregister_hotkey_test() {
		let mut listener = Listener::new();

		assert_eq!(listener.registered_hotkeys().len(), 0);

		let hotkey = ListenerHotkey::new(modifiers::ALT, keys::B);

		assert_eq!(listener.register_hotkey(hotkey, || {}), Ok(()));

		assert_eq!(listener.registered_hotkeys()[0], hotkey);

		assert_eq!(listener.registered_hotkeys().len(), 1);

		assert_eq!(
			listener.register_hotkey(hotkey, || {}),
			Err(HotkeyError::HotkeyAlreadyRegistered(hotkey))
		);

		assert_eq!(listener.registered_hotkeys()[0], hotkey);

		assert_eq!(listener.registered_hotkeys().len(), 1);

		assert_eq!(listener.unregister_hotkey(hotkey), Ok(()));

		assert_eq!(listener.registered_hotkeys().len(), 0);
	}
}
