use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &MovingRandomly)>::query();
    movers.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let desination = get_random_movement(&mut rng) + *pos;
        if map.can_enter_tile(desination) {
            *pos = desination;
        }
    })
}

fn get_random_movement(rng: &mut RandomNumberGenerator) -> Point {
    match rng.range(0, 4) {
        0 => Point::new(-1, 0),
        1 => Point::new(1, 0),
        2 => Point::new(0, 1),
        _ => Point::new(0, -1),
    }
}
