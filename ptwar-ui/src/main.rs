use bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::input::mouse::MouseWheel;
use bevy::time::common_conditions::on_timer;
use bevy::window::{PrimaryWindow, WindowMode};
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
const REGION_HEX_SIZE: Vec2 = Vec2::splat(30.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 2.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
/// Animation time step
const TIME_STEP: Duration = Duration::from_millis(100);

const MIN_ZOOM: f32 = 10.0;
const INITIAL_ZOOM: f32 = 200.0;
const MAX_ZOOM: f32 = 250.0;

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
        .insert_resource(CursorHex {
            hex: None,
            should_track: true,
        })
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, move_camera)
        .add_systems(Update, handle_input)
        .add_systems(Update, hover_hex_update)
        .add_systems(
            Update,
            show_info.run_if(on_timer(Duration::from_millis(100))),
        )
        .run();
}

#[derive(Component)]
struct HoverHex;

#[derive(Debug, Resource)]
struct CursorHex {
    hex: Option<Hex>,
    should_track: bool,
}

#[derive(Debug, PartialEq, Clone)]
enum Zoom {
    In,
    Out,
}

#[derive(Resource, Clone)]
struct ZoomLevel {
    level: f32,
}

impl ZoomLevel {
    pub fn new(level: f32) -> Self {
        Self { level }
    }
    pub fn zoom(&self) -> Zoom {
        if self.level > 100.0 {
            Zoom::Out
        } else {
            Zoom::In
        }
    }
}

#[derive(Resource)]
struct Map {
    all: Vec<Entity>,
    layout: HexLayout,
    pt_world: PtWorld,
    render_on: Zoom,
}

impl Map {
    pub fn center(&self) -> Vec2 {
        self.layout.hex_to_world_pos(Hex::ZERO)
    }

    pub fn despawn_recursive(&self, mut commands: &mut Commands) {
        for x in self.all.iter() {
            commands.get_entity(*x).map(|entity| {
                entity.despawn_recursive();
            });
        }
    }
}

#[derive(Component)]
struct MyCameraMarker;

fn handle_input(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<Map>,
    mut cursor_hex: ResMut<CursorHex>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(ray3d) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world(cam_transform, p).ok())
    {
        if cursor_hex.should_track == false {
            return;
        }

        // get point from ray3d that is on xz plane
        let point = ray3d.origin + ray3d.direction * (ray3d.origin.y / -ray3d.direction.y);
        let hex = grid.layout.world_pos_to_hex(Vec2::new(point.x, point.z));

        if grid.pt_world.regions.contains_key(&hex) == false {
            cursor_hex.hex = None;
            return;
        }

        match cursor_hex.hex {
            Some(hex) => {
                if hex != hex {
                    cursor_hex.hex = Some(hex);
                }
            }
            None => {
                cursor_hex.hex = Some(hex);
            }
        }
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q: Query<(&Camera3d, &mut Transform), With<MyCameraMarker>>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut direction = Vec3::ZERO;

    if keys.just_pressed(KeyCode::KeyR) {
        commands.run_system_cached(regenerate_grid);
    }

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

                        commands.run_system_cached_with(update_zoom, new_pos.y);

                        transform.translation = new_pos;
                    }
                }
            }
            _ => {}
        }
    }
}

