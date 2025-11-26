use bevy::prelude::*;

fn main() {
    println!("Hello, world!");

    let mut app: App = App::new();
    app.add_plugins(DefaultPlugins).add_systems(Startup, setup);
    app.run();
}

fn setup(mut commands: Commands) {
 // 相机必须
    commands.spawn(Camera2d);
    
    // 核心组件只有 3 个：Sprite、Transform、Handle<Image>
    // commands.spawn((
    //     Sprite::from_image(assets.load("player.png")), // 1. 精灵贴图
    //     Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(2.0)),
    // ));
}
