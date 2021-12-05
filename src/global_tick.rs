use std::time::Duration;
use bevy::prelude::*;

pub struct GlobalTick;

impl Plugin for GlobalTick {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(GlobalTimer(Timer::new(Duration::from_micros(1), true)))
            .add_system(GlobalTimer::tick.system());
    }
}

struct GlobalTimer(Timer);

impl GlobalTimer {
    pub fn tick(mut s: ResMut<Self>, time: Res<Time>) -> bool {
        s.0.tick(time.delta()).just_finished()
    }
}