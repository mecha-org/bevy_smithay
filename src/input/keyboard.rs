use crate::state::SmithayRunnerState;
use crate::{AppSendEvent, smithay_windows::SmithayWindows};
use bevy::input::keyboard::Key;
use bevy::input::{ButtonState, keyboard::KeyboardInput};
use bevy::log::warn;
use bevy::prelude::Entity;
use smithay_client_toolkit::{
    delegate_keyboard,
    reexports::client::Proxy,
    seat::keyboard::{KeyEvent, KeyboardHandler, Keysym},
};

/// Converts a Smithay keyboard event to a Bevy keyboard input event.
fn convert_keyboard_event(
    event: KeyEvent,
    entity: Entity,
    state: ButtonState,
) -> bevy::input::keyboard::KeyboardInput {
    KeyboardInput {
        state,
        window: entity,
        key_code: convert_to_key_code(event.keysym),
        logical_key: convert_to_logical_key(event.keysym),
        repeat: false,
    }
}

/// Converts a Smithay Keysym to a Bevy Key.
fn convert_to_logical_key(keysym: Keysym) -> bevy::input::keyboard::Key {
    // First, attempt to get the character representation based on keyboard layout.
    if let Some(c) = keysym.key_char() {
        // Use Key::Character for printable chars. Convert to SmolStr.
        return Key::Character((c.to_string()).into());
    }

    // If key_char() returned None, it's a non-printable key (Control, Function, Arrow etc.)
    // Map the Keysym variant to the corresponding Bevy Key variant.
    match keysym {
        // Function Keys
        Keysym::F1 => Key::F1,
        Keysym::F2 => Key::F2,
        Keysym::F3 => Key::F3,
        Keysym::F4 => Key::F4,
        Keysym::F5 => Key::F5,
        Keysym::F6 => Key::F6,
        Keysym::F7 => Key::F7,
        Keysym::F8 => Key::F8,
        Keysym::F9 => Key::F9,
        Keysym::F10 => Key::F10,
        Keysym::F11 => Key::F11,
        Keysym::F12 => Key::F12,
        Keysym::F13 => Key::F13,
        Keysym::F14 => Key::F14,
        Keysym::F15 => Key::F15,
        Keysym::F16 => Key::F16,
        Keysym::F17 => Key::F17,
        Keysym::F18 => Key::F18,
        Keysym::F19 => Key::F19,
        Keysym::F20 => Key::F20,
        Keysym::F21 => Key::F21,
        Keysym::F22 => Key::F22,
        Keysym::F23 => Key::F23,
        Keysym::F24 => Key::F24,

        // Control & Navigation
        Keysym::Escape => Key::Escape,
        Keysym::Pause => Key::Pause,
        Keysym::Scroll_Lock => Key::ScrollLock,
        Keysym::Print => Key::PrintScreen,
        Keysym::Insert => Key::Insert,
        Keysym::Delete => Key::Delete,
        Keysym::Home => Key::Home,
        Keysym::End => Key::End,
        Keysym::Page_Up => Key::PageUp,
        Keysym::Page_Down => Key::PageDown,
        Keysym::BackSpace => Key::Backspace,
        Keysym::Return | Keysym::KP_Enter => Key::Enter, // Map both main and numpad Enter
        Keysym::Tab => Key::Tab,
        Keysym::Menu => Key::ContextMenu,

        // Arrow Keys
        Keysym::Left => Key::ArrowLeft,
        Keysym::Right => Key::ArrowRight,
        Keysym::Up => Key::ArrowUp,
        Keysym::Down => Key::ArrowDown,

        // Modifiers
        Keysym::Shift_L => Key::Shift,
        Keysym::Shift_R => Key::Shift,
        Keysym::Control_L => Key::Control,
        Keysym::Control_R => Key::Control,
        Keysym::Alt_L => Key::Alt,
        Keysym::Alt_R => Key::Alt,
        Keysym::Super_L | Keysym::Meta_L => Key::Super, // Combine Super/Meta
        Keysym::Super_R | Keysym::Meta_R => Key::Super,
        Keysym::Caps_Lock => Key::CapsLock,
        Keysym::Num_Lock => Key::NumLock,

        // Keys that *might* have produced a character but key_char() returned None.
        // This could happen for dead keys, IME input, or if key_char() implementation is limited.
        // We need a fallback. Panicking with todo!() highlights these cases during development.
        // A production system might need more robust handling (e.g., logging, ignoring, specific Key variant if available).
        Keysym::A
        | Keysym::B
        | Keysym::C
        | Keysym::D
        | Keysym::E
        | Keysym::F
        | Keysym::G
        | Keysym::H
        | Keysym::I
        | Keysym::J
        | Keysym::K
        | Keysym::L
        | Keysym::M
        | Keysym::N
        | Keysym::O
        | Keysym::P
        | Keysym::Q
        | Keysym::R
        | Keysym::S
        | Keysym::T
        | Keysym::U
        | Keysym::V
        | Keysym::W
        | Keysym::X
        | Keysym::Y
        | Keysym::Z
        | Keysym::_0
        | Keysym::_1
        | Keysym::_2
        | Keysym::_3
        | Keysym::_4
        | Keysym::_5
        | Keysym::_6
        | Keysym::_7
        | Keysym::_8
        | Keysym::_9
        | Keysym::grave
        | Keysym::minus
        | Keysym::equal
        | Keysym::bracketleft
        | Keysym::bracketright
        | Keysym::backslash
        | Keysym::semicolon
        | Keysym::apostrophe
        | Keysym::comma
        | Keysym::period
        | Keysym::slash
        | Keysym::space => {
            warn!(
                "Keysym {:?} did not produce a character via key_char(). This might be unexpected.",
                keysym
            );
            // Bevy's Key enum doesn't have a generic 'Unidentified' or 'Unknown' variant.
            // Panicking via todo!() helps catch these during development.
            // You might need custom logic or decide to ignore these cases in production.
            todo!(
                "Unhandled case: Keysym {:?} did not yield a char via key_char(). How to map to bevy::input::keyboard::Key?",
                keysym
            )
        }

        // Catch-all for any other Keysym variants not handled above.
        _ => {
            todo!("Unhandled keysym variant: {:?}", keysym)
        }
    }
}

