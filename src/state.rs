use std::time::Duration;

use bevy::{
    app::PluginsState,
    ecs::system::SystemState,
    prelude::*,
    window::{WindowEvent as BevyWindowEvent, WindowScaleFactorChanged},
};

use smithay_client_toolkit::{
    compositor::CompositorHandler,
    delegate_compositor, delegate_output, delegate_registry, delegate_seat,
    output::{OutputHandler, OutputState},
    reexports::{
        calloop::EventLoop,
        calloop_wayland_source::WaylandSource,
        client::{
            Connection, Proxy,
            globals::registry_queue_init,
            protocol::{wl_keyboard, wl_pointer, wl_surface::WlSurface, wl_touch},
        },
    },
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{Capability, SeatHandler, SeatState},
};

use crate::{CreateWindowParams, smithay_windows::SmithayWindows, system::create_windows};

pub fn smithay_runner(mut app: App) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let conn = Connection::connect_to_env().expect("failed to connect to wayland!");
    let mut event_loop =
        EventLoop::<SmithayRunnerState>::try_new().expect("failed to create event_loop");
    let (globals, event_queue) =
        registry_queue_init::<SmithayRunnerState>(&conn).expect("failed to init registry queue");

    let qh = event_queue.handle();
    let loop_handle = event_loop.handle();
    WaylandSource::new(conn.clone(), event_queue)
        .insert(loop_handle.clone())
        .expect("failed to insert wayland source to event loop");

    let mut smithay_runner_state = SmithayRunnerState {
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        output_state: OutputState::new(&globals, &qh),

        keyboard: None,
        pointer: None,
        touch: None,

        active_keyboard_surface: None,

        app,
        bevy_window_events: vec![],

        active_touches: Default::default(),
    };

    loop {
        let mut create_window = SystemState::<CreateWindowParams<Added<Window>>>::from_world(
            smithay_runner_state.world_mut(),
        );
        create_windows(
            &globals,
            &qh,
            conn.clone(),
            create_window.get_mut(smithay_runner_state.world_mut()),
        );
        create_window.apply(smithay_runner_state.world_mut());
        event_loop
            .dispatch(
                Duration::from_secs_f32(1.0 / 60.0),
                &mut smithay_runner_state,
            )
            .expect("an unexpected error occured");
        smithay_runner_state.run_app_update();
    }
}

pub struct SmithayRunnerState {
    // Wayland States
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,

    // Inputs
    pub(crate) keyboard: Option<wl_keyboard::WlKeyboard>,
    pub(crate) pointer: Option<wl_pointer::WlPointer>,
    pub(crate) touch: Option<wl_touch::WlTouch>,

    // Active Surfaces
    pub(crate) active_keyboard_surface: Option<WlSurface>,

    // Bevy
    app: App,
    pub(crate) bevy_window_events: Vec<BevyWindowEvent>,

    // Touch
    pub(crate) active_touches: std::collections::HashMap<i32, (Entity, Vec2)>,
}

impl SmithayRunnerState {
    pub fn world(&self) -> &World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }

    pub fn run_app_update(&mut self) {
        self.forward_bevy_events();

        if self.app.plugins_state() == PluginsState::Cleaned {
            self.app.update();
        }
    }

    fn forward_bevy_events(&mut self) {
        let buffered_events = self.bevy_window_events.drain(..).collect::<Vec<_>>();
        let world = self.world_mut();

        for winit_event in buffered_events.iter() {
            match winit_event.clone() {
                BevyWindowEvent::AppLifecycle(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::CursorEntered(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::CursorLeft(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::CursorMoved(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::FileDragAndDrop(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::Ime(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::RequestRedraw(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowBackendScaleFactorChanged(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowCloseRequested(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowCreated(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowDestroyed(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowFocused(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowMoved(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowOccluded(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowResized(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowScaleFactorChanged(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::WindowThemeChanged(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::MouseButtonInput(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::MouseMotion(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::MouseWheel(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::PinchGesture(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::RotationGesture(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::DoubleTapGesture(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::PanGesture(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::TouchInput(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::KeyboardInput(e) => {
                    world.send_event(e);
                }
                BevyWindowEvent::KeyboardFocusLost(e) => {
                    world.send_event(e);
                }
            }
        }

        if !buffered_events.is_empty() {
            world
                .resource_mut::<Events<BevyWindowEvent>>()
                .send_batch(buffered_events);
        }
    }
}

impl CompositorHandler for SmithayRunnerState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        new_factor: i32,
    ) {
        let smithay_windows = self.world().non_send_resource::<SmithayWindows>();
        let entity = *smithay_windows
            .smithay_to_entity
            .get(&surface.id())
            .expect("no window created for the surface!");
        self.bevy_window_events
            .push(BevyWindowEvent::WindowScaleFactorChanged(
                WindowScaleFactorChanged {
                    window: entity,
                    scale_factor: new_factor as f64,
                },
            ))
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _new_transform: smithay_client_toolkit::reexports::client::protocol::wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _time: u32,
    ) {
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _surface: &smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface,
        _output: &smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for SmithayRunnerState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn update_output(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput,
    ) {
    }
}

impl SeatHandler for SmithayRunnerState {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }

    fn new_seat(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
    }

    fn new_capability(
        &mut self,
        _: &Connection,
        qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        seat: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard && self.keyboard.is_none() {
            let keyboard = self.seat_state.get_keyboard(qh, &seat, None).unwrap();
            self.keyboard = Some(keyboard);
        }
        if capability == Capability::Pointer && self.pointer.is_none() {
            let pointer = self.seat_state.get_pointer(qh, &seat).unwrap();
            self.pointer = Some(pointer);
        }
        if capability == Capability::Touch && self.touch.is_none() {
            let touch = self.seat_state.get_touch(qh, &seat).unwrap();
            self.touch = Some(touch);

            info!("Touchscreen Attached");
        }
    }

    fn remove_capability(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
        capability: smithay_client_toolkit::seat::Capability,
    ) {
        if capability == Capability::Keyboard {
            if let Some(keyboard) = self.keyboard.take() {
                keyboard.release();
            }
        }
        if capability == Capability::Pointer {
            if let Some(pointer) = self.pointer.take() {
                pointer.release();
            }
        }
    }

    fn remove_seat(
        &mut self,
        _: &Connection,
        _: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _: smithay_client_toolkit::reexports::client::protocol::wl_seat::WlSeat,
    ) {
    }
}

impl ProvidesRegistryState for SmithayRunnerState {
    fn registry(&mut self) -> &mut smithay_client_toolkit::registry::RegistryState {
        &mut self.registry_state
    }

    registry_handlers!(OutputState, SeatState);
}

delegate_compositor!(SmithayRunnerState);
delegate_output!(SmithayRunnerState);
delegate_seat!(SmithayRunnerState);
delegate_registry!(SmithayRunnerState);
