use bevy::core_pipeline::core_3d::ScreenSpaceTransmissionQuality;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::gizmos::primitives::dim3::Plane3dBuilder;
use bevy::input::mouse::MouseWheel;
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::render::mesh::VertexAttributeValues;
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
use std::collections::{HashMap, HashSet};
use std::ops::Add;
use std::time::Duration;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
const REGION_HEX_SIZE: Vec2 = Vec2::splat(30.0);
const REGION_CHUNKS_SIZE_RADIUS: u32 = 10;
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
        .add_plugins(WireframePlugin)
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
        .add_systems(Update, toggle_wireframe)
        .add_systems(
            Update,
            show_info.run_if(on_timer(Duration::from_millis(100))),
        )
        .run();
}

fn toggle_wireframe(
    mut active: Local<bool>,
    mut commands: Commands,
    map: Res<Map>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        *active = !*active;

        if *active {
            for entity in map.all.iter() {
                commands.get_entity(*entity).map(|mut e| {
                    e.insert(Wireframe);
                });
            }
        } else {
            for entity in map.all.iter() {
                commands.get_entity(*entity).map(|mut e| {
                    e.remove::<Wireframe>();
                });
            }
        }
    }
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

    if keys.just_pressed(KeyCode::Digit1) {
        commands.run_system_cached_with(render_type, RenderType::MeshToHex);
    }

    if keys.just_pressed(KeyCode::Digit2) {
        commands.run_system_cached_with(render_type, RenderType::MeshToHexChunk);
    }

    if keys.just_pressed(KeyCode::Digit3) {
        commands.run_system_cached_with(render_type, RenderType::LowHex);
    }

    if keys.just_pressed(KeyCode::Digit4) {
        commands.run_system_cached_with(render_type, RenderType::Hex3D);
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

                        transform.translation = new_pos;
                    }
                }
            }
            _ => {}
        }
    }
}

enum RenderType {
    MeshToHex,
    MeshToHexChunk,
    LowHex,
    Hex3D,
}

fn render_type(input: In<RenderType>, mut map: ResMut<Map>, mut commands: Commands) {
    map.despawn_recursive(&mut commands);

    match input.0 {
        RenderType::MeshToHex => {
            commands.run_system_cached_with(render_grid_all_hex, Zoom::In);
        }
        RenderType::MeshToHexChunk => {
            commands.run_system_cached(render_grid_all_hex_chunk);
        }
        RenderType::LowHex => {
            commands.run_system_cached(render_grid_low_hex);
        }
        RenderType::Hex3D => {
            commands.run_system_cached(render_grid_3d_plane);
        }
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
    commands.run_system_cached_with(render_grid, (false, Zoom::In));
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

    commands.run_system_cached_with(render_grid, (true, ZoomLevel::new(INITIAL_ZOOM).zoom()));
}

/// Hex grid setup
fn render_grid(zoom: In<(bool, Zoom)>, mut map: ResMut<Map>, mut commands: Commands) {
    if zoom.0 .0 == true && map.render_on == zoom.0 .1 {
        return;
    }

    map.render_on = zoom.0 .1;

    if map.render_on == Zoom::In {
        commands.run_system_cached(render_grid_low_hex);
    } else {
        commands.run_system_cached(render_grid_low_hex);
    }
}

fn biome_handler(
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) -> HashMap<Biome, Handle<StandardMaterial>> {
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

    biome_handler
}

fn render_grid_all_hex(
    zoom: In<Zoom>,
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    for (r_hex, region) in map
        .pt_world
        .regions
        .iter()
        .filter(|(hex, _)| **hex == Hex::ZERO)
    {
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

// TODO: Fix me
fn render_grid_3d_plane(
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let region_layout = HexLayout {
        scale: REGION_HEX_SIZE,
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mut biome_handler = biome_handler(&mut materials);

    let mut ids = Vec::new();

    for (r_hex, region) in map.pt_world.regions.iter() {
        // get the center of the region
        let pos = region_layout.hex_to_world_pos(*r_hex) * 1.8;

        let r_layout = HexLayout {
            scale: REGION_HEX_SIZE / map.pt_world.region_radius as f32,
            origin: pos,
            ..default()
        };

        let mut terrain = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(
                    map.pt_world.region_radius as f32,
                    map.pt_world.region_radius as f32,
                )
                .subdivisions(map.pt_world.region_radius * 10),
        );

        if let Some(VertexAttributeValues::Float32x3(positions)) =
            terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            for pos in positions.iter_mut() {
                let [x, y, z] = pos;

                let hex = r_layout.world_pos_to_hex(Vec2::new(*x, *z));

                region.tiles.get(hex).map(|(tile, _)| {
                    pos[1] = ((1.0 + tile.noise.height) * 6.0) as f32;
                });
            }

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[x, y, z]| {
                    let hex = r_layout.world_pos_to_hex(Vec2::new(*x, *z));

                    region
                        .tiles
                        .get(hex)
                        .map(|(tile, _)| {
                            let color = match tile.biome {
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

                            return Color::Srgba(color).to_linear().to_f32_array();
                        })
                        .unwrap_or(Color::WHITE.to_linear().to_f32_array())
                })
                .collect();

            terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }

        terrain.compute_normals();

        let mesh_handle = meshes.add(terrain);

        let id = commands
            .spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(biome_handler[&Biome::Plains].clone()),
                Transform::from_xyz(pos.x, 1.0, pos.y),
            ))
            .id();

        ids.push(id);
    }

    map.all.extend(ids);
}

