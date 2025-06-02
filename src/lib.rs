use bevy::{
    prelude::*,
    window::{RawHandleWrapperHolder, WindowCreated},
};
use shells::layer_shell::LayerShellSettings;
use smithay_windows::SmithayWindows;

mod input;
mod shells;
mod smithay_windows;
mod state;
mod system;

pub mod prelude {
    pub use super::SmithayPlugin;
    pub use super::shells::layer_shell::*;
}

pub struct SmithayPlugin;
impl Plugin for SmithayPlugin {
    fn name(&self) -> &str {
        "bevy_smithay::SmithayPlugin"
    }

    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<SmithayWindows>()
            .add_systems(Last, (system::changed_windows, system::despawn_windows));
        app.set_runner(state::smithay_runner);
    }
}

trait AppSendEvent {
    fn send(&mut self, event: impl Into<bevy::window::WindowEvent>);
}

impl AppSendEvent for Vec<bevy::window::WindowEvent> {
    fn send(&mut self, event: impl Into<bevy::window::WindowEvent>) {
        self.push(Into::<bevy::window::WindowEvent>::into(event));
    }
}

pub type CreateWindowParams<'w, 's, F = ()> = (
    Commands<'w, 's>,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut Window,
            Option<&'static RawHandleWrapperHolder>,
        ),
        F,
    >,
    NonSendMut<'w, SmithayWindows>,
    Option<ResMut<'w, LayerShellSettings>>,
    EventWriter<'w, WindowCreated>,
);
