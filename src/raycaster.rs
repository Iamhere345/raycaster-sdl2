use std::collections::HashSet;

use crate::graphics::*;
use nalgebra::{Vector2, Rotation2};

const MAP_HEIGHT: usize = 24;
const MAP_WIDTH: usize = 24;

const TEX_HEIGHT: u32 = 64;
const TEX_WIDTH: u32 = 64;

const MOVE_SPEED: f64 = 5.0;
const ROT_SPEED: f64 = 3.0;

/*
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
*/

const MAP: [[u32; MAP_WIDTH]; MAP_HEIGHT] =
[
  [4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,7,7,7,7,7,7,7,7],
  [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
  [4,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
  [4,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
  [4,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
  [4,0,4,0,0,0,0,5,5,5,5,5,5,5,5,5,7,7,0,7,7,7,7,7],
  [4,0,5,0,0,0,0,5,0,5,0,5,0,5,0,5,7,0,0,0,7,7,7,1],
  [4,0,6,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
  [4,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,1],
  [4,0,8,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
  [4,0,0,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,7,7,7,1],
  [4,0,0,0,0,0,0,5,5,5,5,0,5,5,5,5,7,7,7,7,7,7,7,1],
  [6,6,6,6,6,6,6,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
  [8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
  [6,6,6,6,6,6,0,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
  [4,4,4,4,4,4,0,4,4,4,6,0,6,2,2,2,2,2,2,2,3,3,3,3],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,0,0,0,6,2,0,0,5,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
  [4,0,6,0,6,0,0,0,0,4,6,0,0,0,0,0,5,0,0,0,0,0,0,2],
  [4,0,0,5,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
  [4,0,6,0,6,0,0,0,0,4,6,0,6,2,0,0,5,0,0,2,0,0,0,2],
  [4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
  [4,4,4,4,4,4,4,4,4,4,1,1,1,2,2,2,2,2,2,3,3,3,3,3]
];

#[derive(PartialEq, Clone, Copy)]
enum SideHit {
    XSide,
    YSide
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
    plr: Player,
    textures: [Vec<CanvasColour>; 8]
}

impl Scene {
    pub fn init() -> Scene {

        let plr = Player {
            pos: Vector2::new(22.0, 12.0),
            dir: Vector2::new(-1.0, 0.0),
        };


		let mut textures: [Vec<CanvasColour>; 8] = [
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
			Vec::with_capacity((TEX_WIDTH * TEX_HEIGHT) as usize),
		];

		// :(
		unsafe {
			for mut vec in textures.iter_mut() {
				vec.set_len((TEX_WIDTH * TEX_HEIGHT) as usize);
			}
		}

        for x in 0..TEX_WIDTH {
            for y in 0..TEX_HEIGHT {
				
				let xor_colour: u32 = (x * 256 / TEX_WIDTH) ^ (y * 256 / TEX_HEIGHT);
				let y_colour: u32 = y * 256 / TEX_HEIGHT;
				let xy_colour: u32 = y * 128 / TEX_HEIGHT + x * 128 / TEX_WIDTH;

				// 65535 == 0xFFFF (i.e the maximum 16 bit value)
				textures[0][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(65536 * 254 * (x != y && x != TEX_WIDTH - y) as u32);	// flat red texture with black cross
				textures[1][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(xy_colour + 256 * xy_colour + 65536 * xy_colour);		// sloped greyscale
				textures[2][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(256 * xy_colour + 65536 * xy_colour);					// sloped yellow gradient
				textures[3][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(xor_colour + 256 * xor_colour + 65536 * xor_colour);		// xor greyscale
				textures[4][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(256 * xor_colour);										// xor green
				textures[5][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(65536 * 192 * (x % 16 != 0 && y % 16 != 0) as u32);		// red bricks
				textures[6][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(65536 * y_colour);										// red gradient
				textures[7][(TEX_WIDTH * x + y) as usize] = CanvasColour::from_u32(128 + 256 * 128 + 65536 * 128)							// flat grey

            }
        }

        Scene {
			plr: plr,
			textures: textures
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
        let mut side: SideHit = SideHit::XSide;

        // DDA algorithm
        while hit_info.is_none() {

            // jump to the next square, either in the x direction or the y direction
            if side_dist.x < side_dist.y {
                side_dist.x += delta_dist.x;
                map_pos.x += ray_step.x;
                
                side = SideHit::XSide;
            } else {
                side_dist.y += delta_dist.y;
                map_pos.y += ray_step.y;

                side = SideHit::YSide;
            }

            let coord = MAP[map_pos.x as usize][map_pos.y as usize];

            if coord > 0 {
                hit_info = Some(side);

                match side {
                    SideHit::XSide => {
                        wall_dist = side_dist.x - delta_dist.x;
                    },
                    SideHit::YSide => {
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

                if side == SideHit::YSide {
                    colour = colour / 2;
                }

            }

        }

        let line_height: i32 = ((SCREEN_HEIGHT as f64 / wall_dist) as i32).clamp(0, SCREEN_HEIGHT as i32);

        //println!("{}", line_height);

        //println!("lh: {line_height}");

        let mut draw_start = -line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_start < 0 { draw_start = 0; }

        let mut draw_end = line_height / 2 + SCREEN_HEIGHT as i32 / 2;
        if draw_end < 0 { draw_end = 0; }

        //println!("de: {draw_end}");

        if draw_end > SCREEN_HEIGHT as i32 {
            println!("lh: {line_height} de: {draw_end}");
            println!("let line_height: i32 = ({SCREEN_HEIGHT} as f64 / {wall_dist} as i32");
        }

		let tex_index = MAP[map_pos.x as usize][map_pos.y as usize] - 1;	// -1 because on the map 0 is empty but the index 0 is used for textures

		// x position of where the wall was hit
		// FIXME
		let mut wall_x = if side == SideHit::YSide {
			player.pos.y + wall_dist * ray_dir.y
		} else {
			player.pos.x + wall_dist * ray_dir.x
		};

		//println!("wall_x: {wall_x} floor: {}", wall_x.floor());

		wall_x -= wall_x.floor();

		//println!("wall_X: {}", wall_x);

		// x coord on the texture
		let mut tex_x = (wall_x * TEX_WIDTH as f64) as i32;
		if side == SideHit::XSide && ray_dir.x > 0.0 { tex_x = TEX_WIDTH as i32 - tex_x - 1; } //else { println!("no XSide"); }
		if side == SideHit::YSide && ray_dir.y < 0.0 { tex_x = TEX_WIDTH as i32 - tex_x - 1; } //else { println!("no YSide") }
		//println!("tex_x: {tex_x}");

		/*
		affine texture mapping
		*/

		// how much to increase the texture coord per pixel
		let tex_step = 1.0 * TEX_HEIGHT as f64 / line_height as f64;
		// starting texture coordinate
		let mut tex_coord = (draw_start - SCREEN_HEIGHT as i32 / 2 + line_height / 2) as f64 * tex_step;

        // TODO i think the issue is with line height or draw_end being way too high (maybe from an overflow)

        ///*
		for y in draw_start..draw_end {

			let tex_y = tex_coord as i32 & (TEX_HEIGHT as i32 - 1);
			tex_coord += tex_step;

			//println!("tex_x: {} tex_y: {}, y: {}, x: {}", tex_x, tex_y, y, x);

			let mut colour = /*CanvasColour::TEAL;*/scene.textures[tex_index as usize][TEX_HEIGHT as usize * tex_x as usize + tex_y as usize];

            //println!("{y}/{draw_end}");

            if colour.b != 0 {
                //println!("{colour}");
            }

			if side == SideHit::YSide {
				colour = colour / 2;
			}

			draw_pixel(screen, x as i32, y, colour);
            //draw_line(screen, x as i32, y, y, colour);

		}
        //*/
        //println!("end");

        //println!("start: {draw_start} end: {draw_end} line height: {line_height}");

        //draw_line(screen, x as i32, draw_start, draw_end, colour);

    }
}

pub fn input(keys: HashSet<InputKeycode>, mouse_delta: f64, delta_time: f64, scene: &mut Scene) {

    let move_speed = MOVE_SPEED * delta_time;
    let rot_speed = ROT_SPEED * delta_time;

    //println!("move {} rot {}", move_speed, rot_speed);

    let player = &mut scene.plr;

    //delta_time = 1.0;

    let mut wish_pos = player.pos;

    for key in keys {
        match key {
            InputKeycode::W => {

                wish_pos += player.dir * move_speed;

            },
            InputKeycode::S => {

                wish_pos -= player.dir * move_speed;

            },
            // rotate to the right
            InputKeycode::D => {
                /*
                let rot = Rotation2::new(rot_speed);
                player.dir = rot * player.dir;
                */

                wish_pos.x -= player.dir.y * move_speed;
                wish_pos.y += player.dir.x * move_speed;
                
            },
            // rotate to the left
            InputKeycode::A => {

                /*
                let rot = Rotation2::new(-rot_speed);
                player.dir = rot * player.dir;
                */

                wish_pos.x += player.dir.y * move_speed;
                wish_pos.y -= player.dir.x * move_speed;

            },
            _ => {}
        }
    }

    let rot = Rotation2::new(mouse_delta * rot_speed);
    player.dir = rot * player.dir;

    if MAP[wish_pos.x as usize][player.pos.y as usize] == 0 {
        player.pos.x = wish_pos.x;
    }

    if MAP[player.pos.x as usize][wish_pos.y as usize] == 0 {
        player.pos.y = wish_pos.y;
    }

    //println!("{:?}", player.pos);

}