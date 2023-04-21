use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.8, 0.8, 0.8);
const HOVERED_BUTTON: Color = Color::rgb(0.4, 0.8, 0.8);
const PRESSED_BUTTON: Color = Color::rgb(0.4, 1.0, 1.0);

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameExitEvent>()
            .add_event::<SimulationStartEvent>()
            .add_event::<SimulationStopEvent>()
            .add_startup_system(setup)
            .add_system(button_system);
    }
}

pub struct GameExitEvent;

pub struct SimulationStartEvent;

pub struct SimulationStopEvent;

#[derive(Component)]
struct ClassicButton(ButtonType);

#[derive(PartialEq, Copy, Clone)]
enum ButtonType {
    Start,
    Stop,
    Exit,
}

pub struct MainMenuPlugin;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!("")
}
