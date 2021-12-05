use std::fmt::{Display, Formatter};
use std::time::Duration;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use fake::Fake;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(RabbitTimer::setup.system())
        // .add_startup_system(LogBoard::setup.system())
        .add_system(RabbitTimer::time.system())
        .add_system(keyboard_add.system())
        .add_system(mouse_add.system())
        .run();
}

struct LogBoard;

impl LogBoard {
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // TODO: setup logboard to list names
        todo!();
    }
}

// TODO: Add Species
struct Rabbit;

struct Name(String);

impl Name {
    pub fn random() -> Self {
        if rand::random() {
            Self(fake::faker::name::en::FirstName().fake())
        } else {
            Self(fake::faker::name::fr_fr::FirstName().fake())
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

struct RabbitTimer(Timer);

impl RabbitTimer {
    pub fn setup(mut commands: Commands) {
        commands.insert_resource(Self(Timer::new(Duration::from_secs(1), true)));
    }

    pub fn time(mut commands: Commands, time: Res<Time>, mut timer: ResMut<Self>) {
        if timer.tick(time) {
            let name = Name::random();
            info!("tick: new rabbit: {}", name);
            commands.spawn().insert(Rabbit).insert(name);
        }
    }

    pub fn tick(&mut self, time: Res<Time>) -> bool {
        self.0.tick(time.delta()).just_finished()
    }
}

fn keyboard_add(mut commands: Commands, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        let name = Name::random();
        info!("keyboard: new rabbit: {}", name);
        commands.spawn().insert(Rabbit).insert(name);
    }
}

fn mouse_add(mut commands: Commands, mouse: Res<Input<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        let name = Name::random();
        info!("mouse: new rabbit: {}", name);
        commands.spawn().insert(Rabbit).insert(name);
    }
}
