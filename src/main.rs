use std::fmt::format;
use rand::prelude::*; 

use rusty_engine::{game, prelude::{
    bevy::{
        input::mouse,
        utils::{label, tracing::event},
    },
    *,
}};

#[derive(Resource)]
struct GameState {
    health_left: i32,
    high_score: u32,
    score: u32,
    lost:bool,
    //enemy_labels: Vec<String>,
    ferris_index: i32, 
     spawn_timer: Timer,
     health_amount: u8,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 5,
            lost:false,
            health_left: 100,
            high_score: 0,
            ferris_index: 0,
            score: 0,
            // enemy_labels: Vec::new(),
              spawn_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn main() {
    let mut game = Game::new();

  // game.window_settings(Wind {
    //    title: "Tutorial!".to_string(),
  //      ..Default::default()
  //  });

    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle , 0.1);

    // Create road

    for i in 0..10{
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
    roadline.scale = 0.1;
    roadline.translation.x = -600.0 + 150.0 * i as f32;
   }

   let obstacle_presets = vec![
    SpritePreset::RacingBarrelBlue,
    SpritePreset::RacingBarrierRed,
    SpritePreset::RacingConeStraight,
   ];
   for (i, preset ) in obstacle_presets.into_iter().enumerate() {
    let obstacle =  game.add_sprite(format!("obstacle{}", i), preset);
    obstacle.layer = 5.0;
    obstacle.collision = true;
    obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
    obstacle.translation.y = thread_rng().gen_range(-300.0..300.0);
   }

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);

    player.translation = Vec2::new(-500.0, 0.0);
    player.layer = 10.0;
    player.rotation = std::f32::consts::FRAC_PI_2;
    player.rotation = LEFT;
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

   // let high_score = game.add_text("high_score", "High score: 0");
  //  high_score.translation = Vec2::new(-520.0, 320.0);

  let health_message = game.add_text("health_message", "Helth: 5");
  health_message.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

  if game_state.lost {
    return ;
  }

    if engine.keyboard_state.just_pressed(KeyCode::Q){
        engine.should_exit = true;
    }
  

   // Keep text near the edges of the screen
   let offset = ((engine.time_since_startup_f64 * 3.0).cos() * 5.0) as f32;
   let score =  engine.texts.get_mut("score").unwrap();
   score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
   score.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

   let health_message =  engine.texts.get_mut("health_message").unwrap();
   health_message.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
   health_message.translation.y = engine.window_dimensions.y / 2.0 - 30.0 + offset;

   for event in engine.collision_events.drain(..){
  if !event.pair.either_contains("player") || event.state.is_end() {
    continue;
  }
  if game_state.health_amount > 0 {
    game_state.health_amount -=1;
    health_message.value = format!("Health: {}", game_state.health_amount);
    engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.4)
  }
   }

    //Handle collisions
    engine.show_colliders = true;
    for event in engine.collision_events.drain(..) {
        if (event.state == CollisionState::Begin && event.pair.one_starts_with("player")) {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }

            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);
            if game_state.score > game_state.high_score {
                game_state.high_score = game_state.score;
                let high_score = engine.texts.get_mut("high_score").unwrap();
                high_score.value = format!("High score: {}", game_state.high_score);
            }
            engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.3);
        }
    }
    // Handle movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 250.0;
    const ROAD_SPEED:f32 = 400.0;

    let mut direction = 0.0;
    // player.translation.x += 100.0 * engine.delta_f32;
    //if  ( engine.keyboard_state.pressed_any(KeyCode::Up) || engine.keyboard_state.pressed(KeyCode::W)) {
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W]))
    {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
   //    player.rotation = player.rotation *  0.15;
   direction += 1.0;
   player.rotation = direction * 0.15;
    }
    if (engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S]))
    {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
        direction -= 1.0;
        player.rotation = direction * 0.15;
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

   // if engine.mouse_state.just_pressed(MouseButton::Left) {
//        if let Some(mouse_location) = engine.mouse_state.location() {
//            let label = format!("ferris {}", game_state.ferris_index);
 //           game_state.ferris_index += 1;
  //          let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
  //          ferris.translation = mouse_location;
  //          ferris.collision = true;
 //       }
  //  }
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris {}", game_state.ferris_index);
        game_state.ferris_index += 1;
   //     let ferris = engine.add_sprite(label.clone(), SpritePreset::RacingCarYellow);
    //    ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
 //       ferris.translation.y = thread_rng().gen_range(-325.0..325.0);
 //       ferris.collision = true;
    }

    // Reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }

    if player.translation.y < -360.0 || player.translation.y > 360.0 {
        game_state.health_amount = 0;
    }


    // Move road

    for sprite in engine.sprites.values_mut(){
        if sprite.label.starts_with("roadline") {
        sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
        if sprite.translation.x < -675.0 {
            sprite.translation.x += 1500.0;
        }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.x = thread_rng().gen_range(-300.0..300.0);
            }
        }
    }
    if game_state.health_amount ==0 {
        game_state.lost = true;
        let game_over = engine.add_text("game over", "Game Over");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
