// Integration tests to characterize current behavior before refactoring
// These tests freeze the existing functionality to ensure refactoring preserves all features

use pong::*;

#[cfg(test)]
mod tests {
    use super::*;

    mod math_utilities {
        use super::*;

        #[test]
        fn vec2d_new_creates_correct_coordinates() {
            let v = Vec2D::new(3.5, -2.1);
            assert_eq!(v.x, 3.5);
            assert_eq!(v.y, -2.1);
        }

        #[test]
        fn vec2d_zero_creates_origin() {
            let v = Vec2D::zero();
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
        }

        #[test]
        fn vec2d_length_calculates_magnitude() {
            let v = Vec2D::new(3.0, 4.0);
            assert_eq!(v.length(), 5.0);
        }

        #[test]
        fn vec2d_normalize_creates_unit_vector() {
            let v = Vec2D::new(3.0, 4.0);
            let normalized = v.normalize();
            assert!((normalized.length() - 1.0).abs() < f32::EPSILON);
            assert_eq!(normalized.x, 0.6);
            assert_eq!(normalized.y, 0.8);
        }

        #[test]
        fn vec2d_normalize_zero_vector_returns_zero() {
            let v = Vec2D::zero();
            let normalized = v.normalize();
            assert_eq!(normalized.x, 0.0);
            assert_eq!(normalized.y, 0.0);
        }
    }

    mod game_constants {
        use super::*;

        #[test]
        fn window_dimensions_are_correct() {
            assert_eq!(WINDOW_WIDTH, 800.0);
            assert_eq!(WINDOW_HEIGHT, 600.0);
        }

        #[test]
        fn paddle_dimensions_are_correct() {
            assert_eq!(PADDLE_WIDTH, 15.0);
            assert_eq!(PADDLE_HEIGHT, 80.0);
            assert_eq!(PADDLE_SPEED, 300.0);
        }

        #[test]
        fn ball_specifications_are_correct() {
            assert_eq!(BALL_SIZE, 15.0);
            assert_eq!(BALL_SPEED, 350.0);
        }

        #[test]
        fn game_rules_are_correct() {
            assert_eq!(WINNING_SCORE, 5);
        }
    }

    mod particle_system {
        use super::*;

        #[test]
        fn particle_new_creates_with_position() {
            let particle = Particle::new(100.0, 200.0);
            assert_eq!(particle.position.x, 100.0);
            assert_eq!(particle.position.y, 200.0);
        }

        #[test]
        fn particle_has_initial_velocity_and_lifetime() {
            let particle = Particle::new(0.0, 0.0);
            // Velocity should be randomized but within reasonable bounds
            assert!(particle.velocity.length() > 0.0);
            assert!(particle.velocity.length() < 1000.0);
            // Lifetime should be positive
            assert!(particle.lifetime > 0.0);
            assert!(particle.lifetime <= 2.0); // Max lifetime from code
        }

        #[test]
        fn particle_update_decreases_lifetime() {
            let mut particle = Particle::new(0.0, 0.0);
            let initial_lifetime = particle.lifetime;
            let dt = 0.016; // ~60 FPS

            let still_alive = particle.update(dt);

            assert!(particle.lifetime < initial_lifetime);
            assert_eq!(still_alive, particle.lifetime > 0.0);
        }

        #[test]
        fn particle_update_moves_position() {
            let mut particle = Particle::new(100.0, 200.0);
            let initial_pos = particle.position;
            let dt = 0.016;

            particle.update(dt);

            // Position should have changed based on velocity
            let position_changed = particle.position.x != initial_pos.x || particle.position.y != initial_pos.y;
            assert!(position_changed);
        }
    }

    mod paddle_behavior {
        use super::*;

        #[test]
        fn paddle_new_creates_at_position() {
            let paddle = Paddle::new(50.0, 100.0);
            assert_eq!(paddle.position.x, 50.0);
            assert_eq!(paddle.position.y, 100.0);
            assert_eq!(paddle.velocity, 0.0);
        }

