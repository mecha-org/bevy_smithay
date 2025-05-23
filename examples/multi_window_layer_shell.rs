#![allow(warnings)]

use bevy::{color::palettes::basic::*, prelude::*, window::WindowCreated, winit::WinitPlugin};
use bevy_smithay::{
    SmithayPlugin,
    prelude::{Anchor, Layer, LayerShellSettings},
};

#[derive(Resource, Default)]
struct NewWindowInfo {
    entity: Option<Entity>,
    is_setup_pending: bool,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.build().disable::<WinitPlugin>(),
            SmithayPlugin,
        ))
        .insert_resource(LayerShellSettings {
            anchor: Anchor::TOP, // Anchor to the bottom of the screen
            layer: Layer::Top,   // Set the layer to background
            keyboard_interactivity: bevy_smithay::prelude::KeyboardInteractivity::OnDemand, // Enable keyboard interactivity on demand
            exclusive_zone: 720, // Reserve 720 pixels at the bottom of the screen
            size: Default::default(), // Use default size
            margin: Default::default(), // Use default margin
        })
        .init_resource::<NewWindowInfo>()
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_new_window, setup_new_window))
        .add_systems(Update, (button_system, exit_on_esc))
        .run();
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn spawn_new_window(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut new_window_info: ResMut<NewWindowInfo>,
    mut layer_shell_settings: ResMut<LayerShellSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyN) {
        layer_shell_settings.size = (400, 250);
        layer_shell_settings.exclusive_zone = 0;
        if new_window_info.entity.is_none() {
            info!("Spawning new UI-only window...");
            let new_window_entity = commands
                .spawn(Window {
                    title: "UI Only Window".to_string(),
                    resolution: (400., 250.).into(),
                    ..default()
                })
                .id();
            new_window_info.entity = Some(new_window_entity);
            new_window_info.is_setup_pending = true;
        } else {
            commands.entity(new_window_info.entity.unwrap()).despawn();
            new_window_info.entity = None;
        }
    }
}

fn setup_new_window(
    mut commands: Commands,
    mut window_created_events: EventReader<WindowCreated>,
    mut new_window_info: ResMut<NewWindowInfo>,
    asset_server: Res<AssetServer>, // For fonts
) {
    for event in window_created_events.read() {
        if Some(event.window) == new_window_info.entity && new_window_info.is_setup_pending {
            info!(
                "New UI window created (ID: {:?}), setting up its camera and UI.",
                event.window
            );

            commands.spawn((Camera2dBundle {
                camera: Camera {
                    target: bevy::render::camera::RenderTarget::Window(
                        bevy::window::WindowRef::Entity(event.window),
                    ),
                    clear_color: ClearColorConfig::Custom(Color::default()),
                    ..default()
                },
                ..default()
            },));
            new_window_info.is_setup_pending = false; // Mark as setup complete
        }
    }
}

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ui camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Button"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}
