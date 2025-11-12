// Utiliza el tipo Vec3 de la biblioteca nalgebra_glm para manejar vectores de 3D.
use nalgebra_glm::Vec3;

// Define una estructura para representar un color con componentes rojo, verde y azul (RGB).
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8, // Componente rojo del color.
    pub g: u8, // Componente verde del color.
    pub b: u8, // Componente azul del color.
}

impl Color {
    // Define colores constantes para un fácil acceso.
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 }; // Color negro.
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 }; // Color blanco.

    // Crea una nueva instancia de Color.
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    // Convierte un vector de 3D (Vec3) a un color. Los componentes del vector se escalan de 0.0-1.0 a 0-255.
    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Color {
            r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    // Convierte un color a un vector de 3D (Vec3). Los componentes del color se normalizan de 0-255 a 0.0-1.0.
    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
    }

    // Convierte el color a un tipo de color compatible con la biblioteca Raylib.
    #[inline]
    pub fn to_raylib(&self) -> raylib::color::Color {
        raylib::color::Color::new(self.r, self.g, self.b, 255)
    }
}

// Define el búfer de fotogramas, que almacena los datos de píxeles y profundidad de una imagen renderizada.
pub struct Framebuffer {
    pub width: usize, // Ancho del búfer de fotogramas en píxeles.
    pub height: usize, // Alto del búfer de fotogramas en píxeles.
    pub buffer: Vec<u8>, // Búfer de píxeles en formato RGBA (4 bytes por píxel).
    pub zbuffer: Vec<f32>, // Búfer de profundidad para el Z-buffering.
}

impl Framebuffer {
    // Crea un nuevo búfer de fotogramas con las dimensiones especificadas.
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height * 4], // Inicializa el búfer de color a negro.
            zbuffer: vec![f32::INFINITY; width * height], // Inicializa el búfer de profundidad a infinito.
        }
    }

    // Limpia el búfer de fotogramas, estableciendo todos los píxeles a un color específico.
    #[inline]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width * self.height {
            let idx = i * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255; // El canal alfa se establece en 255 (opaco).
        }
        self.zbuffer.fill(f32::INFINITY); // Restablece el búfer de profundidad.
    }

    // Establece el color de un píxel en las coordenadas (x, y) si su profundidad es menor que la actual.
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color, depth: f32) {
        if x >= self.width || y >= self.height {
            return; // No hace nada si las coordenadas están fuera de los límites.
        }

        let index = y * self.width + x;

        // Comprueba si el nuevo píxel está más cerca que el píxel existente.
        if depth < self.zbuffer[index] {
            self.zbuffer[index] = depth; // Actualiza el búfer de profundidad.
            let idx = index * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255; // El canal alfa se establece en 255.
        }
    }

    // Devuelve una referencia al búfer de píxeles como un slice de bytes, para ser usado por Raylib.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}
