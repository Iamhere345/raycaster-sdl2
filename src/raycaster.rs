use crate::graphics::*;
use nalgebra::{Vector2, Rotation2};

const MAP_HEIGHT: usize = 24;
const MAP_WIDTH: usize = 24;

const MOVE_SPEED: f64 = 5.0;
const ROT_SPEED: f64 = 3.0;

const MAP: [[u32; MAP_WIDTH]; MAP_HEIGHT] =
[
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,2,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,3,0,0,0,3,0,0,0,1],
  [1,0,0,0,0,0,2,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,2,2,0,2,2,0,0,0,0,3,0,3,0,3,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,4,4,4,4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
  [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]
];

#[derive(PartialEq, Clone, Copy)]
enum SideHit {
    XSide,
    YSize
}
#[derive(Debug)]
pub struct Player {
    pos: Vector2<f64>,
    dir: Vector2<f64>,
}

impl Player {
    pub fn plane(&self) -> Vector2<f64> {
        Vector2::new(-self.dir.y, self.dir.x)
    }
}

pub struct Scene {
    plr: Player
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            plr: Player {
                pos: Vector2::new(22.0, 12.0),
                dir: Vector2::new(-1.0, 0.0),
            }
        }
    }
}



pub fn update(screen: &mut Screen, scene: &mut Scene) {

    let mut player = &scene.plr;

    for x in 0..SCREEN_WIDTH {

        // screen x coord in camera space, where the right side gets 1, the centre get 0 and the left gets -1
        let camera_x = 2.0 * (x as f64) / (SCREEN_WIDTH as f64) - 1.0;

        // any ray point on the camera plan can be calculated with the sum of the direction vector
        // and a part of the plane vector, multiplied by the x position on the camera plane.
        let ray_dir = player.dir + player.plane() * camera_x;

        // which coordinate on the map the ray is in
        let mut map_pos = Vector2::new(player.pos.x.floor() as i32, player.pos.y.floor() as i32);

        // length of the ray from the current position to the next x/y side
        let mut side_dist = Vector2::new(0.0, 0.0);

        // length of the ray from one x/y side to the next x/y side
        let delta_dist = Vector2::new(
            if ray_dir.x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.x).abs() },
            if ray_dir.y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.y).abs() }
        );

        let mut wall_dist = 0.0;

        let mut ray_step = Vector2::new(0, 0);

        let mut hit_info: Option<SideHit> = None;

        // calculate ray_step and side_dist
        if ray_dir.x < 0.0 {
            ray_step.x = -1;
            side_dist.x = (player.pos.x - map_pos.x as f64) * delta_dist.x;
        } else {
            ray_step.x = 1;
            side_dist.x = (map_pos.x as f64 + 1.0 - player.pos.x) * delta_dist.x;
        }
        if ray_dir.y < 0.0 {
            ray_step.y = -1;
            side_dist.y = (player.pos.y - map_pos.y as f64) * delta_dist.y;
        } else {
            ray_step.y = 1;
            side_dist.y = (map_pos.y as f64 + 1.0 - player.pos.y) * delta_dist.y;
        }

        let mut colour: CanvasColour = CanvasColour::WHITE;

        // DDA algorithm
        while hit_info.is_none() {

            let mut side: SideHit = SideHit::XSide;

            // jump to the next square, either in the x direction or the y direction
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += ray_step.x;
                
                side = SideHit::XSide;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += ray_step.y;

                side = SideHit::YSize;
            }

            let coord = MAP[map_pos.x as usize][map_pos.y as usize];

            if coord > 0 {
                hit_info = Some(side);

                match side {
                    SideHit::XSide => {
                        wall_dist = side_dist.x - delta_dist.x;
                    },
                    SideHit::YSize => {
                        wall_dist = side_dist.y - delta_dist.y
                    }
                }

                colour = match coord {
                    1 => CanvasColour::RED,
                    2 => CanvasColour::GREEN,
                    3 => CanvasColour::BLUE,
                    4 => CanvasColour::TEAL,
                    _ => CanvasColour::WHITE
                };

                if side == SideHit::YSize {
                    colour = colour / 2;
                }

            }

        }

        let line_height: i32 = (SCREEN_HEIGHT as f64 / wall_dist) as i32;

        //println!("{}", line_height);

        let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_start < 0 { draw_start = 0; }

        let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_end < 0 { draw_end = 0; }

        draw_line(screen, x as i32, draw_start, draw_end, colour);

    }
}

pub fn input(key: InputKeycode, delta_time: f64, scene: &mut Scene) {

    let move_speed = (MOVE_SPEED * delta_time) + 1.0;
    let rot_speed = (ROT_SPEED * delta_time) + 1.0;

    //println!("move {} rot {}", move_speed, rot_speed);

    let player = &mut scene.plr;

    //delta_time = 1.0;

    let mut wish_pos = player.pos;

    match key {
        InputKeycode::W => {

            wish_pos += player.dir * move_speed;

        },
        InputKeycode::S => {

            wish_pos -= player.dir * move_speed;

        },
        // rotate to the right
        InputKeycode::D => {

            let rot = Rotation2::new(0.1);
            player.dir = rot * player.dir;

        },
        // rotate to the left
        InputKeycode::A => {

            let rot = Rotation2::new(-0.1);
            player.dir = rot * player.dir;

        },
        _ => {}
    }

    if MAP[wish_pos.x as usize][wish_pos.y as usize] == 0 {
        player.pos = wish_pos;
    }

    //println!("{:?}", player.pos);

}