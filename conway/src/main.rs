const SCALE: i32 = 8;

mod framebuffer;
mod conway;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use conway::Life;

fn main() {
    // Arrancar raylib
    let (mut rl, thread) = raylib::init()
        .size(conway::WIDTH as i32 * SCALE, conway::HEIGHT as i32 * SCALE)
        .title("Conway's Game of Life")
        .build();

    // Crear el framebuffer de 100×100
    let mut fb = Framebuffer::new(conway::WIDTH as i32, conway::HEIGHT as i32, Color::WHITE);

    // Crear motor para jeugo
    let mut life = Life::new();

    // Bucle para hacer render constantemente, actualizando matriz acorde al juego
    while !rl.window_should_close() {
        // Llamar a la función render principal
        render(&mut rl, &thread, &mut fb, &mut life);
    }
}

// Render loop
fn render(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    fb: &mut Framebuffer,
    life: &mut Life,
) {
    // Control de velocidad del juego - lograr apreciar el movimiento
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Actualizar la lógica del juego de Conway
    life.update();

    // Renderizar el estado actual en el framebuffer
    life.render(fb);

    // Dibujar el framebuffer en la pantalla
    let mut d = rl.begin_drawing(thread);
    
    // Renderizar pixel por pixel
    for y in 0..conway::HEIGHT {
        for x in 0..conway::WIDTH {
            if let Some(color) = fb.get_pixel(x as i32, y as i32) {
                let rect = Rectangle::new(
                    (x * SCALE as usize) as f32,
                    (y * SCALE as usize) as f32,
                    SCALE as f32,
                    SCALE as f32,
                );
                d.draw_rectangle_rec(rect, color);
            }
        }
    }
}


    