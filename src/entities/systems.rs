use std::ops::Range;

fn interval(low: f32, high: f32) -> Range<i64> {
    (low.floor() as i64)..(high.ceil() as i64)
}

/*fn object_movement(
    chunk_manager: Res<ChunkManager>,
    mut objects: Query<(&mut Object, &mut Transform)>,
    world: Query<(&Chunk)>,
) {
    for (mut object, mut transform) in objects.iter() {
        let old_position = transform.translation;
        object.calc_velocity();
        let velocity = object.velocity();
        let movement = velocity;
        if velocity.y < 0.0 {
            //Check down!
            let collider = object.get_collider();

            for x in interval(collider.lower().x, collider.higher().x) {
                for z in interval(collider.lower().z, collider.higher().z) {

                }
            }
        }

        transform.translation += movement;

        //we assume, that the old position was valid, now we have to check the new position

    }
}*/