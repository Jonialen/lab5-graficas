//! `shaders/utils.rs`
//!
//! Funciones de utilidad para shaders, incluyendo interpolación,
//! conversión de colores y efectos visuales.

use nalgebra_glm::Vec3;

// ===================================================================================
// ========== INTERPOLACIÓN ==========
// ===================================================================================

/// Interpola suavemente entre 0 y 1 cuando `x` está entre `edge0` y `edge1`.
///
/// Utiliza una función de Hermite cúbica para suavizar la transición.
///
/// # Arguments
/// * `edge0` - Valor mínimo del rango
/// * `edge1` - Valor máximo del rango
/// * `x` - Valor a interpolar
///
/// # Returns
/// Valor suavemente interpolado entre 0.0 y 1.0
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Interpolación lineal entre dos vectores 3D.
///
/// # Arguments
/// * `a` - Primer vector (cuando t=0.0)
/// * `b` - Segundo vector (cuando t=1.0)
/// * `t` - Factor de interpolación [0.0, 1.0]
///
/// # Returns
/// Vector interpolado
#[inline]
pub fn mix_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a * (1.0 - t) + b * t
}

// ===================================================================================
// ========== CONVERSIÓN DE COLOR ==========
// ===================================================================================

/// Convierte un valor de temperatura (0.0 a 1.0) a un color RGB.
///
/// Simula la radiación de cuerpo negro, yendo de rojo/naranja (frío)
/// a amarillo y blanco (caliente).
///
/// # Arguments
/// * `temp` - Temperatura normalizada [0.0, 1.0]
///
/// # Returns
/// Color RGB como Vec3 [0.0, 1.0]
#[inline]
pub fn temperature_to_color(temp: f32) -> Vec3 {
    let t = temp.clamp(0.0, 1.0);

    if t < 0.33 {
        // Naranja oscuro → Naranja brillante
        let factor = t / 0.33;
        mix_vec3(Vec3::new(1.0, 0.2, 0.0), Vec3::new(1.0, 0.5, 0.0), factor)
    } else if t < 0.66 {
        // Naranja brillante → Amarillo
        let factor = (t - 0.33) / 0.33;
        mix_vec3(Vec3::new(1.0, 0.5, 0.0), Vec3::new(1.0, 0.9, 0.3), factor)
    } else {
        // Amarillo → Blanco
        let factor = (t - 0.66) / 0.34;
        mix_vec3(Vec3::new(1.0, 0.9, 0.3), Vec3::new(1.0, 1.0, 1.0), factor)
    }
}

/// Convierte un valor de matiz (hue) en un color RGB iridiscente.
///
/// # Arguments
/// * `hue` - Matiz normalizado [0.0, 1.0] (ciclo completo de color)
///
/// # Returns
/// Color RGB como Vec3
#[inline]
pub fn hue_to_rgb(hue: f32) -> Vec3 {
    let h = hue % 1.0;
    
    if h < 0.33 {
        // Magenta → Violeta
        mix_vec3(
            Vec3::new(1.0, 0.0, 0.5),
            Vec3::new(0.5, 0.0, 1.0),
            h * 3.0,
        )
    } else if h < 0.66 {
        // Violeta → Cian
        mix_vec3(
            Vec3::new(0.5, 0.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
            (h - 0.33) * 3.0,
        )
    } else {
        // Cian → Magenta
        mix_vec3(
            Vec3::new(0.0, 1.0, 1.0),
            Vec3::new(1.0, 0.0, 0.5),
            (h - 0.66) * 3.0,
        )
    }
}

// ===================================================================================
// ========== EFECTOS VISUALES ==========
// ===================================================================================

/// Calcula un efecto Fresnel para simular reflexión en los bordes.
///
/// # Arguments
/// * `view_dir` - Dirección de la cámara normalizada
/// * `normal` - Normal de la superficie normalizada
/// * `power` - Exponente para controlar la intensidad (típicamente 2.0-5.0)
///
/// # Returns
/// Intensidad del efecto [0.0, 1.0]
#[inline]
pub fn fresnel(view_dir: &Vec3, normal: &Vec3, power: f32) -> f32 {
    (1.0 - view_dir.dot(normal).abs()).powf(power)
}

/// Genera una pulsación sinusoidal suavizada.
///
/// # Arguments
/// * `time` - Tiempo actual
/// * `frequency` - Frecuencia de pulsación
/// * `min` - Valor mínimo de salida
/// * `max` - Valor máximo de salida
///
/// # Returns
/// Valor pulsante entre min y max
#[inline]
pub fn pulse(time: f32, frequency: f32, min: f32, max: f32) -> f32 {
    let normalized = (time * frequency).sin() * 0.5 + 0.5;
    min + (max - min) * normalized
}

/// Genera una pulsación con curva exponencial (más dramática).
///
/// # Arguments
/// * `time` - Tiempo actual
/// * `frequency` - Frecuencia de pulsación
/// * `power` - Exponente para la curva (mayor = más dramático)
///
/// # Returns
/// Valor pulsante [0.0, 1.0]
#[inline]
pub fn pulse_pow(time: f32, frequency: f32, power: f32) -> f32 {
    ((time * frequency).sin() * 0.5 + 0.5).powf(power)
}