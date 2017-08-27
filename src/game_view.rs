use std::collections::HashMap;
use hate::{Context, Scene, Sprite, Time};
use hate::scene::{Action, Layer};
use core::ObjId;

#[derive(Debug, Clone, Default)]
pub struct Layers {
    pub bg: Layer,
    pub blood: Layer,
    pub grass: Layer,
    pub walkable_tiles: Layer,
    pub attackable_tiles: Layer,
    pub selection_marker: Layer,
    pub fg: Layer,
    pub text: Layer,
}

impl Layers {
    fn sorted(self) -> Vec<Layer> {
        vec![
            self.bg,
            self.blood,
            self.grass,
            self.walkable_tiles,
            self.attackable_tiles,
            self.selection_marker,
            self.fg,
            self.text,
        ]
    }
}

#[derive(Debug)]
pub struct GameView {
    tile_size: f32,
    layers: Layers,
    obj_to_sprite_map: HashMap<ObjId, Sprite>,
    scene: Scene,
}

impl GameView {
    pub fn new() -> Self {
        let obj_to_sprite_map = HashMap::new();
        let layers = Layers::default();
        let scene = Scene::new(layers.clone().sorted());
        let tile_size = 0.11;
        Self {
            scene,
            tile_size,
            layers,
            obj_to_sprite_map,
        }
    }

    pub fn tick(&mut self, context: &mut Context, dtime: Time) {
        self.scene.tick(dtime);
        self.scene.draw(context);
    }

    pub fn add_action(&mut self, action: Box<Action>) {
        self.scene.add_action(action);
    }

    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }

    pub fn layers(&self) -> &Layers {
        &self.layers
    }

    pub fn add_object(&mut self, id: ObjId, sprite: &Sprite) {
        self.obj_to_sprite_map.insert(id, sprite.clone());
    }

    pub fn remove_object(&mut self, id: ObjId) {
        self.obj_to_sprite_map.remove(&id).unwrap();
    }

    pub fn id_to_sprite(&mut self, id: ObjId) -> &Sprite {
        &self.obj_to_sprite_map[&id]
    }
}
