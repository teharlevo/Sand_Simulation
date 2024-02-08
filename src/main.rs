#[warn(non_snake_case)]
use std::{fs, usize};
use std::time::{Instant, Duration};
use  std::marker::Copy;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render:: TextureCreator;
use sdl2::video::{Window, WindowContext};

use rand::{thread_rng, RngCore};

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const WIDTH_ISIZE: isize = WIDTH as isize;
const HEIGHT_ISIZE: isize = HEIGHT as isize;
const MATRIXSIZE:usize = (HEIGHT * WIDTH) as usize;
const PIXELARRAYSIZE:usize = MATRIXSIZE * 4;

const WINDOW_WIDTH: u32 = WIDTH ;
const WINDOW_HEIGHT:u32 = HEIGHT;
const MAP_START:isize = WIDTH_ISIZE * (HEIGHT_ISIZE - 1);

const SPRED_ORDER:[isize;8] = [1,0,-1,0,0,1,0,-1];


//nating, sand, water,smoke ,wood, fire;
const ELEMENTS:usize = 6;

// need color cange
const COLORS:[u8;ELEMENTS * 3] = [0,0,0,194, 178, 128,212,241,249, 132, 136, 132, 149,69,32,255,0,0];

const WEIGHTS:[i8;ELEMENTS] = [-100,1,0,-1,100,100];
fn main() {
    let mut matrix = vec![];
    for _ in 0..MATRIXSIZE {
        matrix.push(Partical{p_type:0,tic:true});
    }

    for i in 0..WIDTH_ISIZE/100 {
        circle(&mut  matrix,i * 120,HEIGHT_ISIZE/2,70,4);   
    }
    circle(&mut  matrix,3 * 120,HEIGHT_ISIZE/2,1,5);   

    let mut input_master = Input::new();

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
        input_master.update(&event_master);
        update(&mut matrix);
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

fn update(matrix:&mut [Partical]){

    let mut rng = thread_rng();
    let mut random = Random::new(rng.next_u64());
    matrix[pos_to_array_paint((WIDTH_ISIZE/4)*3, HEIGHT_ISIZE - 1)].p_type = 1;
    matrix[pos_to_array_paint((WIDTH_ISIZE/4)*1, HEIGHT_ISIZE - 1)].p_type = 2;
    matrix[pos_to_array_paint((WIDTH_ISIZE/4)*2, HEIGHT_ISIZE/4)].p_type = 3;

    for y in 0..HEIGHT_ISIZE{

        for x in 0..WIDTH_ISIZE{
            let dir  = 1;
            let index = pos_to_array_paint(x, y);
            match matrix[index].p_type {
                1 => {let dir ={if random.next_bool(){-1}else{1}};
                partical_move(&[0, -1, -dir, -1, dir, -1], matrix, x, y)},

                2 => {if random.next_bool(){-1}else{1};
                partical_move(&[0, -1, -dir, -1, dir, -1, dir, 0, -dir, 0], matrix, x, y)},

                3 => {let dir ={if random.next_bool(){-1}else{1}};
                partical_move(&[0, 1, -dir, 1, dir, 1, dir, 0, -dir, 0], matrix, x, y)},

                5 => partical_spred(matrix,&mut random, x, y),
                _ => {}
            }
        }
    }

    for i in 0..MATRIXSIZE{
            matrix[i].tic = false;
    }
}

fn pos_to_array_paint(x:isize,y:isize) ->usize{
    return (MAP_START
    + x - y * WIDTH_ISIZE)as usize;
}

fn legal_poaint(x:isize,y:isize) -> bool{
    return x >= 0 && x < WIDTH_ISIZE && y >= 0 && y < HEIGHT_ISIZE;
}

fn partical_move(order:&[isize],matrix:&mut [Partical],x:isize,y:isize){

    let point = pos_to_array_paint(x, y);
    if matrix[point].tic{
        return;
    }
    for i in 0..(order.len() / 2) {
        let new_x = x + order[i * 2];
        let new_y = y + order[i * 2 + 1];
        let intercting_point = pos_to_array_paint(new_x, new_y);

        if legal_poaint(new_x, new_y) 
        && WEIGHTS[matrix[intercting_point].p_type as usize] < WEIGHTS[matrix[point].p_type as usize]{
            matrix[point].tic = true;
            matrix.swap(point, intercting_point);
            break;
        }
    }
}

fn partical_spred(matrix:&mut [Partical],r:&mut Random,x:isize,y:isize){

    let point = pos_to_array_paint(x, y);
    if matrix[point].tic{
        return;
    }
    for i in 0..SPRED_ORDER.len()/2 { 
        if r.next_bool(){
            let new_x = x + SPRED_ORDER[i * 2];
            let new_y = y + SPRED_ORDER[i * 2 + 1];
            let intercting_point = pos_to_array_paint(new_x, new_y);

            if legal_poaint(new_x, new_y) && matrix[intercting_point].p_type == 4 {
                matrix[intercting_point].p_type = 5;
                matrix[intercting_point].tic = true;
            }
        }
    }
    if r.next_bool(){
        matrix[point].p_type = 3;
    }
}

//#[derive(Clone,Copy)]
struct Partical{
    p_type:u8,
    tic:bool,
}

fn circle(matrix:&mut [Partical],x:isize,y:isize,r:isize,partical_type:u8){

    for X in (-r + x)..(r + x){
        for Y in (-r + y)..(r + y){
            if (X - x) * (X - x) + (Y - y) * (Y - y) <= r * r && legal_poaint(X, Y){
                matrix[pos_to_array_paint(X, Y)].p_type = partical_type;
            }
        }
    }
}

fn render(bit_map:&[Partical],canvas:&mut Canvas<Window>,texture_creator:&TextureCreator<WindowContext>){
    let pixel_data:&mut [u8;PIXELARRAYSIZE] = &mut [0u8;PIXELARRAYSIZE];
    pixel_array_from_bit_map(bit_map,pixel_data);

    let texture = create_texture_from_pixels(texture_creator, pixel_data, WIDTH,HEIGHT);
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.copy(&texture, None, Rect::new(0, 0, WIDTH,HEIGHT))
          .expect("Texture rendering failed");
    canvas.present();
}

fn pixel_array_from_bit_map(bit_map:&[Partical],pixel_array:&mut [u8;PIXELARRAYSIZE]){

    for i in 0..MATRIXSIZE{

        let color_index = bit_map[i].p_type as usize * 3;
        let pixel_index = i * 4;

        pixel_array[pixel_index    ] = COLORS[color_index    ];
        pixel_array[pixel_index + 1] = COLORS[color_index + 1];
        pixel_array[pixel_index + 2] = COLORS[color_index + 2];
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
    
    texture.update(None, pixels, width as usize * 4).unwrap();
    texture
}

struct Input {
    mouse_click: bool,
    mouse_x:i32,
    mouse_y:i32,
}

impl Input {
    
    fn new() -> Self {
        Input { mouse_click:false,
        mouse_x:0,
        mouse_y:0,}
    }

    fn reset(&mut self){
        self.mouse_click = false;
        self.mouse_x = 0;
        self.mouse_y = 0;
    }

    fn update(&mut self,e: &sdl2::EventPump){
        self.reset();
    }
}
//random
struct Random {
    state: u64,
}

impl Random {
    
    fn new(seed: u64) -> Self {
        Random { state: seed }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    
    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max);
        min + self.next() % (max - min)
    }

    fn next_bool(&mut self) -> bool {
        self.next();
        self.state > u64::MAX/2
    }

    fn next_bool_chance(&mut self,chance:f64) -> bool {
        self.next();
        self.state as f64> u64::MAX as f64 * chance
    }
}