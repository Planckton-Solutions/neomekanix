use bitflags::bitflags;
pub use crossbeam::channel::{Receiver, Sender};
use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub enum EventType {
    App(AppEvent),
    Window(WindowEvent),
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
    Close,
    Destroyed,
    Resize(u32, u32),
    Focus,
    Blur,
    Move(i32, i32),
    ScaleFactorChanged(f64),
    ThemeChanged(winit::window::Theme),
    Occluded(bool),
    RedrawRequested,
}

#[derive(Debug, Copy, Clone)]
pub enum AppEvent {
    Tick,
    Update,
    Render,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyboardEvent {
    Press(Key, Modifiers, u32),
    Release(Key, Modifiers),
}

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
    ButtonPress(MouseButton, Modifiers),
    ButtonRelease(MouseButton, Modifiers),
    Move(u32, u32),
    Scroll(f64, f64),
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Key {
	A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
	CtrlLeft,CtrlRight,ShiftLeft,ShiftRight,AltLeft,AltRight,OSLeft,OSRight,Menu,
	CapsLock,NumLock,ScrollLock,
	Digit0,Digit1,Digit2,Digit3,Digit4,Digit5,Digit6,Digit7,Digit8,Digit9,
	Num0,Num1,Num2,Num3,Num4,Num5,Num6,Num7,Num8,Num9,
	NumDecimal,NumDivide,NumMultiply,NumMinus,NumPlus,NumEnter,NumEqual,
	F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24,F25,
	PrintScreen,SysRq,Pause,
	Insert,Delete,Home,End,PageUp,PageDown,
	Up,Right,Down,Left,
	BracketLeft,BracketRight,Backslash,
	Semicolon,Apostrophe,Comma,Period,Slash,BackTick,Minus,Equal,
	Tab,Enter,Esc,Space,Backspace,
	World1,World2,
	Unknown,
}
bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Modifiers: u32 {
        const SHIFT = winit::keyboard::ModifiersState::SHIFT.bits();
        const CTRL = winit::keyboard::ModifiersState::CONTROL.bits();
        const ALT = winit::keyboard::ModifiersState::ALT.bits();
        const SUPER = winit::keyboard::ModifiersState::SUPER.bits();
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Primary,
    Secondary,
    Middle,
    Back,
    Forward,
    Button6,
    Button7,
    Button8,
    Button9,
    Button10,
    Button11,
    Button12,
    Button13,
    Button14,
    Button15,
    Button16,
    Button17,
    Button18,
    Button19,
    Button20,
}

#[rustfmt::skip]
impl From<winit::keyboard::KeyCode> for Key {
	fn from(key: winit::keyboard::KeyCode) -> Self {
		use Key::*;

		match key {
            winit::keyboard::KeyCode::Quote   => Apostrophe,
			winit::keyboard::KeyCode::Space        => Space,
			winit::keyboard::KeyCode::Comma        => Comma,
			winit::keyboard::KeyCode::Minus        => Minus,
			winit::keyboard::KeyCode::Period       => Period,
			winit::keyboard::KeyCode::Slash        => Slash,
			winit::keyboard::KeyCode::Numpad0         => Num0,
			winit::keyboard::KeyCode::Numpad1         => Num1,
			winit::keyboard::KeyCode::Numpad2         => Num2,
			winit::keyboard::KeyCode::Numpad3         => Num3,
			winit::keyboard::KeyCode::Numpad4         => Num4,
			winit::keyboard::KeyCode::Numpad5         => Num5,
			winit::keyboard::KeyCode::Numpad6         => Num6,
			winit::keyboard::KeyCode::Numpad7         => Num7,
			winit::keyboard::KeyCode::Numpad8         => Num8,
			winit::keyboard::KeyCode::Numpad9         => Num9,
			winit::keyboard::KeyCode::Semicolon    => Semicolon,
			winit::keyboard::KeyCode::Equal        => Equal,
			winit::keyboard::KeyCode::KeyA            => A,
			winit::keyboard::KeyCode::KeyB            => B,
			winit::keyboard::KeyCode::KeyC            => C,
			winit::keyboard::KeyCode::KeyD            => D,
			winit::keyboard::KeyCode::KeyE            => E,
			winit::keyboard::KeyCode::KeyF            => F,
			winit::keyboard::KeyCode::KeyG            => G,
			winit::keyboard::KeyCode::KeyH            => H,
			winit::keyboard::KeyCode::KeyI            => I,
			winit::keyboard::KeyCode::KeyJ            => J,
			winit::keyboard::KeyCode::KeyK            => K,
			winit::keyboard::KeyCode::KeyL            => L,
			winit::keyboard::KeyCode::KeyM            => M,
			winit::keyboard::KeyCode::KeyN            => N,
			winit::keyboard::KeyCode::KeyO            => O,
			winit::keyboard::KeyCode::KeyP            => P,
			winit::keyboard::KeyCode::KeyQ            => Q,
			winit::keyboard::KeyCode::KeyR            => R,
			winit::keyboard::KeyCode::KeyS            => S,
			winit::keyboard::KeyCode::KeyT            => T,
			winit::keyboard::KeyCode::KeyU            => U,
			winit::keyboard::KeyCode::KeyV            => V,
			winit::keyboard::KeyCode::KeyW            => W,
			winit::keyboard::KeyCode::KeyX            => X,
			winit::keyboard::KeyCode::KeyY            => Y,
			winit::keyboard::KeyCode::KeyZ            => Z,
			winit::keyboard::KeyCode::BracketLeft  => BracketLeft,
			winit::keyboard::KeyCode::Backslash    => Backslash,
			winit::keyboard::KeyCode::BracketRight => BracketRight,
			winit::keyboard::KeyCode::Backquote  => BackTick,
			winit::keyboard::KeyCode::IntlBackslash       => World1,
			winit::keyboard::KeyCode::IntlRo       => World2,
			winit::keyboard::KeyCode::Escape       => Esc,
			winit::keyboard::KeyCode::Enter        => Enter,
			winit::keyboard::KeyCode::Tab          => Tab,
			winit::keyboard::KeyCode::Backspace    => Backspace,
			winit::keyboard::KeyCode::Insert       => Insert,
			winit::keyboard::KeyCode::Delete       => Delete,
			winit::keyboard::KeyCode::ArrowRight        => Right,
			winit::keyboard::KeyCode::ArrowLeft         => Left,
			winit::keyboard::KeyCode::ArrowDown         => Down,
			winit::keyboard::KeyCode::ArrowUp           => Up,
			winit::keyboard::KeyCode::PageUp       => PageUp,
			winit::keyboard::KeyCode::PageDown     => PageDown,
			winit::keyboard::KeyCode::Home         => Home,
			winit::keyboard::KeyCode::End          => End,
			winit::keyboard::KeyCode::CapsLock     => CapsLock,
			winit::keyboard::KeyCode::ScrollLock   => ScrollLock,
			winit::keyboard::KeyCode::NumLock      => NumLock,
			winit::keyboard::KeyCode::PrintScreen  => PrintScreen,
			winit::keyboard::KeyCode::Pause        => Pause,
			winit::keyboard::KeyCode::F1           => F1,
			winit::keyboard::KeyCode::F2           => F2,
			winit::keyboard::KeyCode::F3           => F3,
			winit::keyboard::KeyCode::F4           => F4,
			winit::keyboard::KeyCode::F5           => F5,
			winit::keyboard::KeyCode::F6           => F6,
			winit::keyboard::KeyCode::F7           => F7,
			winit::keyboard::KeyCode::F8           => F8,
			winit::keyboard::KeyCode::F9           => F9,
			winit::keyboard::KeyCode::F10          => F10,
			winit::keyboard::KeyCode::F11          => F11,
			winit::keyboard::KeyCode::F12          => F12,
			winit::keyboard::KeyCode::F13          => F13,
			winit::keyboard::KeyCode::F14          => F14,
			winit::keyboard::KeyCode::F15          => F15,
			winit::keyboard::KeyCode::F16          => F16,
			winit::keyboard::KeyCode::F17          => F17,
			winit::keyboard::KeyCode::F18          => F18,
			winit::keyboard::KeyCode::F19          => F19,
			winit::keyboard::KeyCode::F20          => F20,
			winit::keyboard::KeyCode::F21          => F21,
			winit::keyboard::KeyCode::F22          => F22,
			winit::keyboard::KeyCode::F23          => F23,
			winit::keyboard::KeyCode::F24          => F24,
			winit::keyboard::KeyCode::F25          => F25,
			winit::keyboard::KeyCode::Digit0          => Digit0,
			winit::keyboard::KeyCode::Digit1          => Digit1,
			winit::keyboard::KeyCode::Digit2          => Digit2,
			winit::keyboard::KeyCode::Digit3          => Digit3,
			winit::keyboard::KeyCode::Digit4          => Digit4,
			winit::keyboard::KeyCode::Digit5          => Digit5,
			winit::keyboard::KeyCode::Digit6          => Digit6,
			winit::keyboard::KeyCode::Digit7          => Digit7,
			winit::keyboard::KeyCode::Digit8          => Digit8,
			winit::keyboard::KeyCode::Digit9          => Digit9,
			winit::keyboard::KeyCode::NumpadDecimal    => NumDecimal,
			winit::keyboard::KeyCode::NumpadDivide     => NumDivide,
			winit::keyboard::KeyCode::NumpadMultiply   => NumMultiply,
			winit::keyboard::KeyCode::NumpadSubtract   => NumMinus,
			winit::keyboard::KeyCode::NumpadAdd        => NumPlus,
			winit::keyboard::KeyCode::NumpadEnter      => NumEnter,
			winit::keyboard::KeyCode::NumpadEqual      => NumEqual,
			winit::keyboard::KeyCode::ShiftLeft    => ShiftLeft,
			winit::keyboard::KeyCode::ControlLeft  => CtrlLeft,
			winit::keyboard::KeyCode::AltLeft      => AltLeft,
			winit::keyboard::KeyCode::SuperLeft    => OSLeft,
			winit::keyboard::KeyCode::ShiftRight   => ShiftRight,
			winit::keyboard::KeyCode::ControlRight => CtrlRight,
			winit::keyboard::KeyCode::AltRight     => AltRight,
			winit::keyboard::KeyCode::SuperRight   => OSRight,
			winit::keyboard::KeyCode::ContextMenu         => Menu,
			_      => Unknown,
		}
	}
}

impl From<winit::event::Modifiers> for Modifiers {
    fn from(mods: winit::event::Modifiers) -> Self {
        Modifiers::from(mods.state())
    }
}

impl From<winit::keyboard::ModifiersState> for Modifiers {
    fn from(mods: winit::keyboard::ModifiersState) -> Self {
        Modifiers::from_bits_retain(mods.bits())
    }
}

impl TryFrom<winit::event::WindowEvent> for WindowEvent {
    type Error = winit::event::WindowEvent;

