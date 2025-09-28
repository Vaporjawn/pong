// Pong Game Library
// This exposes the core game components for testing and modular organization

use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};
use ::rand::Rng;

// Game Constants
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const PADDLE_WIDTH: f32 = 15.0;
pub const PADDLE_HEIGHT: f32 = 80.0;
pub const PADDLE_SPEED: f32 = 300.0;
pub const BALL_SIZE: f32 = 15.0;
pub const BALL_SPEED: f32 = 350.0;
pub const WINNING_SCORE: i32 = 5;

// Audio generation functions
pub fn generate_paddle_hit_sound() -> Vec<u8> {
    let sample_rate = 44100u32;
    let duration = 0.1; // 100ms
    let samples = (sample_rate as f64 * duration) as usize;

    let mut audio_data = Vec::with_capacity(samples * 2);

    for i in 0..samples {
        let t = i as f64 / sample_rate as f64;
        let frequency = 800.0 + (400.0 * (-t * 10.0).exp());
        let amplitude = 0.3 * (-t * 8.0).exp();
        let sample = (amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin()) * 32767.0;
        let sample = sample.clamp(-32767.0, 32767.0) as i16;

        let bytes = sample.to_le_bytes();
        audio_data.push(bytes[0]);
        audio_data.push(bytes[1]);
    }

    create_wav_file(&audio_data, sample_rate)
}

pub fn generate_wall_hit_sound() -> Vec<u8> {
    let sample_rate = 44100u32;
    let duration = 0.08; // 80ms
    let samples = (sample_rate as f64 * duration) as usize;

    let mut audio_data = Vec::with_capacity(samples * 2);

    for i in 0..samples {
        let t = i as f64 / sample_rate as f64;
        let frequency = 300.0 + (200.0 * (-t * 15.0).exp());
        let amplitude = 0.25 * (-t * 12.0).exp();
        let sample = (amplitude * (2.0 * std::f64::consts::PI * frequency * t).sin()) * 32767.0;
        let sample = sample.clamp(-32767.0, 32767.0) as i16;

        let bytes = sample.to_le_bytes();
        audio_data.push(bytes[0]);
        audio_data.push(bytes[1]);
    }

    create_wav_file(&audio_data, sample_rate)
}

pub fn generate_score_sound() -> Vec<u8> {
    let sample_rate = 44100u32;
    let duration = 0.4; // 400ms
    let samples = (sample_rate as f64 * duration) as usize;

    let mut audio_data = Vec::with_capacity(samples * 2);

    for i in 0..samples {
        let t = i as f64 / sample_rate as f64;
        let mut sample_value = 0.0;

        // Major chord progression for victory sound
        let frequencies = [523.25, 659.25, 783.99]; // C, E, G
        for &freq in &frequencies {
            let wave = (2.0 * std::f64::consts::PI * freq * t).sin();
            sample_value += wave * 0.15;
        }

        let envelope = (-t * 2.0).exp();
        sample_value *= envelope;

        let sample = (sample_value * 32767.0).clamp(-32767.0, 32767.0) as i16;

        let bytes = sample.to_le_bytes();
        audio_data.push(bytes[0]);
        audio_data.push(bytes[1]);
    }

    create_wav_file(&audio_data, sample_rate)
}

fn create_wav_file(audio_data: &[u8], sample_rate: u32) -> Vec<u8> {
    let mut wav_file = Vec::new();

    // WAV header
    wav_file.extend_from_slice(b"RIFF");
    let file_size = (36 + audio_data.len()) as u32;
    wav_file.extend_from_slice(&file_size.to_le_bytes());
    wav_file.extend_from_slice(b"WAVE");

    // Format chunk
    wav_file.extend_from_slice(b"fmt ");
    wav_file.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
    wav_file.extend_from_slice(&1u16.to_le_bytes()); // PCM format
    wav_file.extend_from_slice(&1u16.to_le_bytes()); // Mono
    wav_file.extend_from_slice(&sample_rate.to_le_bytes());
    wav_file.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // Byte rate
    wav_file.extend_from_slice(&2u16.to_le_bytes()); // Block align
    wav_file.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

    // Data chunk
    wav_file.extend_from_slice(b"data");
    wav_file.extend_from_slice(&(audio_data.len() as u32).to_le_bytes());
    wav_file.extend_from_slice(audio_data);

    wav_file
}

