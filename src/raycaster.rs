use crate::graphics::*;
use crate::vec2::Vec2;

const MAP_HEIGHT: usize = 24;
const MAP_WIDTH: usize = 24;

const MOVE_SPEED: f64 = 5.0;
const ROT_SPEED: f64 = 3.0;

// TODO use nalgebra instead of vec2.rs

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
    pos: Vec2<f64>,
    dir: Vec2<f64>,
    plane: Vec2<f64>
}

pub struct Scene {
    plr: Player
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            plr: Player {
                pos: Vec2::<f64>::new(22.0, 12.0),
                dir: Vec2::<f64>::new(-1.0, 0.0),
                plane: Vec2::<f64>::new(0.0, 0.66)
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
        let ray_dir = Vec2::<f64>::new(
            player.dir.x + player.plane.x * camera_x,
            player.dir.y + player.plane.y * camera_x
        );

        // which coordinate on the map the ray is in
        let mut map_pos = Vec2::<i32>::new(player.pos.x.floor() as i32, player.pos.y.floor() as i32);

        // length of the ray from the current position to the next x/y side
        let mut side_dist = Vec2::<f64>::new(0.0, 0.0);

        // length of the ray from one x/y side to the next x/y side
        let delta_dist = Vec2::<f64>::new(
            if ray_dir.x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.x).abs() },
            if ray_dir.y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir.y).abs() }
        );

        let mut wall_dist = 0.0;

        let mut ray_step = Vec2::<i32>::new(0, 0);

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

pub fn input(key: InputKeycode, mut delta_time: f64, scene: &mut Scene) {

    let move_speed = (MOVE_SPEED * delta_time) + 1.0;
    let rot_speed = (ROT_SPEED * delta_time) + 1.0;

    //println!("move {} rot {}", move_speed, rot_speed);

    let player = &mut scene.plr;

    delta_time = 1.0;

    match key {
        InputKeycode::W => {

            // TODO this is also wrong
            let wish_pos = Vec2::<f64>::new((player.pos.x + player.dir.x) * move_speed, (player.pos.y + player.dir.y) * move_speed);

            if wish_pos.x <= MAP_WIDTH as f64 && MAP[wish_pos.x as usize][player.pos.y as usize] == 0 {
                player.pos.x = wish_pos.x;
                println!("x");
            } else {
                println!("nx");
            }

            if wish_pos.y <= MAP_HEIGHT as f64 && MAP[player.pos.x as usize][wish_pos.y as usize] == 0 {
                player.pos.y = wish_pos.y;
                println!("y");
            } else {
                println!("ny");
            }

        },
        InputKeycode::S => {

            let wish_pos = Vec2::<f64>::new((player.pos.x - player.dir.x) * move_speed, (player.pos.y - player.dir.y) * move_speed);

            if wish_pos.x <= MAP_WIDTH as f64 && MAP[wish_pos.x as usize][player.pos.y as usize] == 0 {
                player.pos.x = wish_pos.x;
                println!("x");
            } else {
                println!("nx");
            }

            if wish_pos.y <= MAP_HEIGHT as f64 && MAP[player.pos.x as usize][wish_pos.y as usize] == 0 {
                player.pos.y = wish_pos.y;
                println!("y");
            } else {
                println!("ny");
            }

        },
        // rotate to the right
        // TODO this seems to just be changing the sign of dir
        // TODO the camera plane isn't perpendicular to the player
        InputKeycode::D => {

            // rotate player dir
            let old_dir_x = player.dir.x;
            
            player.dir.x = player.dir.x * -rot_speed.cos() - player.dir.y * rot_speed.sin();
            player.dir.y = old_dir_x * rot_speed.sin() + player.dir.y * -rot_speed.cos();

            // rotate camera plane
            let old_plane_x = player.plane.x;

            player.plane.x = player.plane.x * -rot_speed.cos() - player.plane.y * rot_speed.sin();
            player.plane.y = old_plane_x * rot_speed.sin() - player.plane.y * -rot_speed.cos();

        },
        // rotate to the left
        InputKeycode::A => {

            // rotate player dir
            let old_dir_x = player.dir.x;
            
            //player.dir.x = player.dir.x * rot_speed.cos() - player.dir.y * -rot_speed.sin();
            //player.dir.y = old_dir_x * -rot_speed.sin() + player.dir.y * rot_speed.cos();

            // rotate camera plane
            let old_plane_x = player.plane.x;

            player.plane.x = player.plane.x * rot_speed.cos() - player.plane.y * rot_speed.sin();
            player.plane.y = old_plane_x * rot_speed.sin() - player.plane.y * rot_speed.cos();

        },
        _ => {}
    }

    //println!("{:?}", player.pos);

}