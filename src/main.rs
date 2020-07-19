mod game_engine;
use game_engine::entity::Entity;

#[macro_use]
extern crate lazy_static;

fn main() {

    let y = Entity::create_entity(12.0, 3.0);
    let y = Entity::create_entity(2.0, 33.0);
    println!("{:#?}", Entity::get_entities());

    match Entity::get_entities().get_mut(&y) {
        Some(entity) => {
            entity.set_coords(123.123, 321.321);
        },
        None => ()
    }
    println!("{:#?}", Entity::get_entities());

    Entity::get_entities().remove(&y);
    println!("{:#?}", Entity::get_entities());

    let y = Entity::create_entity(2.30, 33.30);
    println!("{}", Entity::get_entities().len());

    Entity::get_entities().values_mut()
        .for_each(|e| e.set_coords(e.get_coords().0 + 2.2, e.get_coords().1 + 2.2));
    
    Entity::get_entities().values()
        .for_each(|e| println!("{:#?}", e));
}
