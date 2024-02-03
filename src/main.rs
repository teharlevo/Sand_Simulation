use std::{fs, usize};
use std::time::{Instant, Duration};
//use sdl2::event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render:: TextureCreator;
use sdl2::video::{Window, WindowContext};

use rand::Rng;

const WIDTH: u32 = 500;
const HEIGHT: u32 = 400;
const WIDTH_ISIZE: isize = WIDTH as isize;
const HEIGHT_ISIZE: isize = HEIGHT as isize;
const MATRIXSIZE:usize = (HEIGHT * WIDTH) as usize;
const PIXELARRAYSIZE:usize = MATRIXSIZE * 4;

const WINDOW_WIDTH: u32 = WIDTH ;
const WINDOW_HEIGHT:u32 = HEIGHT;


//nating, sand;
const ELEMENTS:usize = 3;

const COLORS:[u8;ELEMENTS * 3] = [0,0,0,194, 178, 128,212,241,249];
fn main() {
    let matrix: &mut [u8; MATRIXSIZE] = &mut [0u8; MATRIXSIZE];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 Texture Example", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    let mut event_master = sdl_context.event_pump().unwrap();

    let mut last_frame_time = Instant::now();
    let mut fps_counter = 0;
    let mut frames = 0;

    'mainloop: loop {
        for event in event_master.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'mainloop,
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'mainloop;
                },
                _ => {}
            }
        }
        update(matrix);
        render(&matrix, &mut canvas, &texture_creator);

        frames += 1;
        let elapsed_time = last_frame_time.elapsed();
        if elapsed_time >= Duration::from_secs(1) {
            fps_counter = frames;
            frames = 0;
            last_frame_time = Instant::now();
            println!("FPS: {}", fps_counter);
        }
    }
}

fn update(matrix:&mut [u8;MATRIXSIZE]){
    let mut rng = rand::thread_rng();

    matrix[pos_to_array_paint((WIDTH_ISIZE/4)*3, HEIGHT_ISIZE - 1)] = 1;
    matrix[pos_to_array_paint((WIDTH_ISIZE/4)*1, HEIGHT_ISIZE - 1)] = 2;

    for y in 0..HEIGHT_ISIZE{
        for x in 0..WIDTH_ISIZE{
            
            if matrix[pos_to_array_paint(x, y)] == 1{
            if legal_poaint(x, y - 1) && matrix[pos_to_array_paint(x, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x, y - 1)] = 1;
            }
            else if legal_poaint(x - 1, y - 1) && matrix[pos_to_array_paint(x - 1, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x - 1, y - 1)] = 1;
            }else if legal_poaint(x + 1, y - 1) && matrix[pos_to_array_paint(x + 1, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x + 1, y - 1)] = 1;
            }
            }
            else if matrix[pos_to_array_paint(x, y)] == 2{
            if legal_poaint(x, y - 1) && matrix[pos_to_array_paint(x, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x, y - 1)] = 2;
            }
            else if legal_poaint(x - 1, y - 1) && matrix[pos_to_array_paint(x - 1, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x - 1, y - 1)] = 2;

            }else if legal_poaint(x + 1, y - 1) && matrix[pos_to_array_paint(x + 1, y - 1)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x + 1, y - 1)] = 2;

            }
            else if legal_poaint(x + 1, y) && matrix[pos_to_array_paint(x + 1, y)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x + 1, y)] = 2;

            }else if legal_poaint(x - 1, y) && matrix[pos_to_array_paint(x - 1, y)] == 0{

                matrix[pos_to_array_paint(x, y)] = 0;
                matrix[pos_to_array_paint(x - 1, y)] = 2;

            }
            }
        }
    }
}

fn pos_to_array_paint(x:isize,y:isize) ->usize{
    return ((WIDTH_ISIZE * (HEIGHT_ISIZE - 1) )
    + x - y * WIDTH_ISIZE)as usize;
}

fn legal_poaint(x:isize,y:isize) -> bool{
    return x >= 0 && x < WIDTH_ISIZE && y >= 0 && y < HEIGHT_ISIZE;
}

fn render(bit_map:&[u8;MATRIXSIZE],canvas:&mut Canvas<Window>,texture_creator:&TextureCreator<WindowContext>){
    let pixel_data:&mut [u8;PIXELARRAYSIZE] = &mut [0u8;PIXELARRAYSIZE];
    pixel_array_from_bit_map(bit_map,pixel_data);

    let texture = create_texture_from_pixels(texture_creator, pixel_data, WIDTH,HEIGHT);
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.copy(&texture, None, Rect::new(0, 0, WIDTH,HEIGHT))
          .expect("Texture rendering failed");
    canvas.present();
}

fn pixel_array_from_bit_map(bit_map:&[u8;MATRIXSIZE],pixel_array:&mut[u8;PIXELARRAYSIZE]){

    for i in 0..MATRIXSIZE{
        pixel_array[i * 4] =     COLORS[(bit_map[i] * 3) as usize];
        pixel_array[i * 4 + 1] = COLORS[(bit_map[i] * 3 + 1) as usize];
        pixel_array[i * 4 + 2] = COLORS[(bit_map[i] * 3 + 2) as usize];
    }
}

fn create_texture_from_pixels<'a>(
    texture_creator: &'a TextureCreator<sdl2::video::WindowContext>,
    pixels: &'a [u8],
    width: u32,
    height: u32,
) -> Texture<'a>{
    
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::BGR888, width, height)
        .expect("Failed to create texture");
    
    texture
        .update(None, pixels,(width * 4 )as usize)
        .expect("Failed to update texture");

    texture
}