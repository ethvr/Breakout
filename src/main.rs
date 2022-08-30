use macroquad::prelude::*;
use std::process;

const HEIGHT:f32 = 600.0;
const WIDTH:f32 = 800.0;
const PLAYER:[f32; 4] = [340.0,500.0,120.0,30.0];
const BLOCK:[f32; 2] = [100.0,40.0];
const BLOCKS_X:f32 = 6.0;
const BLOCKS_Y:f32 = 5.0;
const BLOCKSIZE: (f32,f32) = (BLOCK[0] + 10.0,BLOCK[1]+ 10.0);
const BOARDSIZE: (f32,f32) = (BLOCKS_X * BLOCKSIZE.0,BLOCKS_Y * BLOCKSIZE.1);
const BALL:[f32;3] = [30.0,30.0,200.0];

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: 50u16,
            color: WHITE,
            ..Default::default()
        },
    );
}

enum GameState {
    Menu,
    Playing,
    Dead,
    Won,
}

#[derive(Debug)]
struct Player {
    rect: Rect,
    c: Color,
    lives: u32,
}
impl Player{
    pub fn new() -> Self {
        Self {
            rect: Rect::new(PLAYER[0],PLAYER[1],PLAYER[2],PLAYER[3]),
            c: YELLOW,
            lives: 3,        
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.c);
    }   
    pub fn update(&mut self,dt:f32){
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * 400.0;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > WIDTH - self.rect.w {
            self.rect.x = WIDTH - self.rect.w;
        }
    }
}

struct Ball {
    rect: Rect,
    c: Color,
    vec: (f32,f32),
}

impl Ball {
    pub fn new(ballposx:f32,ballposy:f32) -> Self {
        Self {
            rect: Rect::new(ballposx,ballposy,BALL[0],BALL[1]),
            c: WHITE,
            vec: (-1f32,1f32), 
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x,self.rect.y,self.rect.w,self.rect.h, self.c);
    }
    pub fn update(&mut self , dt:f32) {
        self.rect.x += dt * self.vec.0 * BALL[2];
        self.rect.y += dt * self.vec.1 * BALL[2];
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut (f32,f32), b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.1 = -to_signum.y * vel.1.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.0 = -to_signum.x * vel.0.abs();
        }
    }
    true
}

#[allow(dead_code)]
struct Block {
    rect: Rect,
    col: Color,
    lives: u32,
}
impl Block {
    pub fn new(pos_x:f32,pos_y:f32) -> Self{
        Self {
            rect: Rect::new(pos_x,pos_y,BLOCK[0],BLOCK[1]),
            col: RED,
            lives: 2
        }
    }
    pub fn draw(&self) {
        let col = match self.lives {
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, col);
    }
}

fn init_blocks() -> Vec<Block> {
    let mut blocks:Vec<Block> = Vec::new();
    let init_pos_x = (WIDTH - BOARDSIZE.0)/2.0;
    let init_pos_y = 60.0;    

    for i in 0..(BLOCKS_X*BLOCKS_Y) as u32 {
        let posx = init_pos_x + i as f32 % BLOCKS_X * BLOCKSIZE.0;
        let posy = init_pos_y + i as f32 % BLOCKS_Y * BLOCKSIZE.1;
        let obstacle = Block::new(posx,posy);
        blocks.push(obstacle);
    }
    blocks
}

fn walls(bl:&mut Ball) {
    let l = Rect::new(-5f32,0f32,5f32,HEIGHT);
    draw_rectangle(l.x,l.y,l.w,l.h,BLACK);              //LEFT WALL
    resolve_collision(&mut bl.rect, &mut bl.vec, &l);

    let t = Rect::new(0f32,-5f32,WIDTH,5f32);
    draw_rectangle(t.x,t.y,t.w,t.h,BLACK);              // TOP WALL
    resolve_collision(&mut bl.rect, &mut bl.vec, &t);

    let r = Rect::new(WIDTH,0f32,5f32,HEIGHT);
    draw_rectangle(r.x,r.y,r.w,r.h,BLACK);              // RIGHT WALL
    resolve_collision(&mut bl.rect, &mut bl.vec, &r);
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("src/rs/Cairo-Regular.ttf").await.unwrap();
    let mut tom: Player = Player::new();
    let mut family:Vec<Block> = init_blocks();
    let mut bl = Ball::new(400f32,400f32);
    let mut score: u32 = 0;
    let mut game: GameState = GameState::Menu;
    loop {
        match game {
            GameState::Menu => {
                clear_background(BLACK);
                draw_title_text("Press SPACE to start",font);
                if is_key_down(KeyCode::Space) {
                    game = GameState::Playing;
                }
            }
            GameState::Playing => {
                clear_background(BLACK);
                walls(&mut bl);
                draw_text_ex(
                    &format!("Score: {}",score),
                    400f32,
                    50f32,
                    TextParams {
                        font,
                        font_size: 50u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
                draw_text_ex(
                    &format!("Lives: {}",tom.lives),
                    200f32,
                    50f32,
                    TextParams {
                        font,
                        font_size: 50u16,
                        color: WHITE,
                        ..Default::default()
                    },
                );
                resolve_collision(&mut bl.rect, &mut bl.vec, &tom.rect);
                bl.draw();
                bl.update(get_frame_time() as f32);
                family.retain(|block| block.lives != 0);  
                for block in family.iter_mut() {
                    if resolve_collision(&mut bl.rect, &mut bl.vec, &block.rect){
                        if block.lives == 2 {
                            block.lives = 1;
                        } else if block.lives == 1 {
                            block.lives = 0
                        }
                    }
                    if block.lives == 0 {
                        score += 10;
                    }
                    block.draw();
                }
                if family.len() == 0 {
                    game = GameState::Won;
                }
                if bl.rect.y > HEIGHT {
                    if tom.lives == 3 {
                        tom.lives = 2;
                    } else if tom.lives == 2 {
                        tom.lives = 1;
                    }else if tom.lives == 1 {
                        tom.lives = 0;
                    }else if tom.lives == 0 {
                        game = GameState::Dead;
                    }
                    println!("{}",tom.lives);
                    bl.rect.x = 400f32;
                    bl.rect.y = 400f32;
                }
                tom.update(get_frame_time());
                tom.draw();
            },
            GameState::Dead => {
                clear_background(BLACK);
                draw_title_text(&format!("you DIED! {} score",score),font);
                if is_key_down(KeyCode::Space) {
                    process::exit(1);
                }
            },
            GameState::Won => {
                clear_background(BLACK);
                draw_title_text(
                &format!("you WON! {} score",score),font);
                if is_key_down(KeyCode::Space) {
                    process::exit(1);
                }
            }
        };
        next_frame().await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let t = Ball::new(400f32,400f32);

        assert_eq!(t.c,WHITE);
        assert_eq!(t.vec,(-1f32,1f32));
    }

    #[test]
    fn test2() {
        let q = Player::new();

        assert_eq!(q.lives,3);
        assert_eq!(q.c,YELLOW);
    }

}
