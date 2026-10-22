mod enums;

use enums::{
    Villager,
    Direction,
    World,
};



fn main() {
    for _ in 0..10 {
        let world = World::random();
        let World {villager_1, villager_2, direction_heaven} = world;
        
        // question: If I asked if the left road leads to heaven, would you say yes?
        let response = villager_1.eval(
            villager_1.ask_equal(Direction::Left, direction_heaven),
        );

        println!("Response: {}", response);

        let direction = if response {
            Direction::Left
        } else {
            Direction::Right
        };

        world.check_answer(direction);

        println!("{}\n\n", world);
    }
}
