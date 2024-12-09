use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            ..default()
        }))
        .add_systems(Startup, 
            (
                setup_asset,
            )
        )
        .add_systems(Update, 
        (
                update_sprites,
                update_debug,
            ) 
        )      
        .run();
}

#[derive(Component)]
pub struct SpriteInfo{
    pub file_name: String
}

pub fn setup_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Camera2d::default(),
    ));   
    for p in vec!["black.png", "blue.png", "green.png", "red.png", "white.png"]{
        commands.spawn((
            Sprite::from_image(asset_server.load(p)),
            SpriteInfo{
                file_name: String::from(p)
            }
        ));
    }
}

pub fn update_sprites(
    mut sprites: Query<(&SpriteInfo, &mut Transform)>,
){
    let mut os_tx = -400.0;
    for (_si, mut transform) in sprites.iter_mut(){
        os_tx += 140.0;
        transform.translation = Vec3::new(os_tx, 0.0, 0.0);
    }
}

pub fn update_debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sprites: Query<&SpriteInfo>,
){
    if keyboard_input.just_released(KeyCode::KeyZ) {
        let mut files = Vec::new();
        for s in sprites.iter(){
            files.push(&s.file_name);
        }
        println!("{:?}", files);
    }
}