// Math utilities
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == 0.0 {
            *self
        } else {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
}

// Particle system
pub struct Particle {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let speed = rng.gen_range(50.0..200.0);
        let lifetime = rng.gen_range(0.5..2.0);

        Self {
            position: Vec2D::new(x, y),
            velocity: Vec2D::new(angle.cos() * speed, angle.sin() * speed),
            lifetime,
            max_lifetime: lifetime,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.lifetime -= dt;

        self.lifetime > 0.0
    }

    pub fn draw(&self) {
        let alpha = self.lifetime / self.max_lifetime;
        let size = 3.0 * alpha;
        draw_circle(
            self.position.x,
            self.position.y,
            size,
            Color::new(1.0, 1.0, 1.0, alpha),
        );
    }
}

// Paddle entity
pub struct Paddle {
    pub position: Vec2D,
    pub velocity: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2D::new(x, y),
            velocity: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position.y += self.velocity * dt;

        // Keep paddle within window bounds
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        } else if self.position.y > WINDOW_HEIGHT - PADDLE_HEIGHT {
            self.position.y = WINDOW_HEIGHT - PADDLE_HEIGHT;
        }
    }

    pub fn draw(&self) {
        // Draw main paddle
        draw_rectangle(
            self.position.x,
            self.position.y,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
            WHITE,
        );

        // Draw glow effect
        let glow_intensity = 0.3;
        for i in 1..=3 {
            let offset = i as f32 * 2.0;
            let alpha = glow_intensity / (i as f32);
            draw_rectangle_lines(
                self.position.x - offset,
                self.position.y - offset,
                PADDLE_WIDTH + offset * 2.0,
                PADDLE_HEIGHT + offset * 2.0,
                2.0,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }
    }

    pub fn get_center_y(&self) -> f32 {
        self.position.y + PADDLE_HEIGHT / 2.0
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, PADDLE_WIDTH, PADDLE_HEIGHT)
    }
}

// Ball entity
pub struct Ball {
    pub position: Vec2D,
    pub velocity: Vec2D,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle = if rng.gen::<bool>() {
            rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0)
        } else {
            rng.gen_range(3.0 * std::f32::consts::PI / 4.0..5.0 * std::f32::consts::PI / 4.0)
        };

        Self {
            position: Vec2D::new(x, y),
            velocity: Vec2D::new(BALL_SPEED * angle.cos(), BALL_SPEED * angle.sin()),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;

        // Bounce off top and bottom walls
        if self.position.y <= 0.0 || self.position.y >= WINDOW_HEIGHT - BALL_SIZE {
            self.velocity.y = -self.velocity.y;
            self.position.y = self.position.y.clamp(0.0, WINDOW_HEIGHT - BALL_SIZE);
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, BALL_SIZE, BALL_SIZE, WHITE);

        // Draw glow effect
        let glow_intensity = 0.4;
        for i in 1..=3 {
            let offset = i as f32 * 1.5;
            let alpha = glow_intensity / (i as f32);
            draw_rectangle_lines(
                self.position.x - offset,
                self.position.y - offset,
                BALL_SIZE + offset * 2.0,
                BALL_SIZE + offset * 2.0,
                1.0,
                Color::new(1.0, 1.0, 1.0, alpha),
            );
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, BALL_SIZE, BALL_SIZE)
    }

    pub fn reset(&mut self) {
        self.position = Vec2D::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0);

        let mut rng = ::rand::thread_rng();
        let angle = if rng.gen::<bool>() {
            rng.gen_range(-std::f32::consts::PI / 4.0..std::f32::consts::PI / 4.0)
        } else {
            rng.gen_range(3.0 * std::f32::consts::PI / 4.0..5.0 * std::f32::consts::PI / 4.0)
        };

        self.velocity = Vec2D::new(BALL_SPEED * angle.cos(), BALL_SPEED * angle.sin());
    }

    pub fn handle_paddle_collision(&mut self, paddle: &Paddle) {
        let ball_rect = self.get_rect();
        let paddle_rect = paddle.get_rect();

        if ball_rect.overlaps(&paddle_rect) {
            // Calculate relative intersection point (between -1.0 and 1.0)
            let intersect_y = (self.position.y + BALL_SIZE / 2.0) - paddle.get_center_y();
            let normalized_intersect = intersect_y / (PADDLE_HEIGHT / 2.0);

            // Calculate new angle based on intersection point
            let angle = normalized_intersect * std::f32::consts::PI / 4.0;

            // Reverse horizontal direction and apply angle
            let direction = if self.velocity.x > 0.0 { -1.0 } else { 1.0 };
            self.velocity.x = BALL_SPEED * angle.cos() * direction;
            self.velocity.y = BALL_SPEED * angle.sin();

            // Move ball away from paddle to prevent multiple collisions
            if direction < 0.0 {
                self.position.x = paddle_rect.x - BALL_SIZE;
            } else {
                self.position.x = paddle_rect.x + PADDLE_WIDTH;
            }
        }
    }
}

