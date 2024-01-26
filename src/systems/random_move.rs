use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();
    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let desination = get_random_movement(&mut rng) + *pos;
        commands.push((
            (),
            WantsToMove {
                entity: *entity,
                destination: desination,
            },
        ));
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
