mod enums;

use enums::{
    Villager,
    Direction,
    World,
};



fn main() {
    for _ in 0..10 {
        let world = World::random();
        let World {villager_1, villager_2, villager_3, direction_heaven} = world;
        
        // Your solution here
        // Example question to villager_1: "Does left lead to heaven?"
        let first_response = villager_1.ask_equal(Direction::Left, direction_heaven);

        println!("First response: {}", first_response);

        let second_response = if first_response {
            // You can make a more complex, composite question
            // To villager_2: "If I asked villager_ 1 and villager_ 2 the statement "does the right path
            // lead to heaven?", would their answers be the same?"
            villager_3.ask_equal(villager_1.ask_equal(Direction::Right, direction_heaven), villager_2.ask_equal(Direction::Right, direction_heaven))
        } else {
            // You can also ask about other tribesmen
            // To villager_3: "If I asked villager_ 1 if villager_ 3 is a truth teller, would he say yes?"
            villager_2.eval(villager_1.ask_equal(villager_3, Villager::Truth))
        };

        println!("Second response: {}", second_response);

        let direction = if second_response {
            Direction::Left
        } else {
            Direction::Right
        };

        world.check_answer(direction);

        println!("{}\n\n", world);
    }
}