        #[test]
        fn paddle_get_center_y_calculates_correctly() {
            let paddle = Paddle::new(0.0, 100.0);
            let center_y = paddle.get_center_y();
            assert_eq!(center_y, 100.0 + PADDLE_HEIGHT / 2.0);
        }

        #[test]
        fn paddle_get_rect_returns_correct_bounds() {
            let paddle = Paddle::new(50.0, 100.0);
            let rect = paddle.get_rect();
            assert_eq!(rect.x, 50.0);
            assert_eq!(rect.y, 100.0);
            assert_eq!(rect.w, PADDLE_WIDTH);
            assert_eq!(rect.h, PADDLE_HEIGHT);
        }

        #[test]
        fn paddle_update_moves_position() {
            let mut paddle = Paddle::new(0.0, 100.0);
            paddle.velocity = 200.0; // Moving down
            let dt = 0.1; // 100ms

            paddle.update(dt);

            assert_eq!(paddle.position.y, 100.0 + 200.0 * 0.1);
        }

        #[test]
        fn paddle_update_clamps_to_window_bounds() {
            let mut paddle = Paddle::new(0.0, -10.0); // Above window
            paddle.velocity = -100.0; // Moving up

            paddle.update(0.1);

            assert_eq!(paddle.position.y, 0.0); // Should be clamped to top

            // Test bottom bound
            paddle.position.y = WINDOW_HEIGHT + 10.0; // Below window
            paddle.velocity = 100.0; // Moving down

            paddle.update(0.1);

            assert_eq!(paddle.position.y, WINDOW_HEIGHT - PADDLE_HEIGHT); // Should be clamped to bottom
        }
    }

    mod ball_physics {
        use super::*;

        #[test]
        fn ball_new_creates_at_position() {
            let ball = Ball::new(400.0, 300.0);
            assert_eq!(ball.position.x, 400.0);
            assert_eq!(ball.position.y, 300.0);
        }

        #[test]
        fn ball_new_has_initial_velocity() {
            let ball = Ball::new(0.0, 0.0);
            // Ball should have some initial velocity
            assert!(ball.velocity.length() > 0.0);
            // Velocity should be normalized to BALL_SPEED
            assert!((ball.velocity.length() - BALL_SPEED).abs() < 1.0);
        }

        #[test]
        fn ball_get_rect_returns_correct_bounds() {
            let ball = Ball::new(100.0, 200.0);
            let rect = ball.get_rect();
            assert_eq!(rect.x, 100.0);
            assert_eq!(rect.y, 200.0);
            assert_eq!(rect.w, BALL_SIZE);
            assert_eq!(rect.h, BALL_SIZE);
        }

        #[test]
        fn ball_update_moves_position() {
            let mut ball = Ball::new(100.0, 200.0);
            let initial_velocity = ball.velocity;
            let dt = 0.016;

            ball.update(dt);

            let expected_x = 100.0 + initial_velocity.x * dt;
            let expected_y = 200.0 + initial_velocity.y * dt;

            assert!((ball.position.x - expected_x).abs() < 0.1);
            assert!((ball.position.y - expected_y).abs() < 0.1);
        }

        #[test]
        fn ball_bounces_off_top_wall() {
            let mut ball = Ball::new(100.0, -5.0); // Above window
            ball.velocity = Vec2D::new(100.0, -200.0); // Moving up

            ball.update(0.016);

            assert!(ball.velocity.y > 0.0); // Should be moving down now
            assert_eq!(ball.position.y, 0.0); // Should be at top boundary
        }

        #[test]
        fn ball_bounces_off_bottom_wall() {
            let mut ball = Ball::new(100.0, WINDOW_HEIGHT + 5.0); // Below window
            ball.velocity = Vec2D::new(100.0, 200.0); // Moving down

            ball.update(0.016);

            assert!(ball.velocity.y < 0.0); // Should be moving up now
            assert_eq!(ball.position.y, WINDOW_HEIGHT - BALL_SIZE); // Should be at bottom boundary
        }

