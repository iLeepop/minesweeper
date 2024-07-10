use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, (camera_setup,));

    app.run();

}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


