use std::{
	collections::HashMap,
	mem,
	ptr,
	sync::{
		Arc,
		Mutex,
		mpsc,
		mpsc::{Receiver, Sender},
	},
};

use x11_dl::xlib;

use super::traits::*;

pub mod modifiers {
	use x11_dl::{keysym, xlib};

	pub const ALT:u32 = xlib::Mod1Mask;

	pub const ALT_GR:u32 = keysym::XK_Alt_R;

	pub const CONTROL:u32 = xlib::ControlMask;

	pub const SHIFT:u32 = xlib::ShiftMask;

	pub const SUPER:u32 = xlib::Mod4Mask;
}

pub mod keys {
	use x11_dl::keysym;

	pub const BACKSPACE:u32 = keysym::XK_BackSpace;

	pub const TAB:u32 = keysym::XK_Tab;

	pub const ENTER:u32 = keysym::XK_Return;

	pub const CAPS_LOCK:u32 = keysym::XK_Caps_Lock;

	pub const ESCAPE:u32 = keysym::XK_Escape;

	pub const SPACEBAR:u32 = keysym::XK_space;

	pub const PAGE_UP:u32 = keysym::XK_Page_Up;

	pub const PAGE_DOWN:u32 = keysym::XK_Page_Down;

	pub const END:u32 = keysym::XK_End;

	pub const HOME:u32 = keysym::XK_Home;

	pub const ARROW_LEFT:u32 = keysym::XK_Left;

	pub const ARROW_RIGHT:u32 = keysym::XK_Right;

	pub const ARROW_UP:u32 = keysym::XK_Up;

	pub const ARROW_DOWN:u32 = keysym::XK_Down;

	pub const PRINT_SCREEN:u32 = keysym::XK_Print;

	pub const CLEAR:u32 = keysym::XK_Clear;

	pub const INSERT:u32 = keysym::XK_Insert;

	pub const DELETE:u32 = keysym::XK_Delete;

	pub const SCROLL_LOCK:u32 = keysym::XK_Scroll_Lock;

	pub const HELP:u32 = keysym::XK_Help;

	pub const NUMLOCK:u32 = keysym::XK_Num_Lock;
	// Media
	pub const VOLUME_MUTE:u32 = keysym::XF86XK_AudioMute;

	pub const VOLUME_DOWN:u32 = keysym::XF86XK_AudioLowerVolume;

	pub const VOLUME_UP:u32 = keysym::XF86XK_AudioRaiseVolume;

	pub const MEDIA_NEXT:u32 = keysym::XF86XK_AudioNext;

	pub const MEDIA_PREV:u32 = keysym::XF86XK_AudioPrev;

	pub const MEDIA_STOP:u32 = keysym::XF86XK_AudioStop;

	pub const MEDIA_PLAY_PAUSE:u32 = keysym::XF86XK_AudioPlay;

	pub const LAUNCH_MAIL:u32 = keysym::XF86XK_Mail;
	// F1-F12
	pub const F1:u32 = keysym::XK_F1;

	pub const F2:u32 = keysym::XK_F2;

	pub const F3:u32 = keysym::XK_F3;

	pub const F4:u32 = keysym::XK_F4;

	pub const F5:u32 = keysym::XK_F5;

	pub const F6:u32 = keysym::XK_F6;

	pub const F7:u32 = keysym::XK_F7;

	pub const F8:u32 = keysym::XK_F8;

	pub const F9:u32 = keysym::XK_F9;

	pub const F10:u32 = keysym::XK_F10;

	pub const F11:u32 = keysym::XK_F11;

	pub const F12:u32 = keysym::XK_F12;
	// Numpad
	pub const ADD:u32 = keysym::XK_KP_Add;

	pub const SUBTRACT:u32 = keysym::XK_KP_Subtract;

	pub const MULTIPLY:u32 = keysym::XK_KP_Multiply;

	pub const DIVIDE:u32 = keysym::XK_KP_Divide;

	pub const DECIMAL:u32 = keysym::XK_KP_Decimal;

	pub const NUMPAD0:u32 = keysym::XK_KP_0;

	pub const NUMPAD1:u32 = keysym::XK_KP_1;

	pub const NUMPAD2:u32 = keysym::XK_KP_2;

	pub const NUMPAD3:u32 = keysym::XK_KP_3;

	pub const NUMPAD4:u32 = keysym::XK_KP_4;

	pub const NUMPAD5:u32 = keysym::XK_KP_5;

	pub const NUMPAD6:u32 = keysym::XK_KP_6;

	pub const NUMPAD7:u32 = keysym::XK_KP_7;

	pub const NUMPAD8:u32 = keysym::XK_KP_8;

