use bevy::prelude::*;

pub fn load_scene_system(asset_server: Res<AssetServer>, mut scene_spawner: ResMut<SceneSpawner>) {
    // Scenes are loaded just like any other asset.
    let scene_handle: Handle<DynamicScene> = asset_server.load("saves/quicksave_objects.scn.ron");

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in
    // the World with new entity ids. This guarantees that it will not overwrite existing
    // entities.
    scene_spawner.spawn_dynamic(scene_handle);
}
