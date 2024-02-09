use std::fs;
use image::open;
use std::path::Path;
use std::time::{Instant, Duration};

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render:: TextureCreator;
use sdl2::video::{Window, WindowContext};

use rand::{thread_rng, RngCore};

use sdl2::mouse::MouseButton;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseWheelDirection;
use sdl2::EventPump;

const WIDTH: u32 = 	640;
const HEIGHT: u32 = 360;
const WIDTH_I32: i32 = WIDTH as i32;
const HEIGHT_I32: i32 = HEIGHT as i32;
const MATRIXSIZE:usize = (HEIGHT * WIDTH) as usize;
const PIXELARRAYSIZE:usize = MATRIXSIZE * 4;

const WINDOW_WIDTH: u32 = WIDTH ;
const WINDOW_HEIGHT:u32 = HEIGHT;
const MAP_START:i32 = WIDTH_I32 * (HEIGHT_I32 - 1);

const SPRED_ORDER:[i32;8] = [0,1,-1,0,0,-1,1,0];


//nating, sand, water,smoke ,wood, fire,oil,plant,lava,stone;
const ELEMENTS:usize = 10;
//const ELEMENTS_U8:u8 = ELEMENTS as u8;

// need color cange
const COLORS:[u8;ELEMENTS * 3 + 3] = [0,0,0,194, 178, 128,0,0,255, 132, 136, 132, 149
,69,32,255,0,0,55,58,54,0,255,0,255,140,0,100,100,100,255,0,0];

const WEIGHTS:[i8;ELEMENTS] = [-100,2,0,-2,100,100,1,100,0,100];

const FLAMES_NUM:usize = 2;
const FLAMES_RESOLT:[u8;FLAMES_NUM] = [5,7];
const FALMEBILTY:[f64;ELEMENTS * FLAMES_NUM] =
[0.05f64,0.0f64,0.0f64,0.0f64,0.3f64,0.0f64,0.99f64,0.6f64,0.0f64,0.0f64,//catch fire;
 0.0f64,0.0f64,0.6f64,0.0f64,0.0f64,0.0f64,0.0f64,0.0f64,0.0f64,0.0f64,//catch plant
 ];

const KEYS:[Scancode;10] = [Scancode::Num0,Scancode::Num1,Scancode::Num2
,Scancode::Num3,Scancode::Num4,Scancode::Num5
,Scancode::Num6,Scancode::Num7,Scancode::Num8,Scancode::Num9];
const MAX_RADIOS:i32 = 100;
const RED_CIRCLE_OUT_LINE:i32 = 4;

fn main() {
    //let mut matrix = spma_file_to_image("9503390649809988085.spma");
    let  mut matrix:Vec<Partical> = vec![]; 
    for _ in 0..MATRIXSIZE {
        matrix.push(Partical{p_type:0,tic:true,intrcact_tic:true});
    }

    //for i in 0..WIDTH_I32/100 {
    //    circle(&mut  matrix,i * 120,HEIGHT_I32/2,70,2);   
    //}
    //circle(&mut  matrix,3 * 120,(HEIGHT_I32/4)*1,20,7);   

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
                sdl2::event::Event::MouseWheel { x, y, direction, .. } => {
                    match direction {
                        MouseWheelDirection::Normal => {
                            input_master.add_to_radios(y);
                        }
                        MouseWheelDirection::Flipped => {
                            input_master.add_to_radios(y);
                        }
                        _ => {}
                    }
                },
                sdl2::event::Event::DropFile { filename, .. } => {
                    let file_type = filename.split(".").last().unwrap();
                    if file_type.eq("spma"){
                        matrix = spma_file_to_matrix(&filename);
                    }
                    else if file_type.eq("png") || file_type.eq("jpg"){
                        matrix = image_file_to_matrix(&filename);
                    }
                }
                _ => {}
            }
        }

        input_master.update(&event_master);
        update(&mut matrix,&mut input_master);
        render(&matrix,&input_master, &mut canvas, &texture_creator);

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