	pub const NUMPAD9:u32 = keysym::XK_KP_9;

	pub const KEY_0:u32 = '0' as u32;

	pub const KEY_1:u32 = '1' as u32;

	pub const KEY_2:u32 = '2' as u32;

	pub const KEY_3:u32 = '3' as u32;

	pub const KEY_4:u32 = '4' as u32;

	pub const KEY_5:u32 = '5' as u32;

	pub const KEY_6:u32 = '6' as u32;

	pub const KEY_7:u32 = '7' as u32;

	pub const KEY_8:u32 = '8' as u32;

	pub const KEY_9:u32 = '9' as u32;

	pub const A:u32 = 'A' as u32;

	pub const B:u32 = 'B' as u32;

	pub const C:u32 = 'C' as u32;

	pub const D:u32 = 'D' as u32;

	pub const E:u32 = 'E' as u32;

	pub const F:u32 = 'F' as u32;

	pub const G:u32 = 'G' as u32;

	pub const H:u32 = 'H' as u32;

	pub const I:u32 = 'I' as u32;

	pub const J:u32 = 'J' as u32;

	pub const K:u32 = 'K' as u32;

	pub const L:u32 = 'L' as u32;

	pub const M:u32 = 'M' as u32;

	pub const N:u32 = 'N' as u32;

	pub const O:u32 = 'O' as u32;

	pub const P:u32 = 'P' as u32;

	pub const Q:u32 = 'Q' as u32;

	pub const R:u32 = 'R' as u32;

	pub const S:u32 = 'S' as u32;

	pub const T:u32 = 'T' as u32;

	pub const U:u32 = 'U' as u32;

	pub const V:u32 = 'V' as u32;

	pub const W:u32 = 'W' as u32;

	pub const X:u32 = 'X' as u32;

	pub const Y:u32 = 'Y' as u32;

	pub const Z:u32 = 'Z' as u32;

	pub const EQUAL:u32 = keysym::XK_equal;

	pub const MINUS:u32 = keysym::XK_minus;

	pub const SINGLE_QUOTE:u32 = keysym::XK_leftsinglequotemark;

	pub const COMMA:u32 = keysym::XK_comma;

	pub const PERIOD:u32 = keysym::XK_period;

	pub const SEMICOLON:u32 = keysym::XK_semicolon;

	pub const SLASH:u32 = keysym::XK_slash;

	pub const OPEN_QUOTE:u32 = keysym::XK_acute;

	pub const OPEN_BRACKET:u32 = keysym::XK_bracketleft;

	pub const BACK_SLASH:u32 = keysym::XK_backslash;

	pub const CLOSE_BRACKET:u32 = keysym::XK_bracketright;
}

enum HotkeyMessage {
	RegisterHotkey(ListenerId, u32, u32),
	RegisterHotkeyResult(Result<ListenerId, HotkeyError>),
	UnregisterHotkey(ListenerId),
	UnregisterHotkeyResult(Result<(), HotkeyError>),
	DropThread,
}

type ListenerId = (i32, u32);

pub struct Listener {
	handlers:ListenerMap,
	sender:Sender<HotkeyMessage>,
	receiver:Receiver<HotkeyMessage>,
}

type ListenerMap = Arc<Mutex<HashMap<ListenerId, (ListenerHotkey, Box<ListenerCallback>)>>>;