// Game states
#[derive(PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}

// Main game structure
pub struct Game {
    pub player_paddle: Paddle,
    pub ai_paddle: Paddle,
    pub ball: Ball,
    pub player_score: i32,
    pub ai_score: i32,
    pub game_state: GameState,
    pub particles: Vec<Particle>,
    pub ball_trail: Vec<Vec2D>,
    pub paddle_hit_sound: Option<Sound>,
    pub wall_hit_sound: Option<Sound>,
    pub score_sound: Option<Sound>,
}

impl Game {
    pub async fn new() -> Self {
        // Load sounds asynchronously
        let paddle_hit_sound = load_sound_from_bytes(&generate_paddle_hit_sound()).await.ok();
        let wall_hit_sound = load_sound_from_bytes(&generate_wall_hit_sound()).await.ok();
        let score_sound = load_sound_from_bytes(&generate_score_sound()).await.ok();

        Self {
            player_paddle: Paddle::new(30.0, WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0),
            ai_paddle: Paddle::new(
                WINDOW_WIDTH - 30.0 - PADDLE_WIDTH,
                WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            ),
            ball: Ball::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0),
            player_score: 0,
            ai_score: 0,
            game_state: GameState::Playing,
            particles: Vec::new(),
            ball_trail: Vec::new(),
            paddle_hit_sound,
            wall_hit_sound,
            score_sound,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.game_state != GameState::Playing {
            return;
        }

        // Handle player input
        self.player_paddle.velocity = 0.0;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.player_paddle.velocity = -PADDLE_SPEED;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.player_paddle.velocity = PADDLE_SPEED;
        }

        // Simple AI for the computer paddle
        let ball_center_y = self.ball.position.y + BALL_SIZE / 2.0;
        let ai_center_y = self.ai_paddle.get_center_y();
        let ai_speed = PADDLE_SPEED * 0.8; // Make AI slightly slower for fairness

        if ball_center_y < ai_center_y - 10.0 {
            self.ai_paddle.velocity = -ai_speed;
        } else if ball_center_y > ai_center_y + 10.0 {
            self.ai_paddle.velocity = ai_speed;
        } else {
            self.ai_paddle.velocity = 0.0;
        }

        // Update game objects
        self.player_paddle.update(dt);
        self.ai_paddle.update(dt);

        // Check for wall collision before updating ball
        let old_ball_y = self.ball.position.y;
        self.ball.update(dt);

        // Play wall hit sound if ball bounced off top or bottom
        if (old_ball_y <= 0.0 || old_ball_y >= WINDOW_HEIGHT - BALL_SIZE) &&
           (self.ball.position.y <= 0.0 || self.ball.position.y >= WINDOW_HEIGHT - BALL_SIZE) {
            if let Some(sound) = &self.wall_hit_sound {
                play_sound_once(sound);
            }
        }

        // Handle paddle-ball collisions
        let old_ball_velocity = self.ball.velocity;
        self.ball.handle_paddle_collision(&self.player_paddle);
        self.ball.handle_paddle_collision(&self.ai_paddle);

        // Create particles on paddle hit
        if (self.ball.velocity.x > 0.0) != (old_ball_velocity.x > 0.0) {
            // Play paddle hit sound
            if let Some(sound) = &self.paddle_hit_sound {
                play_sound_once(sound);
            }

            for _ in 0..8 {
                self.particles.push(Particle::new(
                    self.ball.position.x + BALL_SIZE / 2.0,
                    self.ball.position.y + BALL_SIZE / 2.0,
                ));
            }
        }

        // Update particles
        self.particles.retain_mut(|particle| particle.update(dt));