/// Converts a Smithay Keysym to a Bevy KeyCode.
fn convert_to_key_code(keysym: Keysym) -> bevy::prelude::KeyCode {
    match keysym {
        // Letters
        Keysym::A => bevy::prelude::KeyCode::KeyA,
        Keysym::B => bevy::prelude::KeyCode::KeyB,
        Keysym::C => bevy::prelude::KeyCode::KeyC,
        Keysym::D => bevy::prelude::KeyCode::KeyD,
        Keysym::E => bevy::prelude::KeyCode::KeyE,
        Keysym::F => bevy::prelude::KeyCode::KeyF,
        Keysym::G => bevy::prelude::KeyCode::KeyG,
        Keysym::H => bevy::prelude::KeyCode::KeyH,
        Keysym::I => bevy::prelude::KeyCode::KeyI,
        Keysym::J => bevy::prelude::KeyCode::KeyJ,
        Keysym::K => bevy::prelude::KeyCode::KeyK,
        Keysym::L => bevy::prelude::KeyCode::KeyL,
        Keysym::M => bevy::prelude::KeyCode::KeyM,
        Keysym::N => bevy::prelude::KeyCode::KeyN,
        Keysym::O => bevy::prelude::KeyCode::KeyO,
        Keysym::P => bevy::prelude::KeyCode::KeyP,
        Keysym::Q => bevy::prelude::KeyCode::KeyQ,
        Keysym::R => bevy::prelude::KeyCode::KeyR,
        Keysym::S => bevy::prelude::KeyCode::KeyS,
        Keysym::T => bevy::prelude::KeyCode::KeyT,
        Keysym::U => bevy::prelude::KeyCode::KeyU,
        Keysym::V => bevy::prelude::KeyCode::KeyV,
        Keysym::W => bevy::prelude::KeyCode::KeyW,
        Keysym::X => bevy::prelude::KeyCode::KeyX,
        Keysym::Y => bevy::prelude::KeyCode::KeyY,
        Keysym::Z => bevy::prelude::KeyCode::KeyZ,

        Keysym::a => bevy::prelude::KeyCode::KeyA,
        Keysym::b => bevy::prelude::KeyCode::KeyB,
        Keysym::c => bevy::prelude::KeyCode::KeyC,
        Keysym::d => bevy::prelude::KeyCode::KeyD,
        Keysym::e => bevy::prelude::KeyCode::KeyE,
        Keysym::f => bevy::prelude::KeyCode::KeyF,
        Keysym::g => bevy::prelude::KeyCode::KeyG,
        Keysym::h => bevy::prelude::KeyCode::KeyH,
        Keysym::i => bevy::prelude::KeyCode::KeyI,
        Keysym::j => bevy::prelude::KeyCode::KeyJ,
        Keysym::k => bevy::prelude::KeyCode::KeyK,
        Keysym::l => bevy::prelude::KeyCode::KeyL,
        Keysym::m => bevy::prelude::KeyCode::KeyM,
        Keysym::n => bevy::prelude::KeyCode::KeyN,
        Keysym::o => bevy::prelude::KeyCode::KeyO,
        Keysym::p => bevy::prelude::KeyCode::KeyP,
        Keysym::q => bevy::prelude::KeyCode::KeyQ,
        Keysym::r => bevy::prelude::KeyCode::KeyR,
        Keysym::s => bevy::prelude::KeyCode::KeyS,
        Keysym::t => bevy::prelude::KeyCode::KeyT,
        Keysym::u => bevy::prelude::KeyCode::KeyU,
        Keysym::v => bevy::prelude::KeyCode::KeyV,
        Keysym::w => bevy::prelude::KeyCode::KeyW,
        Keysym::x => bevy::prelude::KeyCode::KeyX,
        Keysym::y => bevy::prelude::KeyCode::KeyY,
        Keysym::z => bevy::prelude::KeyCode::KeyZ,

        // Numbers Row (might be Keysym::0, Keysym::1 etc. in some libs)
        Keysym::_0 => bevy::prelude::KeyCode::Digit0,
        Keysym::_1 => bevy::prelude::KeyCode::Digit1,
        Keysym::_2 => bevy::prelude::KeyCode::Digit2,
        Keysym::_3 => bevy::prelude::KeyCode::Digit3,
        Keysym::_4 => bevy::prelude::KeyCode::Digit4,
        Keysym::_5 => bevy::prelude::KeyCode::Digit5,
        Keysym::_6 => bevy::prelude::KeyCode::Digit6,
        Keysym::_7 => bevy::prelude::KeyCode::Digit7,
        Keysym::_8 => bevy::prelude::KeyCode::Digit8,
        Keysym::_9 => bevy::prelude::KeyCode::Digit9,

        // Function Keys
        Keysym::F1 => bevy::prelude::KeyCode::F1,
        Keysym::F2 => bevy::prelude::KeyCode::F2,
        Keysym::F3 => bevy::prelude::KeyCode::F3,
        Keysym::F4 => bevy::prelude::KeyCode::F4,
        Keysym::F5 => bevy::prelude::KeyCode::F5,
        Keysym::F6 => bevy::prelude::KeyCode::F6,
        Keysym::F7 => bevy::prelude::KeyCode::F7,
        Keysym::F8 => bevy::prelude::KeyCode::F8,
        Keysym::F9 => bevy::prelude::KeyCode::F9,
        Keysym::F10 => bevy::prelude::KeyCode::F10,
        Keysym::F11 => bevy::prelude::KeyCode::F11,
        Keysym::F12 => bevy::prelude::KeyCode::F12,
        Keysym::F13 => bevy::prelude::KeyCode::F13,
        Keysym::F14 => bevy::prelude::KeyCode::F14,
        Keysym::F15 => bevy::prelude::KeyCode::F15,
        Keysym::F16 => bevy::prelude::KeyCode::F16,
        Keysym::F17 => bevy::prelude::KeyCode::F17,
        Keysym::F18 => bevy::prelude::KeyCode::F18,
        Keysym::F19 => bevy::prelude::KeyCode::F19,
        Keysym::F20 => bevy::prelude::KeyCode::F20,
        Keysym::F21 => bevy::prelude::KeyCode::F21,
        Keysym::F22 => bevy::prelude::KeyCode::F22,
        Keysym::F23 => bevy::prelude::KeyCode::F23,
        Keysym::F24 => bevy::prelude::KeyCode::F24,

        // Arrow Keys
        Keysym::Left => bevy::prelude::KeyCode::ArrowLeft,
        Keysym::Right => bevy::prelude::KeyCode::ArrowRight,
        Keysym::Up => bevy::prelude::KeyCode::ArrowUp,
        Keysym::Down => bevy::prelude::KeyCode::ArrowDown,

        // Modifiers
        Keysym::Shift_L => bevy::prelude::KeyCode::ShiftLeft,
        Keysym::Shift_R => bevy::prelude::KeyCode::ShiftRight,
        Keysym::Control_L => bevy::prelude::KeyCode::ControlLeft,
        Keysym::Control_R => bevy::prelude::KeyCode::ControlRight,
        Keysym::Alt_L => bevy::prelude::KeyCode::AltLeft,
        Keysym::Alt_R => bevy::prelude::KeyCode::AltRight,
        Keysym::Super_L | Keysym::Meta_L => bevy::prelude::KeyCode::SuperLeft, // Handle Super/Meta variations
        Keysym::Super_R | Keysym::Meta_R => bevy::prelude::KeyCode::SuperRight,
        Keysym::Caps_Lock => bevy::prelude::KeyCode::CapsLock,
        Keysym::Num_Lock => bevy::prelude::KeyCode::NumLock,
        Keysym::Scroll_Lock => bevy::prelude::KeyCode::ScrollLock,

        // Navigation & Editing
        Keysym::Home => bevy::prelude::KeyCode::Home,
        Keysym::End => bevy::prelude::KeyCode::End,
        Keysym::Page_Up => bevy::prelude::KeyCode::PageUp,
        Keysym::Page_Down => bevy::prelude::KeyCode::PageDown,
        Keysym::Insert => bevy::prelude::KeyCode::Insert,
        Keysym::Delete => bevy::prelude::KeyCode::Delete,
        Keysym::BackSpace => bevy::prelude::KeyCode::Backspace, // Note: Bevy uses Backspace

        // Whitespace & Control (Keysym::Return for main Enter)
        Keysym::space => bevy::prelude::KeyCode::Space,
        Keysym::Tab => bevy::prelude::KeyCode::Tab,
        Keysym::Return | Keysym::KP_Enter => bevy::prelude::KeyCode::Enter, // Map both Enter and Numpad Enter
        Keysym::Escape => bevy::prelude::KeyCode::Escape,

        // Numpad Keys (might start with KP_ in some libs)
        Keysym::KP_0 => bevy::prelude::KeyCode::Numpad0,
        Keysym::KP_1 => bevy::prelude::KeyCode::Numpad1,
        Keysym::KP_2 => bevy::prelude::KeyCode::Numpad2,
        Keysym::KP_3 => bevy::prelude::KeyCode::Numpad3,
        Keysym::KP_4 => bevy::prelude::KeyCode::Numpad4,
        Keysym::KP_5 => bevy::prelude::KeyCode::Numpad5,
        Keysym::KP_6 => bevy::prelude::KeyCode::Numpad6,
        Keysym::KP_7 => bevy::prelude::KeyCode::Numpad7,
        Keysym::KP_8 => bevy::prelude::KeyCode::Numpad8,
        Keysym::KP_9 => bevy::prelude::KeyCode::Numpad9,
        Keysym::KP_Add => bevy::prelude::KeyCode::NumpadAdd,
        Keysym::KP_Subtract => bevy::prelude::KeyCode::NumpadSubtract,
        Keysym::KP_Multiply => bevy::prelude::KeyCode::NumpadMultiply,
        Keysym::KP_Divide => bevy::prelude::KeyCode::NumpadDivide,
        Keysym::KP_Decimal => bevy::prelude::KeyCode::NumpadDecimal,
        Keysym::KP_Separator => bevy::prelude::KeyCode::NumpadComma, // Bevy uses NumpadComma for the separator
        Keysym::KP_Equal => bevy::prelude::KeyCode::NumpadEqual,
        // KP_Enter handled above with Return

        // Punctuation & Symbols (Names can vary significantly)
        Keysym::grave => bevy::prelude::KeyCode::Backquote, // `
        Keysym::minus => bevy::prelude::KeyCode::Minus,     // -
        Keysym::equal => bevy::prelude::KeyCode::Equal,     // =
        Keysym::bracketleft => bevy::prelude::KeyCode::BracketLeft, // [
        Keysym::bracketright => bevy::prelude::KeyCode::BracketRight, // ]
        Keysym::backslash => bevy::prelude::KeyCode::Backslash, // \
        Keysym::semicolon => bevy::prelude::KeyCode::Semicolon, // ;
        Keysym::apostrophe => bevy::prelude::KeyCode::Quote, // ' (Bevy uses Quote)
        Keysym::comma => bevy::prelude::KeyCode::Comma,     // ,
        Keysym::period => bevy::prelude::KeyCode::Period,   // .
        Keysym::slash => bevy::prelude::KeyCode::Slash,     // /

        // Other Keys
        Keysym::Print => bevy::prelude::KeyCode::PrintScreen,
        Keysym::Pause => bevy::prelude::KeyCode::Pause,
        Keysym::Menu => bevy::prelude::KeyCode::ContextMenu, // Bevy uses ContextMenu

        // --- Fallback ---
        // This arm catches any Keysym not explicitly handled above.
        // Using todo!() will cause a panic, alerting you during development
        // that a key mapping is missing. For production, consider returning Option<KeyCode>
        // or logging an error instead of panicking.
        _ => {
            // e.g., eprintln!("Warning: Unhandled keysym: {:?}", keysym);
            // If the function returned Option<KeyCode>, you'd return None here.
            // Since it must return KeyCode, panic is the explicit way to fail.
            todo!("Unhandled keysym: {:?}", keysym)
            // Or potentially return a "default" like Escape, but this hides errors:
            // KeyCode::Escape
        }
    }
}