fn update(matrix:&mut [Partical],input:&mut Input){

    let mut rng = thread_rng();
    let mut random = Random::new(rng.next_u64());

    if input.mouse_click[0]{
        circle(matrix,input.mouse_x,input.mouse_y,input.radios,input.element);   
    }

    if input.save{
        save(matrix, rng.next_u64().to_string().as_str());
        input.save = false;
    }

    if input.stop {return;}

    for y in 0..HEIGHT_I32{
        //let dir_x:Vec<i32> = {if random.next_bool(){(0..WIDTH_I32).collect()}else{(0..WIDTH_I32).rev().collect()}};
        for x in 0..WIDTH_I32{
            let dir  = 1;
            let index = pos_to_array_paint(x, y);
            match matrix[index].p_type {

                1 => {let dir ={if random.next_bool(){-1}else{1}};
                partical_move(&[0, -1, -dir, -1, dir, -1], matrix, x, y)},

                2 => {if random.next_bool(){-1}else{1};
                partical_move(&[0, -1, -dir, -1, dir, -1, dir, 0, -dir, 0], matrix, x, y)},

                3 => {let dir ={if random.next_bool(){-1}else{1}};
                partical_move(&[0, 1, -dir, 1, dir, 1, dir, 0, -dir, 0], matrix, x, y)},

                5 => partical_spred(matrix,0,&mut random, x, y,0.2f64),

                6 => {if random.next_bool(){-1}else{1};
                partical_move(&[0, -1, -dir, -1, dir, -1, dir, 0, -dir, 0], matrix, x, y)},

                7 => partical_spred(matrix,1,&mut random, x, y,0.0f64),

                8 => {if random.next_bool(){-1}else{1};
                partical_move(&[0, -1, -dir, -1, dir, -1, dir, 0, -dir, 0], matrix, x, y);
                partical_spred(matrix,0,&mut random, x, y,0.0f64)},
                _ => {}
            }
        }
    }

    for i in 0..MATRIXSIZE{
        matrix[i].tic = false;
        matrix[i].intrcact_tic = false;
    }
}

fn pos_to_array_paint(x:i32,y:i32) ->usize{
    return (MAP_START
    + x - y * WIDTH_I32)as usize;
}

fn legal_poaint(x:i32,y:i32) -> bool{
    return x >= 0 && x < WIDTH_I32 && y >= 0 && y < HEIGHT_I32;
}

fn partical_move(order:&[i32],matrix:&mut [Partical],x:i32,y:i32){

    let point = pos_to_array_paint(x, y);
    if matrix[point].tic{
        return;
    }
    for i in 0..(order.len() / 2) {
        let new_x = x + order[i * 2];
        let new_y = y + order[i * 2 + 1];
        let intercting_point = pos_to_array_paint(new_x, new_y);

        if legal_poaint(new_x, new_y) 
        && (matrix[intercting_point].p_type == 0 || 
            (WEIGHTS[matrix[intercting_point].p_type as usize] < WEIGHTS[matrix[point].p_type as usize]
        && !matrix[intercting_point].intrcact_tic && new_y - y == -1)){
            matrix[point].tic = true;
            matrix[intercting_point].intrcact_tic = true;
            matrix.swap(point, intercting_point);
            break;
        }
    }
}

fn partical_spred(matrix:&mut [Partical],flameabilty_type:usize,r:&mut Random,x:i32,y:i32
    ,chance:f64){

    let point = pos_to_array_paint(x, y);
    if matrix[point].tic{
        return;
    }
    for i in 0..SPRED_ORDER.len()/2 { 
        let new_x = x + SPRED_ORDER[i * 2];
        let new_y = y + SPRED_ORDER[i * 2 + 1];
        let intercting_point = pos_to_array_paint(new_x, new_y);

        if legal_poaint(new_x, new_y) && r.next_bool_chance(
            FALMEBILTY[matrix[intercting_point].p_type as usize + flameabilty_type * ELEMENTS]
        ){
            matrix[intercting_point].p_type = FLAMES_RESOLT[flameabilty_type];
            matrix[intercting_point].tic = true;
        }
    }
    if r.next_bool_chance(chance){
        matrix[pos_to_array_paint(x, y)].p_type = 3;
    }

}

