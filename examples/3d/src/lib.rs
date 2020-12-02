use bevy::prelude::*;

#[creator::creator_main]
pub fn main() {
    println!("Initialization.");
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.88, 0.87, 0.86)))
        .add_plugins(DefaultPlugins)
        // .add_startup_system(icon.system())
        // .add_startup_system(helmet.system())
        .run();
}

fn icon(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let texture_handle = asset_server.load("branding/icon.png");
    // commands
    //     .spawn(Camera2dBundle::default())
    //     .spawn(SpriteBundle {
    //         material: materials.add(texture_handle.into()),
    //         ..Default::default()
    //     });
}

// TODO: Fix support of gltf scenes
// fn helmet(commands: &mut Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn_scene(asset_server.load("models/helmet/FlightHelmet.gltf"))
//         .spawn(LightBundle {
//             transform: Transform::from_translation(Vec3::new(4.0, 5.0, 4.0)),
//             ..Default::default()
//         })
//         .spawn(Camera3dBundle {
//             transform: Transform::from_translation(Vec3::new(0.7, 0.7, 1.0))
//                 .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::unit_y()),
//             ..Default::default()
//         });
// }