impl KeyboardHandler for SmithayRunnerState {
    /// Called when the keyboard focus enters a surface.
    fn enter(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        wl_surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _: &[smithay_client_toolkit::seat::keyboard::Keysym],
    ) {
        // Hold the active keyboard surface when focus enters.
        self.active_keyboard_surface = Some(wl_surface.clone());
    }

    /// Called when the keyboard focus leaves a surface.
    fn leave(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        wl_surface: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _: u32,
    ) {
        // Release the active keyboard surface when focus leaves.
        if let Some(active_surface) = &mut self.active_keyboard_surface {
            if active_surface.id() == wl_surface.id() {
                self.active_keyboard_surface.take();
            }
        }
    }

    /// Called when a key is pressed.
    fn press_key(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        if let Some(active_surface) = &self.active_keyboard_surface {
            let smithay_windows = self.world().non_send_resource::<SmithayWindows>();
            let entity = smithay_windows.smithay_to_entity.get(&active_surface.id());

            // TODO: Destroy surface when window is despawned.
            if entity.is_none() {
                return;
            }
            let entity = *entity.unwrap();

            let bevy_event = convert_keyboard_event(event, entity, ButtonState::Pressed);
            self.bevy_window_events.send(bevy_event);
        } else {
            warn!("there is no active window to send keyboard events!");
        }
    }

    /// Called when a key is released.
    fn release_key(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        event: smithay_client_toolkit::seat::keyboard::KeyEvent,
    ) {
        if let Some(active_surface) = &self.active_keyboard_surface {
            let smithay_windows = self.world().non_send_resource::<SmithayWindows>();
            let entity = smithay_windows.smithay_to_entity.get(&active_surface.id());

            // TODO: Destroy surface when window is despawned.
            if entity.is_none() {
                return;
            }
            let entity = *entity.unwrap();

            let bevy_event = convert_keyboard_event(event, entity, ButtonState::Released);
            self.bevy_window_events.send(bevy_event);
        } else {
            panic!("There is no window available to send keyboard events!");
        }
    }

    /// Called when the keyboard modifiers are updated.
    fn update_modifiers(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: &smithay_client_toolkit::reexports::client::protocol::wl_keyboard::WlKeyboard,
        _: u32,
        _: smithay_client_toolkit::seat::keyboard::Modifiers,
        _: u32,
    ) {
    }
}

delegate_keyboard!(SmithayRunnerState);
