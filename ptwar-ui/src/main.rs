use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::MouseWheel;
use bevy::time::common_conditions::on_timer;
use bevy::window::WindowMode;
use bevy::{
    color::palettes::css,
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use hexx::storage::HexStore;
use hexx::*;
use ptwar::world::tile::Biome;
use ptwar::world::PtWorld;
use rand::prelude::IteratorRandom;
use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 2.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
/// Animation time step
const TIME_STEP: Duration = Duration::from_millis(100);

const MIN_ZOOM: f32 = 10.0;
const MAX_ZOOM: f32 = 100.0;

const VIEW_ANGLE: f32 = 45.0;

fn main() {
    App::new()
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PTwar".to_string(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AmbientLight {
            brightness: 200.0,
            ..default()
        })
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, move_camera)
        .add_systems(
            Update,
            show_info.run_if(on_timer(Duration::from_millis(100))),
        )
        .run();
}

#[derive(Debug, Resource)]
struct Map {
    entities: HashMap<Hex, Entity>,
    layout: HexLayout,
}

impl Map {
    pub fn center(&self) -> Vec2 {
        self.layout.hex_to_world_pos(Hex::ZERO)
    }
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    ring: u32,
    hexes: Vec<Hex>,
}

#[derive(Component)]
struct MyCameraMarker;

fn move_camera(
    map: Res<Map>,
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q: Query<(&Camera3d, &mut Transform), With<MyCameraMarker>>,
) {
    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        direction.z -= 1.;
    }

    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        direction.z += 1.;
    }

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.;
    }

    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        direction.x += 1.;
    }
    // Fix camera angle

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    for (camera, mut transform) in q.iter_mut() {
        // TODO: limit camera movement
        // Fix camera angle
        transform.translation = transform.translation.add(direction);
    }

    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Pixel => {
                if ev.y != 0.0 {
                    if let (_camera, mut transform) = q.single_mut() {
                        let norm = transform.translation.normalize();

                        // Divide by 3.0 to make the zoom slower
                        let mut new_pos = transform.translation + norm * (ev.y / 4.0);
                        // Doesn't allow zooming on X axis
                        new_pos.x = transform.translation.x;

                        if new_pos.y < MIN_ZOOM || new_pos.y > MAX_ZOOM {
                            return;
                        }

                        transform.translation = new_pos;
                    }
                }
            }
            _ => {}
        }
    }
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 90.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        MyCameraMarker,
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(90.0, 90.0, 00.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Text
    commands.spawn((
        InfoText,
        Text::new("DEBUG"),
        TextFont {
            font_size: 24.,
            ..Default::default()
        },
        TextColor(Color::LinearRgba(LinearRgba::WHITE)),
    ));
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rng = &mut rand::rng();

    let pt_world = PtWorld::from_seed(rng.random());

    let layout = HexLayout {
        scale: HEX_SIZE,
        ..default()
    };
    // materials
    let region = pt_world.regions.iter().choose(rng).unwrap().1;

    let entities = region
        .tiles
        .iter()
        .map(|(hex, tile)| {
            let pos = layout.hex_to_world_pos(*hex);
            let height = (tile.noise * 2.0) as f32;

            let mesh = hexagonal_column(&layout, 1.0 + height);
            let mesh_handle = meshes.add(mesh);

            let color = match tile.biome {
                Biome::Desert => css::YELLOW,
                Biome::Forest => css::DARK_GREEN,
                Biome::Water => css::BLUE,
                Biome::Plains => css::GREEN,
                Biome::DenseForest => css::DARK_GREEN,
                Biome::Hill => css::LIGHT_GRAY,
                Biome::Mountain => css::DARK_GRAY,
                Biome::City => css::PURPLE,
            };

            let default_material = materials.add(Color::Srgba(color));

            let id = commands
                .spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(default_material),
                    Transform::from_xyz(pos.x, 1.0, pos.y),
                ))
                .id();
            (*hex, id)
        })
        .collect();

    commands.insert_resource(Map { entities, layout });
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout, height: f32) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

#[derive(Component)]
pub struct InfoText;

pub fn show_info(
    state: Res<Map>,
    mut text_query: Query<&mut Text, With<InfoText>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut text_info = format!("Hexes: {}\n", state.entities.len());

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let add = format!("FPS: {}\n", format!("{value:.0}"));

            text_info.push_str(add.as_str());
        }
    }

    text_query.get_single_mut().unwrap().0 = text_info;
}
