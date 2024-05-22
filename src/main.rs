use bevy::{prelude::*, render::render_resource::ShaderType};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
    ) {

    commands.spawn(Camera2dBundle::default());

    let tile_row: [i32; 4] = [0, 0, 0, 0];
    let tile_map: [[i32; 4]; 4] = [tile_row, tile_row, tile_row, tile_row];

    for (i, _) in tile_map.iter().enumerate() {
        for (j, _) in tile_map[i].iter().enumerate() {
            let x_pos = ((i as f32) * 60.0) + ((j as f32) * 60.0);
            let y_pos = ((i as f32) * 30.0) - ((j as f32) * 30.0);
            commands.spawn(SpriteBundle {
                texture: asset_server.load("tiles/dirt.png"),
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