impl HotkeyListener for Listener {
	fn new() -> Listener {
		let hotkeys = ListenerMap::default();

		let hotkey_map = hotkeys.clone();

		let (method_sender, thread_receiver) = mpsc::channel();

		let (thread_sender, method_receiver) = mpsc::channel();

		std::thread::spawn(move || {
			let xlib = xlib::Xlib::open().unwrap();

			unsafe {
				let display = (xlib.XOpenDisplay)(ptr::null());

				let root = (xlib.XDefaultRootWindow)(display);

				// Only trigger key release at end of repeated keys
				let mut supported_rtrn:i32 = mem::MaybeUninit::uninit().assume_init();
				(xlib.XkbSetDetectableAutoRepeat)(display, 1, &mut supported_rtrn);

				(xlib.XSelectInput)(display, root, xlib::KeyReleaseMask);

				let mut event:xlib::XEvent = mem::MaybeUninit::uninit().assume_init();

				loop {
					if (xlib.XPending)(display) > 0 {
						(xlib.XNextEvent)(display, &mut event);

						if let xlib::KeyRelease = event.get_type() {
							if let Some((_, handler)) = hotkey_map
								.lock()
								.unwrap()
								.get_mut(&(event.key.keycode as i32, event.key.state))
							{
								handler();
							}
						}
					}

					match thread_receiver.try_recv() {
						Ok(HotkeyMessage::RegisterHotkey(_, modifiers, key)) => {
							let keycode = (xlib.XKeysymToKeycode)(display, key.into()) as i32;

							let result = (xlib.XGrabKey)(
								display,
								keycode,
								modifiers,
								root,
								0,
								xlib::GrabModeAsync,
								xlib::GrabModeAsync,
							);

							if result == 0 {
								if let Err(err) =
									thread_sender.send(HotkeyMessage::RegisterHotkeyResult(Err(
										HotkeyError::BackendApiError(0),
									))) {
									eprintln!("hotkey: thread_sender.send error {}", err);
								}
							} else if let Err(err) = thread_sender
								.send(HotkeyMessage::RegisterHotkeyResult(Ok((keycode, modifiers))))
							{
								eprintln!("hotkey: thread_sender.send error {}", err);
							}
						},
						Ok(HotkeyMessage::UnregisterHotkey(id)) => {
							let result = (xlib.XUngrabKey)(display, id.0, id.1, root);

							if result == 0 {
								if let Err(err) =
									thread_sender.send(HotkeyMessage::UnregisterHotkeyResult(Err(
										HotkeyError::BackendApiError(0),
									))) {
									eprintln!("hotkey: thread_sender.send error {}", err);
								}
							} else if let Err(err) =
								thread_sender.send(HotkeyMessage::UnregisterHotkeyResult(Ok(())))
							{
								eprintln!("hotkey: thread_sender.send error {}", err);
							}
						},
						Ok(HotkeyMessage::DropThread) => {
							(xlib.XCloseDisplay)(display);

							return;
						},
						Err(err) => {
							if let std::sync::mpsc::TryRecvError::Disconnected = err {
								eprintln!("hotkey: try_recv error {}", err);
							}
						},
						_ => unreachable!("other message should not arrive"),
					};

					std::thread::sleep(std::time::Duration::from_millis(50));
				}
			}
		});

		Listener { handlers:hotkeys, sender:method_sender, receiver:method_receiver }
	}

	fn register_hotkey<F>(&mut self, hotkey:ListenerHotkey, handler:F) -> Result<(), HotkeyError>
	where
		F: 'static + FnMut() + Send, {
		for (key, _) in self.handlers.lock().unwrap().values() {
			if *key == hotkey {
				return Err(HotkeyError::HotkeyAlreadyRegistered(hotkey));
			}
		}

		self.sender
			.send(HotkeyMessage::RegisterHotkey((0, 0), hotkey.modifiers, hotkey.key))
			.map_err(|_| HotkeyError::ChannelError())?;

		match self.receiver.recv() {
			Ok(HotkeyMessage::RegisterHotkeyResult(Ok(id))) => {
				self.handlers.lock().unwrap().insert(id, (hotkey, Box::new(handler)));

				Ok(())
			},
			Ok(HotkeyMessage::RegisterHotkeyResult(Err(err))) => Err(err),
			Err(_) => Err(HotkeyError::ChannelError()),
			_ => Err(HotkeyError::Unknown),
		}
	}

	fn unregister_hotkey(&mut self, hotkey:ListenerHotkey) -> Result<(), HotkeyError> {
		let mut found_id = (-1, 0);

		for (id, (key, _)) in self.handlers.lock().unwrap().iter() {
			if *key == hotkey {
				found_id = *id;

				break;
			}
		}

		if found_id == (-1, 0) {
			return Err(HotkeyError::HotkeyNotRegistered(hotkey));
		}

		self.sender
			.send(HotkeyMessage::UnregisterHotkey(found_id))
			.map_err(|_| HotkeyError::ChannelError())?;

		if self.handlers.lock().unwrap().remove(&found_id).is_none() {
			panic!("hotkey should never be none")
		};

		match self.receiver.recv() {
			Ok(HotkeyMessage::UnregisterHotkeyResult(Ok(_))) => Ok(()),
			Ok(HotkeyMessage::UnregisterHotkeyResult(Err(err))) => Err(err),
			Err(_) => Err(HotkeyError::ChannelError()),
			_ => Err(HotkeyError::Unknown),
		}
	}

	fn registered_hotkeys(&self) -> Vec<ListenerHotkey> {
		let mut result = Vec::new();

		for v in self.handlers.lock().unwrap().values() {
			result.push(v.0);
		}

		result
	}
}

impl Drop for Listener {
	fn drop(&mut self) {
		if let Err(err) = self.sender.send(HotkeyMessage::DropThread) {
			eprintln!("cant send close thread message {}", err);
		}
	}
}
