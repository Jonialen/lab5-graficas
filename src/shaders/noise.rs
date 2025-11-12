//! `shaders/noise.rs`
//!
//! Implementaciones de funciones de generación de ruido procedural.
//! Incluye Perlin Noise, Simplex Noise y Cellular (Worley) Noise.

use nalgebra_glm::Vec3;

// ===================================================================================
// ========== PERLIN NOISE ==========
// ===================================================================================

/// Implementación simplificada de ruido Perlin en 3D.
///
/// Genera un ruido suave y continuo, ideal para texturas naturales como nubes o terreno.
/// El resultado se normaliza al rango [0.0, 1.0].
///
/// # Arguments
/// * `x`, `y`, `z` - Coordenadas en el espacio 3D
///
/// # Returns
/// Valor de ruido en el rango [0.0, 1.0]
#[inline]
pub fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    // Coordenadas enteras y fraccionarias
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();

    // Suavizado de las coordenadas fraccionarias
    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    // Hashes de las 8 esquinas del cubo
    let aaa = hash(xi, yi, zi);
    let aba = hash(xi, yi + 1, zi);
    let aab = hash(xi, yi, zi + 1);
    let abb = hash(xi, yi + 1, zi + 1);
    let baa = hash(xi + 1, yi, zi);
    let bba = hash(xi + 1, yi + 1, zi);
    let bab = hash(xi + 1, yi, zi + 1);
    let bbb = hash(xi + 1, yi + 1, zi + 1);

    // Interpolación trilineal
    let x1 = lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1.0, yf, zf), u);
    let x2 = lerp(
        grad(aba, xf, yf - 1.0, zf),
        grad(bba, xf - 1.0, yf - 1.0, zf),
        u,
    );
    let y1 = lerp(x1, x2, v);

    let x3 = lerp(
        grad(aab, xf, yf, zf - 1.0),
        grad(bab, xf - 1.0, yf, zf - 1.0),
        u,
    );
    let x4 = lerp(
        grad(abb, xf, yf - 1.0, zf - 1.0),
        grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
        u,
    );
    let y2 = lerp(x3, x4, v);

    // Interpolación final y mapeo al rango [0, 1]
    (lerp(y1, y2, w) + 1.0) * 0.5
}

/// Función de suavizado (fade) para Perlin Noise.
///
/// Utiliza la curva polinómica 6t^5 - 15t^4 + 10t^3
#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Interpolación lineal entre dos valores.
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Genera un hash pseudoaleatorio a partir de coordenadas enteras.
#[inline]
fn hash(x: i32, y: i32, z: i32) -> i32 {
    let mut n = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(z.wrapping_mul(1274126177));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n & 0xff
}

/// Selecciona un gradiente pseudoaleatorio y calcula el producto punto.
#[inline]
fn grad(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };
    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

// ===================================================================================
// ========== SIMPLEX NOISE ==========
// ===================================================================================

/// Implementación simplificada de ruido Simplex en 3D.
///
/// Es computacionalmente más eficiente que Perlin Noise y produce menos artefactos
/// direccionales. Esta implementación combina dos capas de ruido Perlin.
///
/// # Arguments
/// * `x`, `y`, `z` - Coordenadas en el espacio 3D
///
/// # Returns
/// Valor de ruido en el rango aproximado [0.0, 1.0]
#[inline]
pub fn simplex_noise(x: f32, y: f32, z: f32) -> f32 {
    let n0 = perlin_noise(x, y, z);
    let n1 = perlin_noise(x * 2.0 + 5.2, y * 2.0 + 1.3, z * 2.0 + 8.1);
    (n0 + n1 * 0.5) / 1.5
}

// ===================================================================================
// ========== CELLULAR/WORLEY NOISE ==========
// ===================================================================================

/// Implementación de ruido celular (Worley/Voronoi).
///
/// Crea patrones que se asemejan a células o cristales, calculando la distancia
/// al punto de una red pseudoaleatoria más cercano.
///
/// # Arguments
/// * `x`, `y`, `z` - Coordenadas en el espacio 3D
///
/// # Returns
/// Valor de ruido donde 1.0 representa las "paredes" celulares
#[inline]
pub fn cellular_noise(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor();
    let yi = y.floor();
    let zi = z.floor();

    let mut min_dist = 10.0f32;

    // Itera sobre el cubo de 3x3x3 celdas alrededor de la celda actual
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                let cell_x = xi + i as f32;
                let cell_y = yi + j as f32;
                let cell_z = zi + k as f32;

                // Genera un punto pseudoaleatorio dentro de cada celda
                let rand_x = cell_noise(cell_x, cell_y, cell_z);
                let rand_y = cell_noise(cell_x + 1.0, cell_y + 2.0, cell_z + 3.0);
                let rand_z = cell_noise(cell_x + 4.0, cell_y + 5.0, cell_z + 6.0);

                let point_x = cell_x + rand_x;
                let point_y = cell_y + rand_y;
                let point_z = cell_z + rand_z;

                // Calcula la distancia euclidiana al punto
                let dist =
                    ((x - point_x).powi(2) + (y - point_y).powi(2) + (z - point_z).powi(2)).sqrt();
                min_dist = min_dist.min(dist);
            }
        }
    }

    // Invierte para que las "paredes" celulares sean brillantes
    1.0 - min_dist.min(1.0)
}

/// Función de hash simple para generar puntos en el ruido celular.
#[inline]
fn cell_noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 12.9898 + y * 78.233 + z * 45.164).sin() * 43758.5453).fract()
}

// ===================================================================================
// ========== TURBULENCIA (MULTI-OCTAVA) ==========
// ===================================================================================

/// Genera turbulencia sumando múltiples "octavas" de un tipo de ruido.
///
/// Cada octava tiene mayor frecuencia y menor amplitud, añadiendo detalle progresivo.
///
/// # Arguments
/// * `p` - Posición en el espacio 3D
/// * `octaves` - Número de capas de ruido (típicamente 3-6)
/// * `noise_type` - Tipo de ruido: 0=Perlin, 1=Simplex, 2=Cellular
///
/// # Returns
/// Valor de turbulencia acumulado
#[inline]
pub fn turbulence(p: Vec3, octaves: i32, noise_type: i32) -> f32 {
    let mut sum = 0.0;
    let mut freq = 1.0;
    let mut amp = 1.0;

    for _ in 0..octaves {
        let noise = match noise_type {
            0 => perlin_noise(p.x * freq, p.y * freq, p.z * freq),
            1 => simplex_noise(p.x * freq, p.y * freq, p.z * freq),
            2 => cellular_noise(p.x * freq, p.y * freq, p.z * freq),
            _ => perlin_noise(p.x * freq, p.y * freq, p.z * freq),
        };
        sum += amp * noise;
        freq *= 2.0; // Doble frecuencia
        amp *= 0.5;  // Mitad amplitud
    }
    sum
}