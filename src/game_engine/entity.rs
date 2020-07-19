use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref ENTITIES: Mutex<HashMap<Uuid, Entity>> = {
        Mutex::new(HashMap::new())
    };    
}

#[derive(Debug)]
pub struct Entity {
    id: Uuid,
    x: f32,
    y: f32,
    //TODO: sprite
    //spite: Sprite
    //TODO: image index
    //image_index: ImageIndex
}

impl Entity {
    pub fn create_entity(x: f32, y: f32) -> Uuid {
        let id = Uuid::new_v4();
        let e = Entity {
            id,
            x,
            y
        };
        Entity::add_entity(id, e);
        id
    }

    pub fn add_entity(id: Uuid, entity: Entity) {
        ENTITIES.lock().unwrap().insert(id, entity);
    }

    pub fn get_entities() -> MutexGuard<'static,HashMap<Uuid, Entity>> {
        ENTITIES.lock().unwrap()
    }

    pub fn set_coords(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    
    pub fn get_coords(&self) -> (f32, f32) {
         (self.x,self.y)
    }
}