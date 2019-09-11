use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const NUM_OF_SLICES: u32 = 40;
const HALF_HEIGHT: u32 = WINDOW_HEIGHT / 2;
const HEIGHT_SLICE: u32 = HALF_HEIGHT / NUM_OF_SLICES;

const GRAD_R_TOP: u32 = 102;
const GRAD_R_BOT: u32 = 0;

const GRAD_G_TOP: u32 = 255;
const GRAD_G_BOT: u32 = 150;

const GRAD_B_TOP: u32 = 255;
const GRAD_B_BOT: u32 = 255;



fn main() -> Result<(), String >  {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("This is my Window", WINDOW_WIDTH, WINDOW_HEIGHT )
        .position_centered()
        .build()
        .expect( "could not init video subsystem" );

    let mut canvas = window.into_canvas().build()
        .expect( "could not make a canvas" );

    canvas.set_draw_color( Color::RGB( GRAD_R_TOP as u8, GRAD_G_TOP as u8, GRAD_B_TOP as u8 ) );

    canvas.clear();
    canvas.present();

    let mut i = 0;
    while i * HEIGHT_SLICE < HALF_HEIGHT  {

        let mut red: u32 = GRAD_R_BOT + ( ( (GRAD_R_TOP - GRAD_R_BOT ) / NUM_OF_SLICES ) * i );
        let mut green: u32 = GRAD_G_BOT + ( ( (GRAD_G_TOP - GRAD_G_BOT ) / NUM_OF_SLICES ) * i );
        let mut blue: u32 = GRAD_B_BOT + ( ( (GRAD_B_TOP - GRAD_B_BOT ) / NUM_OF_SLICES ) * i );

        if red > GRAD_R_TOP  {
            red = GRAD_R_TOP
        }
        if green > GRAD_G_TOP  {
            green = GRAD_G_TOP
        }
        if blue > GRAD_B_TOP  {
            blue = GRAD_B_TOP
        }

        println!( "red = {}, green = {}, blue = {}", red, green, blue );
        canvas.set_draw_color( Color::RGB( red as u8, green as u8, blue as u8 ) );
        canvas.fill_rect(Rect::new(0, ( i * HEIGHT_SLICE ) as i32, WINDOW_WIDTH, HEIGHT_SLICE ));

        i = i + 1

    }  //end while loop

    //canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}

            }  //end match

        }  //end inner for loop

    }  //end running

    Ok(())


}