fn render_grid_all_hex_chunk(
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let region_layout = HexLayout {
        scale: REGION_HEX_SIZE,
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mut all_ent = Vec::new();

    for (r_hex, region) in map
        .pt_world
        .regions
        .iter()
        .filter(|(hex, _)| **hex == Hex::ZERO)
    {
        // get the center of the region
        let pos = region_layout.hex_to_world_pos(*r_hex) * 1.8;

        // Withou position, all meshes will be at the same place at render, but the transform will be on center
        let r_layout = HexLayout {
            scale: REGION_HEX_SIZE / map.pt_world.region_radius as f32,
            ..default()
        };

        let mut chunks = HashSet::new();

        for hex in Hex::ZERO.range(map.pt_world.region_radius) {
            let chunk_id = hex.to_lower_res(REGION_CHUNKS_SIZE_RADIUS);

            chunks.insert(chunk_id);
        }

        for chunk in chunks {
            let chunk_center = chunk.to_higher_res(REGION_CHUNKS_SIZE_RADIUS);

            let chunk_bound = HexBounds::new(chunk_center, REGION_CHUNKS_SIZE_RADIUS);

            for render_chunk in region.render_chunks(&chunk_bound) {
                let handler = biome_handler(&mut materials);

                let color = handler[&render_chunk.biome].clone();

                // TODO: Apply greedy meshing
                let mut builder = ColumnMeshBuilder::new(&r_layout, 20.0)
                    .without_bottom_face()
                    .center_aligned()
                    .with_multi_custom_sides_options([
                        Some(FaceOptions::new()),
                        Some(FaceOptions::new()),
                        Some(FaceOptions::new()),
                        None,
                        None,
                        None,
                    ]);

                let mut final_mesh_info = MeshInfo::default();

                for hex_in_chunk in render_chunk.tiles {
                    let mut builder = builder.clone().at(hex_in_chunk);

                    region.tiles.get(hex_in_chunk).map(|(tile, _)| {
                        builder.height = ((1.0 + tile.noise.height) * 4.0) as f32;
                        let mesh_info = builder.build();

                        final_mesh_info.merge_with(mesh_info);
                    });
                }

                let mut final_mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    final_mesh_info.vertices.to_vec(),
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, final_mesh_info.normals)
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, final_mesh_info.uvs)
                .with_inserted_indices(Indices::U16(final_mesh_info.indices));

                final_mesh.compute_normals();

                let mesh_handle = meshes.add(final_mesh);

                let id = commands
                    .spawn((
                        Mesh3d(mesh_handle.clone()),
                        MeshMaterial3d(color),
                        Transform::from_xyz(pos.x, 1.0, pos.y),
                    ))
                    .id();

                all_ent.push(id);
            }
        }
    }
    map.all = all_ent;
    map.layout = region_layout;
}

fn render_grid_low_hex(
    mut map: ResMut<Map>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let region_layout = HexLayout {
        scale: REGION_HEX_SIZE,
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let mut all_ent = Vec::new();

    for (r_hex, region) in map.pt_world.regions.iter() {
        // get the center of the region
        let pos = region_layout.hex_to_world_pos(*r_hex) * 1.8;

        // Withou position, all meshes will be at the same place at render, but the transform will be on center
        let r_layout = HexLayout {
            scale: REGION_HEX_SIZE / map.pt_world.region_radius as f32,
            ..default()
        };

        let mut chunks = HashSet::new();

        for hex in Hex::ZERO.range(map.pt_world.region_radius) {
            let chunk_id = hex.to_lower_res(REGION_CHUNKS_SIZE_RADIUS);

            chunks.insert(chunk_id);
        }

        for chunk in chunks {
            let chunk_center = chunk.to_higher_res(REGION_CHUNKS_SIZE_RADIUS);

            let mut builder = ColumnMeshBuilder::new(&r_layout, 20.0)
                .without_bottom_face()
                .center_aligned()
                .with_multi_custom_sides_options([
                    Some(FaceOptions::new()),
                    Some(FaceOptions::new()),
                    Some(FaceOptions::new()),
                    None,
                    None,
                    None,
                ]);

            let mut final_mesh_info = MeshInfo::default();

            for hex_in_chunk in chunk_center.range(REGION_CHUNKS_SIZE_RADIUS) {
                let mut builder = builder.clone().at(hex_in_chunk);

                region.tiles.get(hex_in_chunk).map(|(tile, _)| {
                    builder.height = ((1.0 + tile.noise.height) * 4.0) as f32;
                    let mesh_info = builder.build();

                    final_mesh_info.merge_with(mesh_info);
                });
            }

            let colors: Vec<_> = final_mesh_info
                .vertices
                .iter()
                .map(|position| {
                    let hex = r_layout.world_pos_to_hex(Vec2::new(position.x, position.z));

                    let biome = region
                        .tiles
                        .get(hex)
                        .map(|(tile, _)| tile.biome)
                        .unwrap_or(Biome::Plains);

                    let color = match biome {
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

                    Color::Srgba(color).to_linear().to_f32_array()
                })
                .collect();

            let final_mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, final_mesh_info.vertices.to_vec())
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, final_mesh_info.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, final_mesh_info.uvs)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_indices(Indices::U16(final_mesh_info.indices));

            let mesh_handle = meshes.add(final_mesh);

            let id = commands
                .spawn((
                    Mesh3d(mesh_handle.clone()),
                    MeshMaterial3d(materials.add(Color::WHITE)),
                    Transform::from_xyz(pos.x, 1.0, pos.y),
                ))
                .id();

            all_ent.push(id);
        }
    }
    map.all = all_ent;
    map.layout = region_layout;
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