fn update_zoom(
    zoom: In<f32>,
    mut zoom_res: ResMut<ZoomLevel>,
    map: Res<Map>,
    mut commands: Commands,
) {
    let new_zoom = ZoomLevel::new(zoom.0);

    if zoom_res.zoom() != new_zoom.zoom() {
        map.despawn_recursive(&mut commands);
        zoom_res.level = zoom.0;

        commands.run_system_cached_with(render_grid, new_zoom.zoom());
    }
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    commands.insert_resource(ZoomLevel::new(INITIAL_ZOOM));

    commands.spawn((
        Camera3d {
            screen_space_specular_transmission_quality: ScreenSpaceTransmissionQuality::Low,
            ..default()
        },
        Transform::from_xyz(0.0, INITIAL_ZOOM, INITIAL_ZOOM * 0.6).looking_at(Vec3::ZERO, Vec3::Y),
        MyCameraMarker,
    ));
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
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

fn regenerate_grid(mut map: ResMut<Map>, mut commands: Commands) {
    let rng = &mut rand::rng();

    let pt_world = PtWorld::from_seed(rng.random());

    map.pt_world = pt_world;
    map.despawn_recursive(&mut commands);

    // TODO: get current zoom level
    commands.run_system_cached_with(render_grid, Zoom::In);
}

/// Hex grid setup
fn setup_grid(mut commands: Commands) {
    let rng = &mut rand::rng();

    let pt_world = PtWorld::from_seed(rng.random());

    commands.insert_resource(Map {
        all: vec![],
        pt_world,
        layout: Default::default(),
        render_on: Zoom::In,
    });

    commands.run_system_cached_with(render_grid, ZoomLevel::new(INITIAL_ZOOM).zoom());
}

/// Hex grid setup
fn render_grid(
    zoom: In<Zoom>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if map.render_on == zoom.0 {
        return;
    }

    let region_layout = HexLayout {
        scale: REGION_HEX_SIZE,
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mut biome_handler = HashMap::new();
    for x in Biome::all().into_iter() {
        let color = match x {
            Biome::Desert => css::YELLOW,
            Biome::Forest => css::DARK_GREEN,
            Biome::Water => css::BLUE,
            Biome::Plains => css::GREEN,
            Biome::DenseForest => css::DARK_GREEN,
            Biome::Hill => css::LIGHT_GRAY,
            Biome::Mountain => css::DARK_GRAY,
            Biome::City => css::PURPLE,
            Biome::Road => css::BLACK,
            Biome::CityCenter => css::RED,
        };

        let material = materials.add(Color::Srgba(color));

        biome_handler.insert(x, material);
    }

    let mut all_ent = Vec::new();

    for (r_hex, region) in map.pt_world.regions.iter() {
        // get the center of the region
        let pos = region_layout.hex_to_world_pos(*r_hex) * 1.8;

        let r_layout = HexLayout {
            scale: REGION_HEX_SIZE / map.pt_world.region_radius as f32,
            origin: pos,
            ..default()
        };

        let mesh = hexagonal_column(&r_layout, 2.0, zoom.0.clone());
        let mesh_handle = meshes.add(mesh);

        let ent: Vec<Entity> = region
            .tiles
            .iter()
            .map(|(hex, (tile, _))| {
                let pos = r_layout.hex_to_world_pos(hex);
                let height = (1.0 + tile.noise.height) * 4.0;

                let material = biome_handler
                    .get(&tile.biome)
                    .unwrap_or(&biome_handler[&Biome::Plains]);

                commands
                    .spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_xyz(pos.x, height as f32, pos.y),
                    ))
                    .id()
            })
            .collect();

        all_ent.extend(ent);
    }

    map.all = all_ent;
    map.layout = region_layout;
    map.render_on = zoom.0;
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout, height: f32, zoom: Zoom) -> Mesh {
    let facing = match zoom {
        Zoom::In => Some(FaceOptions::new()),
        Zoom::Out => None,
    };

    // render only what user can see
    let side_options: [Option<FaceOptions>; 6] = [facing, facing, facing, None, None, None];

    let mut mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .center_aligned()
        .with_subdivisions(0)
        .with_multi_custom_sides_options(side_options)
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
    let mut text_info = format!("Hexes: {}\n", state.all.len());

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            let add = format!("FPS: {}\n", format!("{value:.0}"));

            text_info.push_str(add.as_str());
        }
    }

    text_query.get_single_mut().unwrap().0 = text_info;
}

pub fn hover_hex_update(state: Res<Map>, text_query: Query<(&Transform), With<HoverHex>>) {
    for transform in text_query.iter() {
        let pos = transform.translation;
        //println!("Hovering over hex at {:?}", pos);
    }
}