//#[derive(Clone,Copy)]
struct Partical{
    p_type:u8,
    tic:bool,
    intrcact_tic:bool,
}

fn circle(matrix:&mut [Partical],x:i32,y:i32,r:i32,partical_type:u8){

    for X in (-r + x)..(r + x){
        for Y in (-r + y)..(r + y){
            if (X - x) * (X - x) + (Y - y) * (Y - y) <= r * r && legal_poaint(X, Y){
                matrix[pos_to_array_paint(X, Y)].p_type = partical_type;
            }
        }
    }
}

fn circle_layer(pixel_data:&mut [u8;PIXELARRAYSIZE],x:i32,y:i32,r:i32,out_line_size:i32){

    let out_line_radius = r + out_line_size / 2;
    let inner_radius = r - out_line_size / 2;

    for X in (-out_line_radius + x)..(out_line_radius + x) {
        for Y in (-out_line_radius + y)..(out_line_radius + y) {
            if legal_poaint(X, Y) && ((X - x) * (X - x) + (Y - y) * (Y - y) >= r * r - inner_radius *  out_line_radius && 
            (X - x) * (X - x) + (Y - y) * (Y - y) <= r * r){

                let pos = pos_to_array_paint(X, Y) * 4;
                pixel_data[pos    ] = 255;
                pixel_data[pos + 1] = 0;
                pixel_data[pos + 2] = 0;
            }
        }
    }
}

fn render(bit_map:&[Partical],input:&Input,canvas:&mut Canvas<Window>,texture_creator:&TextureCreator<WindowContext>){
    let pixel_data:&mut [u8;PIXELARRAYSIZE] = &mut [0u8;PIXELARRAYSIZE];
    pixel_array_from_bit_map(bit_map,pixel_data);
    circle_layer(pixel_data,input.mouse_x,input.mouse_y,input.radios,
        input.radios * 2 - RED_CIRCLE_OUT_LINE);

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
    mouse_click: [bool;3],
    mouse_x:i32,
    mouse_y:i32,
    element:u8,
    radios:i32,
    save:bool,
    save_helper:bool,
    stop:bool,
    stop_helper:bool,
}

impl Input {
    
    fn new() -> Self {
        Input { mouse_click:[false;3],
        mouse_x:0,
        mouse_y:0,
        element:1,
        radios:50,
        save:false,
        save_helper:false,
        stop:false,
        stop_helper:false,
    }
    }

    fn reset(&mut self){
        self.mouse_click = [false,false,false];
        self.mouse_x = 0;
        self.mouse_y = 0;
    }

    fn update(&mut self,e: &sdl2::EventPump){
        self.mouse_click[0] = e.mouse_state().is_mouse_button_pressed(MouseButton::Left);
        self.mouse_click[1] = e.mouse_state().is_mouse_button_pressed(MouseButton::Middle);
        self.mouse_click[2] = e.mouse_state().is_mouse_button_pressed(MouseButton::Right);
        self.mouse_x = e.mouse_state().x();
        self.mouse_y = HEIGHT_I32 - e.mouse_state().y();

        for i in 0..ELEMENTS{
            if e.keyboard_state().is_scancode_pressed(KEYS[i]){
                self.element = i as u8;
            }
        }

        if self.save_helper && e.keyboard_state().is_scancode_pressed(Scancode::S){
            self.save_helper = false;
            self.save = true;
        }
        else if !e.keyboard_state().is_scancode_pressed(Scancode::S){
            self.save_helper = true;
        }

        if self.stop_helper && e.keyboard_state().is_scancode_pressed(Scancode::Space){
            self.stop_helper = false;
            self.stop = !self.stop;
        }
        else if !e.keyboard_state().is_scancode_pressed(Scancode::Space){
            self.stop_helper = true;
        }
    }

