use crate::app::Character;
use crate::material::MaterialLibrary;
use crate::scene::Scene;

/// Empty visual scaffold. The new diorama will be designed here from scratch.
pub fn build_stage(_character: Character, _materials: &MaterialLibrary) -> Scene {
    Scene::with_background(0x10152C)
}

/// Stable animated entry point for the app state machine.
pub fn build_stage_at(character: Character, materials: &MaterialLibrary, _time: f32) -> Scene {
    build_stage(character, materials)
}
