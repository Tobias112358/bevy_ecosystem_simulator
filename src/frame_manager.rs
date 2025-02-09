use bevy::prelude::*;
use std::time::Duration;


#[derive(Resource)]
pub struct FrameControl {
    pub timer: Timer,
}

pub(super) fn plugin(app: &mut App) {
    app
        .insert_resource(FrameControl {
            timer: Timer::new(Duration::from_millis(90), TimerMode::Repeating),
        })
        .add_systems(Update, frame_control_tick);
}

pub fn frame_control_tick(
    time: Res<Time>,
    mut animal_frame_control: ResMut<FrameControl>,
) {
    animal_frame_control.timer.tick(time.delta());
}