    fn add_to_radios(&mut self,add:i32){
        self.radios += add;
        if self.radios > MAX_RADIOS{
            self.radios = MAX_RADIOS;
        }
        else if self.radios <= 0 {
            self.radios = 0;
        }
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
        !(self.state as f64> u64::MAX as f64 * chance)
    }
}

fn save(matrix:&[Partical],name:&str){
    let mut text = String::from("");

    text = text + name +"\n";

    text = text + "w:" + &WIDTH.to_string() + "\n";
    text = text + "h:" + &HEIGHT.to_string() + "\n" + "\n";
    for y in 0..HEIGHT_I32{
        let mut line_text = String::from("");
        for x in 0..WIDTH_I32{
            let pixel = matrix[pos_to_array_paint(x, y)].p_type;
            let pixel_text = format!("{:02X}", pixel);
            line_text += pixel_text.as_str();
        }
        text += line_text.as_str(); 
        text += "\n";
    }
    _ = fs::write(name.to_string() + ".spma", text);
}

fn spma_file_to_matrix(path:&str) -> Vec<Partical>{

    let mut matrix:Vec<Partical> = vec![];

    let file_str = fs::read_to_string(path).unwrap();
    let lines:Vec<&str> = file_str.split("\n").collect();
    let name = &lines.get(0).unwrap()[0..lines.get(0).unwrap().len()];

    let width_text = lines.get(1).unwrap()[0..lines.get(1).unwrap().len()]
    .split(":").last().unwrap();

    let hight_text = lines.get(2).unwrap()[0..lines.get(2).unwrap().len()]
    .split(":").last().unwrap();

    let width= width_text.to_string().parse::<usize>().unwrap();
    let hight = hight_text.to_string().parse::<usize>().unwrap();

    for y in (4..(hight + 4)).rev(){
        let nums_str:&str = lines.get(y).unwrap();
        let nums = hex_to_bytes(nums_str).unwrap();
        for x in 0..width{
            matrix.push(Partical{
                p_type:nums[x],
                tic:false,
                intrcact_tic: false,
            })
        }
    }
    return  matrix;
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                      .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

const HEIGHT_F32:f32 = HEIGHT as f32;
const WIDTH_F32:f32 = WIDTH as f32;

fn image_file_to_matrix(path:&str) -> Vec<Partical>{

    let mut matrix:Vec<Partical> = vec![];

    let img = open(path).unwrap().into_rgb8();

    let y_move:f32 = img.height() as f32/HEIGHT_F32;
    let x_move:f32 = img.width() as f32/WIDTH_F32;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pixel = 
            img.get_pixel((x as f32 * x_move) as u32, (y as f32 * y_move) as u32);
            let mut dis = [0i32;ELEMENTS];

            for elemt in 0..ELEMENTS{
                dis[elemt] = ((pixel.0[0]  as i32 - COLORS[elemt * 3]  as i32) as i32).abs()
                + ((pixel.0[1]  as i32 - COLORS[elemt * 3 + 1] as i32)).abs()
                + ((pixel.0[2] as i32 - COLORS[elemt * 3 + 2] as i32)).abs()
            }
            
            let mut ele = 0;
            let mut maxdis = 900;
            for elemt in 0..ELEMENTS {
                if dis[elemt] < maxdis{
                    ele = elemt as u8;
                    maxdis = dis[elemt];
                }
            }
            matrix.push(Partical { p_type: ele, tic: false, intrcact_tic:false })
        }
    }
    return  matrix;
}