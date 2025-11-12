//! Módulo de framebuffer para renderizado por software.
//
// Este archivo define las estructuras y métodos para el manejo de color, almacenamiento de píxeles y profundidad (z-buffer)
// en imágenes renderizadas, facilitando la integración con librerías gráficas como Raylib.

use nalgebra_glm::Vec3; // Vector 3D para manipulación de colores flotantes.

/// Representa un color RGB de 8 bits por canal.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Componente rojo (0-255).
    pub r: u8,
    /// Componente verde (0-255).
    pub g: u8,
    /// Componente azul (0-255).
    pub b: u8,
}

impl Color {
    /// Crea un nuevo color RGB.
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    /// Convierte un vector Vec3 (componentes 0.0-1.0) a un color RGB (0-255).
    #[inline]
    pub fn from_vec3(v: Vec3) -> Self {
        Color {
            r: (v.x.clamp(0.0, 1.0) * 255.0) as u8,
            g: (v.y.clamp(0.0, 1.0) * 255.0) as u8,
            b: (v.z.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }
}

/// Framebuffer que almacena los datos de color y profundidad de la imagen renderizada.
pub struct Framebuffer {
    /// Ancho del framebuffer en píxeles.
    pub width: usize,
    /// Alto del framebuffer en píxeles.
    pub height: usize,
    /// Búfer de color en formato RGBA (4 bytes por píxel).
    pub buffer: Vec<u8>,
    /// Búfer de profundidad (z-buffer) para pruebas de visibilidad.
    pub zbuffer: Vec<f32>,
}

impl Framebuffer {
    /// Crea un nuevo framebuffer con las dimensiones especificadas.
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height * 4], // Inicializa el color a negro.
            zbuffer: vec![f32::INFINITY; width * height], // Inicializa la profundidad a infinito.
        }
    }

    /// Limpia el framebuffer, estableciendo todos los píxeles a un color y reseteando el z-buffer.
    #[inline]
    pub fn clear(&mut self, color: Color) {
        for i in 0..self.width * self.height {
            let idx = i * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255; // Canal alfa opaco.
        }
        self.zbuffer.fill(f32::INFINITY); // Resetea la profundidad.
    }

    /// Establece el color de un píxel (x, y) si pasa la prueba de profundidad.
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color, depth: f32) {
        if x >= self.width || y >= self.height {
            return; // Ignora coordenadas fuera de rango.
        }

        let index = y * self.width + x;

        // Solo dibuja si el nuevo píxel está más cerca que el anterior.
        if depth < self.zbuffer[index] {
            self.zbuffer[index] = depth;
            let idx = index * 4;
            self.buffer[idx] = color.r;
            self.buffer[idx + 1] = color.g;
            self.buffer[idx + 2] = color.b;
            self.buffer[idx + 3] = 255;
        }
    }

    /// Devuelve el búfer de color como slice de bytes para integración con APIs gráficas.
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}