        #[test]
        fn ball_reset_centers_position() {
            let mut ball = Ball::new(100.0, 200.0);

            ball.reset();

            assert_eq!(ball.position.x, WINDOW_WIDTH / 2.0);
            assert_eq!(ball.position.y, WINDOW_HEIGHT / 2.0);
            // Should have new random velocity
            assert!(ball.velocity.length() > 0.0);
        }
    }

    mod collision_detection {
        use super::*;

        #[test]
        fn ball_paddle_collision_detection() {
            let paddle = Paddle::new(100.0, 200.0);
            let mut ball = Ball::new(90.0, 220.0); // Positioned to collide
            ball.velocity = Vec2D::new(-200.0, 0.0); // Moving towards paddle

            let initial_velocity_x = ball.velocity.x;
            ball.handle_paddle_collision(&paddle);

            // Velocity X should be reversed if collision occurred
            if ball.get_rect().overlaps(&paddle.get_rect()) {
                assert!(ball.velocity.x != initial_velocity_x);
            }
        }

        #[test]
        fn ball_collision_changes_angle_based_on_paddle_intersection() {
            let paddle = Paddle::new(100.0, 200.0);
            let mut ball = Ball::new(90.0, 200.0); // Hit paddle at top
            ball.velocity = Vec2D::new(-200.0, 0.0);

            ball.handle_paddle_collision(&paddle);

            if ball.get_rect().overlaps(&paddle.get_rect()) {
                // Ball should have some Y velocity component based on intersection point
                assert!(ball.velocity.y != 0.0);
            }
        }

        #[test]
        fn ball_moves_away_from_paddle_after_collision() {
            let paddle = Paddle::new(100.0, 200.0);
            let mut ball = Ball::new(105.0, 220.0); // Inside paddle
            ball.velocity = Vec2D::new(-200.0, 0.0);

            ball.handle_paddle_collision(&paddle);

            let ball_rect = ball.get_rect();
            let paddle_rect = paddle.get_rect();

            // Ball should be moved outside paddle bounds
            assert!(!ball_rect.overlaps(&paddle_rect) || ball.velocity.x > 0.0);
        }
    }

    mod audio_generation {
        use super::*;

        #[test]
        fn generate_paddle_hit_sound_creates_wav_data() {
            let sound_data = generate_paddle_hit_sound();

            // Should have WAV header (44 bytes) plus audio data
            assert!(sound_data.len() > 44);

            // Check WAV header signature
            assert_eq!(&sound_data[0..4], b"RIFF");
            assert_eq!(&sound_data[8..12], b"WAVE");
            assert_eq!(&sound_data[12..16], b"fmt ");
        }

        #[test]
        fn generate_wall_hit_sound_creates_wav_data() {
            let sound_data = generate_wall_hit_sound();

            // Should have WAV header plus audio data
            assert!(sound_data.len() > 44);

            // Check WAV header signature
            assert_eq!(&sound_data[0..4], b"RIFF");
            assert_eq!(&sound_data[8..12], b"WAVE");
        }

        #[test]
        fn generate_score_sound_creates_wav_data() {
            let sound_data = generate_score_sound();

            // Should have WAV header plus audio data
            assert!(sound_data.len() > 44);

            // Check WAV header signature
            assert_eq!(&sound_data[0..4], b"RIFF");
            assert_eq!(&sound_data[8..12], b"WAVE");
        }

        #[test]
        fn wav_header_has_correct_format() {
            let sound_data = generate_paddle_hit_sound();

            // PCM format (16-bit, mono, 44100 Hz)
            let format = u16::from_le_bytes([sound_data[20], sound_data[21]]);
            assert_eq!(format, 1); // PCM format

            let channels = u16::from_le_bytes([sound_data[22], sound_data[23]]);
            assert_eq!(channels, 1); // Mono

            let sample_rate = u32::from_le_bytes([sound_data[24], sound_data[25], sound_data[26], sound_data[27]]);
            assert_eq!(sample_rate, 44100); // 44.1 kHz
        }
    }
}