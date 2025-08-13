use bevy::prelude::*;
use bevy_egui::EguiPlugin;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper".to_string(),
            resolution: (700., 800.).into(),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins(EguiPlugin::default());
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::default());
    app.add_systems(Startup, camera_setup);
    app.add_plugins(BoardPlugin);

    app.run();

}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}