    fn try_from(event: winit::event::WindowEvent) -> Result<Self, Self::Error> {
        match event {
            winit::event::WindowEvent::CloseRequested => Ok(Self::Close),
            winit::event::WindowEvent::Destroyed => Ok(Self::Destroyed),
            winit::event::WindowEvent::Resized(size) => Ok(Self::Resize(size.width, size.height)),
            winit::event::WindowEvent::Focused(true) => Ok(Self::Focus),
            winit::event::WindowEvent::Focused(false) => Ok(Self::Blur),
            winit::event::WindowEvent::Moved(position) => Ok(Self::Move(position.x, position.y)),
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                Ok(Self::ScaleFactorChanged(scale_factor))
            }
            winit::event::WindowEvent::ThemeChanged(theme) => Ok(Self::ThemeChanged(theme)),
            winit::event::WindowEvent::Occluded(occluded) => Ok(Self::Occluded(occluded)),
            winit::event::WindowEvent::RedrawRequested => Ok(Self::RedrawRequested),
            event => Err(event),
        }
    }
}

impl From<winit::event::MouseButton> for MouseButton {
    fn from(button: winit::event::MouseButton) -> Self {
        use MouseButton::*;

        match button {
            winit::event::MouseButton::Left => Primary,
            winit::event::MouseButton::Right => Secondary,
            winit::event::MouseButton::Middle => Middle,
            winit::event::MouseButton::Back => Back,
            winit::event::MouseButton::Forward => Forward,
            winit::event::MouseButton::Other(_) => Button6,
        }
    }
}
