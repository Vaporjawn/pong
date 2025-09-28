# System Patterns: Rust Pong Game Architecture

## Code Organization Patterns

### Entity-Component Structure
- **Game struct**: Central game state manager
- **Paddle struct**: Player and AI paddle entities
- **Ball struct**: Ball physics and movement
- **Particle struct**: Visual effects system
- **Vec2D struct**: Custom 2D vector mathematics

### State Management Pattern
```rust
enum GameState {
    Playing,
    GameOver,
}
```
Clean state machine for game phases with proper transitions.

### Physics Engine Architecture
- **Collision Detection**: Rectangle-based overlap detection
- **Velocity System**: Proper physics with delta time
- **Reflection Algorithm**: Angle-based ball bouncing with paddle intersection calculation

### Memory Management
- **Vec collections**: Dynamic particle and trail systems
- **Stack allocation**: All game objects use stack memory
- **No unsafe code**: Pure safe Rust implementation

## Advanced Features Implemented

### Visual Polish
1. **Particle Effects**: Collision and scoring particles with lifetime management
2. **Ball Trail**: Motion blur effect with position history
3. **Paddle Glow**: Visual feedback for active paddles
4. **Dotted Center Line**: Classic Pong court appearance

### Gameplay Systems
1. **AI Intelligence**: Computer opponent with strategic positioning
2. **Progressive Difficulty**: AI speed slightly reduced for balance
3. **Angle Reflection**: Realistic physics based on paddle intersection point
4. **Winning Condition**: First to 10 points with proper game over state

### Input Handling
- **Multiple Controls**: W/S keys OR arrow keys
- **Smooth Movement**: Velocity-based paddle control
- **Game State Input**: R for reset, ESC for quit

## Performance Optimizations
- **Efficient Rendering**: Minimal draw calls
- **Proper Game Loop**: 60 FPS with frame timing
- **Memory Efficiency**: Vec cleanup for particles/trails
- **Collision Optimization**: Early bounds checking