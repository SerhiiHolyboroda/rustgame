use std::fmt::format;

use rusty_engine::prelude::{
    bevy::{input::mouse, utils::{label, tracing::event}},
    *,
};

#[derive(Resource)]
struct GameState {
    health_left: i32,
//    high_score: u32,
    current_score: u32,
    //enemy_labels: Vec<String>,
    ferris_index:i32
   // spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_left: 100,
        //    high_score: 0,
        ferris_index:0,
            current_score: 0,
           // enemy_labels: Vec::new(),
        //    spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn main() {
    let mut game = Game::new();

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);

    player.translation = Vec2::new(200.0, 0.0);
    player.rotation = std::f32::consts::FRAC_PI_2;
    player.rotation = UP;
    player.scale = 1.0;
    player.collision = true;

  
    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    //Handle collisions
    engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        if (event.state == CollisionState::Begin && event.pair.one_starts_with("player")) {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            game_state.current_score += 1;
            println!("Current score: {}", game_state.current_score);
        }
    }
    // Handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;
    // player.translation.x += 100.0 * engine.delta_f32;
    //if  ( engine.keyboard_state.pressed_any(KeyCode::Up) || engine.keyboard_state.pressed(KeyCode::W)) {
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W]))
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S]))
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A]))
    {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D]))
    {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    if engine.mouse_state.just_pressed(MouseButton::Left){
        if let Some(mouse_location) = engine.mouse_state.location() {
         let label = format!("ferris {}", game_state.ferris_index);
         game_state.ferris_index += 1;
            let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
            ferris.translation = mouse_location;
            ferris.collision = true;
        }
    }
    
    }

