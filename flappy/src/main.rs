use bracket_lib::prelude::*;
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FPS: f32 = 45.;
const FRAME_DURATION: f32 = 1.0 / FPS * 1000.; // milliseconds。 60fps
const STACLE_DURATION: f32 = 1000.; // milliseconds
enum GameMode{
    Menu,
    Playing,
    End,
}
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        let size = i32::max(2, 20-score);
        let gap_y = random.range(10, 40);
        Self {
            x,
            gap_y,
            size,
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = self.x == player.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}


struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x+=1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct State {
    player: Player,
    frame_time: f32,
    obstacle_time: f32,
    obstacles: Vec<Obstacle>,
    score: i32,
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle_time: 0.0,
            obstacles: Vec::from([Obstacle::new(SCREEN_WIDTH, 0)]),
            score: 0,
            mode: GameMode::Menu,
        }
    }
    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        self.obstacle_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score: {}", self.score));
        for obs in self.obstacles.iter_mut() {
            obs.render(ctx, self.player.x);
            if self.player.x == obs.x + 1 {
                self.score += 1;
            }
            if self.player.y > SCREEN_HEIGHT || obs.hit_obstacle(&self.player) {
                self.mode = GameMode::End;
            }
        }
        // 移除 x 值小于 0 的障碍物
        self.obstacles.retain(|obs| obs.x >= 0);
        if self.obstacle_time > STACLE_DURATION {
            self.obstacle_time = 0.0;
            self.obstacles.push(Obstacle::new(self.player.x + SCREEN_WIDTH, self.score));
        }

    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, format!("You scored: {}", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quit Game");
        
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle_time = 0.0;
        self.obstacles = Vec::from([Obstacle::new(SCREEN_WIDTH, 0)]);
        self.score = 0;
        self.mode = GameMode::Playing;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode  {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello world!")
        .build()?;
    main_loop(context, State::new())
}
