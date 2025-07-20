use raylib::prelude::*;

pub struct Framebuffer {
    image: Image,
    width: i32,
    height: i32,
}

// Implementación de Framebuffer, se parece al Object de Kotlin
impl Framebuffer {
    // Crea un framebuffer (matriz) de tamaño width x height
    pub fn new(width: i32, height: i32, bg_color: Color) -> Self {
        let image = Image::gen_image_color(width, height, bg_color);
        Framebuffer {
            image, width, height
        }
    }

    // Dibujar un pixel en el FB
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x>= 0 && x < self.width && y >= 0 && y < self.height {
            // Dibujar pixel en la imagen
            self.image.draw_pixel(x, y, color);
        } else {
            println!("Error: Pixel fuera de los límites del framebuffer");
        }
    }

    /// Devuelve el color almacenado en (x, y) o `None` si está fuera de rango
    pub fn get_pixel(&mut self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(self.image.get_color(x, y))
    }
    
    // Función get_color para determinar el color de una célula según su estado
    // Esta función es parte de la dinámica del juego de Conway
    pub fn get_color(&self, alive: bool) -> Color {
        if alive {
            Color::WHITE  // Células vivas son blancas
        } else {
            Color::BLACK  // Células muertas son gris oscuro
        }
    }

}