//! `shaders/mod.rs`
//!
//! Módulo principal de shaders que organiza y re-exporta todos los componentes.

use crate::framebuffer::Color;
use nalgebra_glm::Vec3;

// Submódulos
pub mod noise;      // Funciones de generación de ruido
pub mod utils;      // Utilidades para shaders
pub mod star_types; // Implementaciones de shaders de estrellas

// Re-exportar el trait principal
pub trait StarShader {
    /// Calcula el color de un fragmento en una posición específica de la superficie.
    ///
    /// # Arguments
    /// * `pos` - La posición del fragmento en el espacio del objeto.
    /// * `normal` - La normal de la superficie en esa posición.
    /// * `time` - El tiempo actual de la animación, para efectos dinámicos.
    ///
    /// # Returns
    /// Devuelve el `Color` calculado para el fragmento.
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color;
}

// Re-exportar los shaders para facilitar su uso
pub use star_types::{ClassicSunShader, PlasmaStarShader, PulsarShader, SupernovaShader};