        // Update ball trail
        self.ball_trail.push(Vec2D::new(
            self.ball.position.x + BALL_SIZE / 2.0,
            self.ball.position.y + BALL_SIZE / 2.0,
        ));
        if self.ball_trail.len() > 10 {
            self.ball_trail.remove(0);
        }

        // Check for scoring
        if self.ball.position.x < 0.0 {
            self.ai_score += 1;

            // Play score sound
            if let Some(sound) = &self.score_sound {
                play_sound_once(sound);
            }

            // Create score particles
            for _ in 0..15 {
                self.particles.push(Particle::new(
                    self.ball.position.x + BALL_SIZE / 2.0,
                    self.ball.position.y + BALL_SIZE / 2.0,
                ));
            }

            self.ball.reset();
            self.ball_trail.clear();
            if self.ai_score >= WINNING_SCORE {
                self.game_state = GameState::GameOver;
            }
        } else if self.ball.position.x > WINDOW_WIDTH {
            self.player_score += 1;

            // Play score sound
            if let Some(sound) = &self.score_sound {
                play_sound_once(sound);
            }

            // Create score particles
            for _ in 0..15 {
                self.particles.push(Particle::new(
                    self.ball.position.x + BALL_SIZE / 2.0,
                    self.ball.position.y + BALL_SIZE / 2.0,
                ));
            }

            self.ball.reset();
            self.ball_trail.clear();
            if self.player_score >= WINNING_SCORE {
                self.game_state = GameState::GameOver;
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        // Draw center line
        for i in 0..20 {
            let y = i as f32 * (WINDOW_HEIGHT / 20.0);
            if i % 2 == 0 {
                draw_rectangle(
                    WINDOW_WIDTH / 2.0 - 2.0,
                    y,
                    4.0,
                    WINDOW_HEIGHT / 20.0,
                    WHITE,
                );
            }
        }

        // Draw ball trail
        for (i, trail_pos) in self.ball_trail.iter().enumerate() {
            let alpha = (i as f32 / self.ball_trail.len() as f32) * 0.3;
            draw_circle(trail_pos.x, trail_pos.y, 3.0, Color::new(1.0, 1.0, 1.0, alpha));
        }

        // Draw game objects
        self.player_paddle.draw();
        self.ai_paddle.draw();
        self.ball.draw();

        // Draw particles
        for particle in &self.particles {
            particle.draw();
        }

        // Draw scores
        let font_size = 48.0;
        let player_score_text = format!("{}", self.player_score);
        let ai_score_text = format!("{}", self.ai_score);

        draw_text(
            &player_score_text,
            WINDOW_WIDTH / 4.0 - 20.0,
            80.0,
            font_size,
            WHITE,
        );
        draw_text(
            &ai_score_text,
            3.0 * WINDOW_WIDTH / 4.0 - 20.0,
            80.0,
            font_size,
            WHITE,
        );

        // Draw instructions
        if self.game_state == GameState::Playing {
            let instructions = "W/S or Up/Down arrows to move";
            draw_text(
                instructions,
                WINDOW_WIDTH / 2.0 - 150.0,
                WINDOW_HEIGHT - 30.0,
                20.0,
                GRAY,
            );
        }

        // Draw game over screen
        if self.game_state == GameState::GameOver {
            let winner = if self.player_score >= WINNING_SCORE {
                "PLAYER WINS!"
            } else {
                "AI WINS!"
            };

            draw_text(
                winner,
                WINDOW_WIDTH / 2.0 - 100.0,
                WINDOW_HEIGHT / 2.0 - 50.0,
                36.0,
                WHITE,
            );

            draw_text(
                "Press R to restart or ESC to quit",
                WINDOW_WIDTH / 2.0 - 150.0,
                WINDOW_HEIGHT / 2.0,
                20.0,
                GRAY,
            );
        }
    }

    pub fn handle_input(&mut self) {
        if self.game_state == GameState::GameOver {
            if is_key_pressed(KeyCode::R) {
                self.reset();
            }
        }
    }

    pub fn reset(&mut self) {
        self.player_score = 0;
        self.ai_score = 0;
        self.ball.reset();
        self.player_paddle.position.y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;
        self.ai_paddle.position.y = WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0;
        self.game_state = GameState::Playing;
        self.particles.clear();
        self.ball_trail.clear();
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Pong - Rust Edition".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}