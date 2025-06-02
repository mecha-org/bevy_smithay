use bevy::prelude::{Component, Resource};
use raw_window_handle::{
    DisplayHandle, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle,
    WaylandDisplayHandle, WaylandWindowHandle, WindowHandle,
};
use smithay_client_toolkit::{
    delegate_layer,
    globals::GlobalData,
    reexports::{
        client::{
            Connection, Dispatch, Proxy, QueueHandle, globals::GlobalList,
            protocol::wl_surface::WlSurface,
        },
        protocols_wlr::layer_shell::v1::client::{zwlr_layer_shell_v1, zwlr_layer_surface_v1},
    },
    shell::{
        WaylandSurface,
        wlr_layer::{LayerShell, LayerShellHandler, LayerSurface, LayerSurfaceData},
    },
};

pub use smithay_client_toolkit::shell::wlr_layer::{Anchor, KeyboardInteractivity, Layer};

use crate::state::SmithayRunnerState;

#[derive(Debug, Resource, Component, Clone)]
pub struct LayerShellSettings {
    /// Defines where the layer surface should be anchored to the screen.
    ///
    /// You can anchor the layer surface to any combination of the top, bottom, left, and right edges of the screen.
    pub anchor: Anchor,
    /// Defines the size of the layer surface in pixels.
    pub size: (u32, u32),
    /// Defines the amount of exclusive space the layer surface should reserve.
    ///
    /// Other surfaces will not be placed in this area. A negative value means that the layer surface
    /// will not reserve any exclusive space.
    pub exclusive_zone: i32,
    /// Defines the margins for the layer surface.
    ///
    /// Margins are specified in the order: top, right, bottom, left.
    pub margin: (i32, i32, i32, i32),
    /// Defines how the layer surface should handle keyboard interactivity.
    ///
    /// If set to `Exclusive`, the layer surface will receive all keyboard input.
    /// If set to `OnDemand`, the layer surface will only receive keyboard input when it is focused.
    /// If set to `None`, the layer surface will never receive keyboard input.
    pub keyboard_interactivity: KeyboardInteractivity,
    /// Defines the layer that the surface should be placed on.
    ///
    /// The layer determines the stacking order of the surface. Surfaces on higher layers are
    /// always drawn on top of surfaces on lower layers.
    pub layer: Layer,
}

impl Default for LayerShellSettings {
    fn default() -> Self {
        Self {
            anchor: Anchor::empty(),
            exclusive_zone: Default::default(),
            margin: Default::default(),
            size: (256, 256),
            keyboard_interactivity: Default::default(),
            layer: Layer::Top,
        }
    }
}

pub struct LayerShellWindow {
    window: Option<LayerSurface>,
    conn: Connection,
}

impl LayerShellWindow {
    pub fn layer_surface(&self) -> &LayerSurface {
        self.window
            .as_ref()
            .expect("trying to access layer surface after destroying")
    }

    pub fn destroy(&mut self) {}

    pub fn layer_surface_mut(&mut self) -> &mut LayerSurface {
        self.window
            .as_mut()
            .expect("trying to access layer surface after destroying")
    }
}

impl HasWindowHandle for LayerShellWindow {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let raw_window_handle = RawWindowHandle::Wayland(WaylandWindowHandle::new(
            core::ptr::NonNull::new(
                self.window
                    .as_ref()
                    .expect("window handles doesn't exist because surface was destroyed")
                    .wl_surface()
                    .id()
                    .as_ptr() as *mut _,
            )
            .unwrap(),
        ));
        unsafe { Ok(WindowHandle::borrow_raw(raw_window_handle)) }
    }
}

impl HasDisplayHandle for LayerShellWindow {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle, raw_window_handle::HandleError> {
        let raw_display_handle = RawDisplayHandle::Wayland(WaylandDisplayHandle::new(
            core::ptr::NonNull::new(self.conn.backend().display_ptr() as *mut _).unwrap(),
        ));
        unsafe { Ok(DisplayHandle::borrow_raw(raw_display_handle)) }
    }
}

// TODO: Destroy surfaces when the window is despawned.
// impl Drop for LayerShellWindow {
//     fn drop(&mut self) {
//         self.window
//             .as_ref()
//             .expect("destrying layer_surface twice")
//             .wl_surface()
//             .destroy();
//         drop(self.window.take());
//     }
// }

pub fn create_window<State>(
    globals: &GlobalList,
    qh: &QueueHandle<State>,
    surface: WlSurface,
    conn: Connection,
    settings: &LayerShellSettings,
) -> LayerShellWindow
where
    State: LayerShellHandler
        + Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, GlobalData>
        + Dispatch<zwlr_layer_surface_v1::ZwlrLayerSurfaceV1, LayerSurfaceData>
        + 'static,
{
    let layer_shell = LayerShell::bind(globals, qh).expect("layer shell not available");
    let layer =
        layer_shell.create_layer_surface(qh, surface, settings.layer, Some("simple_layer"), None);

    layer.set_anchor(settings.anchor);
    layer.set_keyboard_interactivity(settings.keyboard_interactivity);
    layer.set_size(settings.size.0, settings.size.1);
    layer.set_margin(
        settings.margin.0,
        settings.margin.1,
        settings.margin.2,
        settings.margin.3,
    );
    layer.set_exclusive_zone(settings.exclusive_zone);
    layer.commit();

    LayerShellWindow {
        window: Some(layer),
        conn,
    }
}

impl LayerShellHandler for SmithayRunnerState {
    fn closed(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &QueueHandle<Self>,
        _: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
    ) {
    }

    fn configure(
        &mut self,
        _: &smithay_client_toolkit::reexports::client::Connection,
        _: &QueueHandle<Self>,
        _: &smithay_client_toolkit::shell::wlr_layer::LayerSurface,
        _: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        _: u32,
    ) {
    }
}

delegate_layer!(SmithayRunnerState);
