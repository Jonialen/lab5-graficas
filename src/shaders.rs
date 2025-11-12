use crate::framebuffer::Color;
use nalgebra_glm::Vec3;

pub trait StarShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color;
}

// ========== FUNCIONES DE RUIDO ==========

// Perlin Noise simplificado - Ruido suave y continuo
#[inline]
fn perlin_noise(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let zi = z.floor() as i32;

    let xf = x - x.floor();
    let yf = y - y.floor();
    let zf = z - z.floor();

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let aaa = hash(xi, yi, zi);
    let aba = hash(xi, yi + 1, zi);
    let aab = hash(xi, yi, zi + 1);
    let abb = hash(xi, yi + 1, zi + 1);
    let baa = hash(xi + 1, yi, zi);
    let bba = hash(xi + 1, yi + 1, zi);
    let bab = hash(xi + 1, yi, zi + 1);
    let bbb = hash(xi + 1, yi + 1, zi + 1);

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

    (lerp(y1, y2, w) + 1.0) * 0.5
}

#[inline]
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

#[inline]
fn hash(x: i32, y: i32, z: i32) -> i32 {
    let mut n = x
        .wrapping_mul(374761393)
        .wrapping_add(y.wrapping_mul(668265263))
        .wrapping_add(z.wrapping_mul(1274126177));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n & 0xff
}

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

// Simplex Noise simplificado - Más eficiente que Perlin
#[inline]
fn simplex_noise(x: f32, y: f32, z: f32) -> f32 {
    let n0 = perlin_noise(x, y, z);
    let n1 = perlin_noise(x * 2.0 + 5.2, y * 2.0 + 1.3, z * 2.0 + 8.1);
    (n0 + n1 * 0.5) / 1.5
}

// Cellular/Worley Noise - Crea patrones celulares
#[inline]
fn cellular_noise(x: f32, y: f32, z: f32) -> f32 {
    let xi = x.floor();
    let yi = y.floor();
    let zi = z.floor();

    let mut min_dist = 10.0f32;

    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                let cell_x = xi + i as f32;
                let cell_y = yi + j as f32;
                let cell_z = zi + k as f32;

                let rand_x = cell_noise(cell_x, cell_y, cell_z);
                let rand_y = cell_noise(cell_x + 1.0, cell_y + 2.0, cell_z + 3.0);
                let rand_z = cell_noise(cell_x + 4.0, cell_y + 5.0, cell_z + 6.0);

                let point_x = cell_x + rand_x;
                let point_y = cell_y + rand_y;
                let point_z = cell_z + rand_z;

                let dist =
                    ((x - point_x).powi(2) + (y - point_y).powi(2) + (z - point_z).powi(2)).sqrt();
                min_dist = min_dist.min(dist);
            }
        }
    }

    1.0 - min_dist.min(1.0)
}

#[inline]
fn cell_noise(x: f32, y: f32, z: f32) -> f32 {
    ((x * 12.9898 + y * 78.233 + z * 45.164).sin() * 43758.5453).fract()
}

// Turbulencia - Suma múltiples octavas de ruido
#[inline]
fn turbulence(p: Vec3, octaves: i32, noise_type: i32) -> f32 {
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
        freq *= 2.0;
        amp *= 0.5;
    }
    sum
}

// ========== FUNCIONES DE UTILIDAD ==========

#[inline]
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn mix_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    a * (1.0 - t) + b * t
}

// Conversión de temperatura a color (simulando cuerpo negro)
#[inline]
fn temperature_to_color(temp: f32) -> Vec3 {
    let t = temp.clamp(0.0, 1.0);

    if t < 0.33 {
        let factor = t / 0.33;
        mix_vec3(Vec3::new(1.0, 0.2, 0.0), Vec3::new(1.0, 0.5, 0.0), factor)
    } else if t < 0.66 {
        let factor = (t - 0.33) / 0.33;
        mix_vec3(Vec3::new(1.0, 0.5, 0.0), Vec3::new(1.0, 0.9, 0.3), factor)
    } else {
        let factor = (t - 0.66) / 0.34;
        mix_vec3(Vec3::new(1.0, 0.9, 0.3), Vec3::new(1.0, 1.0, 1.0), factor)
    }
}

// ========== SHADER 1: SOL CLÁSICO (PERLIN NOISE) ==========
pub struct ClassicSunShader;

impl StarShader for ClassicSunShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Turbulencia base usando Perlin Noise
        let turb_offset = Vec3::new(time * 0.1, time * 0.05, 0.0);
        let turbulence = turbulence(normalized_pos * 3.0 + turb_offset, 5, 0);

        // Manchas solares (áreas más oscuras)
        let spot_noise = perlin_noise(
            normalized_pos.x * 8.0 + time * 0.2,
            normalized_pos.y * 8.0,
            normalized_pos.z * 8.0,
        );
        let solar_spots = smoothstep(0.65, 0.75, spot_noise);

        // Temperatura base con variación
        let base_temp = 0.7 + turbulence * 0.15 - solar_spots * 0.3;
        let temp_color = temperature_to_color(base_temp);

        // Emisión de luz pulsante
        let pulse = (time * 2.0).sin() * 0.05 + 0.95;
        let emission = temp_color * (1.5 + turbulence * 0.5) * pulse;

        // Efecto de borde brillante (corona)
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let fresnel = (1.0 - normal.dot(&view_dir).abs()).powf(3.0);
        let corona = Vec3::new(1.0, 0.8, 0.3) * fresnel * 0.5;

        let final_color = (emission + corona).component_mul(&Vec3::new(1.2, 1.0, 0.8));
        Color::from_vec3(final_color)
    }
}

// ========== SHADER 2: PULSAR (SIMPLEX NOISE) ==========
pub struct PulsarShader;

impl StarShader for PulsarShader {
    fn fragment(&self, pos: &Vec3, _normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Pulsación principal
        let pulse_freq = 3.0;
        let pulse = ((time * pulse_freq).sin() * 0.5 + 0.5).powf(2.0);

        // Rotación de patrones usando Simplex
        let angle = time * 0.5;
        let rot_x = normalized_pos.x * angle.cos() - normalized_pos.z * angle.sin();
        let rot_z = normalized_pos.x * angle.sin() + normalized_pos.z * angle.cos();

        let pattern = simplex_noise(rot_x * 5.0, normalized_pos.y * 5.0, rot_z * 5.0);

        // Bandas rotatorias
        let bands = (normalized_pos.y * 10.0 + time * 2.0).sin() * 0.5 + 0.5;
        let combined = pattern * bands;

        // Color basado en intensidad
        let intensity = (combined + pulse) * 0.5;
        let hot_color = Vec3::new(0.2, 0.5, 1.0); // Azul caliente
        let cold_color = Vec3::new(0.8, 0.2, 1.0); // Púrpura
        let base_color = mix_vec3(cold_color, hot_color, intensity);

        // Emisión variable
        let emission = base_color * (2.0 + pulse * 1.5);

        // Picos de energía en los polos
        let pole_intensity = (1.0 - normalized_pos.y.abs()).powf(4.0);
        let pole_burst = Vec3::new(1.0, 1.0, 1.0) * pole_intensity * pulse * 2.0;

        let final_color = emission + pole_burst;
        Color::from_vec3(final_color)
    }
}

// ========== SHADER 3: ESTRELLA DE PLASMA (OPTIMIZADO) ==========
pub struct PlasmaStarShader;

impl StarShader for PlasmaStarShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Usar Simplex en lugar de Cellular para mejor performance
        let vortex1 = simplex_noise(
            normalized_pos.x * 4.0 + time * 0.3,
            normalized_pos.y * 4.0,
            normalized_pos.z * 4.0 + time * 0.2,
        );

        let vortex2 = simplex_noise(
            normalized_pos.x * 6.0 - time * 0.4,
            normalized_pos.y * 6.0 + time * 0.1,
            normalized_pos.z * 6.0,
        );

        let plasma_pattern = (vortex1 + vortex2 * 0.5) / 1.5;

        // Filamentos eléctricos (reducir frecuencia)
        let filaments = perlin_noise(
            normalized_pos.x * 10.0,
            normalized_pos.y * 10.0 + time * 2.0,
            normalized_pos.z * 10.0,
        );
        let filament_boost = smoothstep(0.6, 0.8, filaments) * 1.5;

        // Color iridiscente
        let hue = (plasma_pattern * 2.0 + time * 0.5) % 1.0;
        let plasma_color = if hue < 0.33 {
            mix_vec3(
                Vec3::new(1.0, 0.0, 0.5),
                Vec3::new(0.5, 0.0, 1.0),
                hue * 3.0,
            )
        } else if hue < 0.66 {
            mix_vec3(
                Vec3::new(0.5, 0.0, 1.0),
                Vec3::new(0.0, 1.0, 1.0),
                (hue - 0.33) * 3.0,
            )
        } else {
            mix_vec3(
                Vec3::new(0.0, 1.0, 1.0),
                Vec3::new(1.0, 0.0, 0.5),
                (hue - 0.66) * 3.0,
            )
        };

        // Emisión intensa
        let emission = plasma_color * (2.0 + plasma_pattern + filament_boost);

        // Borde eléctrico
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let edge = (1.0 - normal.dot(&view_dir).abs()).powf(2.0);
        let electric_edge = Vec3::new(0.5, 1.0, 1.0) * edge * (1.0 + (time * 10.0).sin() * 0.3);

        let final_color = emission + electric_edge;
        Color::from_vec3(final_color)
    }
}

// ========== SHADER 4: SUPERNOVA (MULTI-LAYER) ==========
pub struct SupernovaShader;

impl StarShader for SupernovaShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Expansión simulada
        let expansion = (time * 0.5).sin() * 0.2 + 1.0;
        let expanded_pos = normalized_pos * expansion;

        // Núcleo interno (Perlin)
        let core = turbulence(expanded_pos * 5.0, 4, 0);
        let core_color = temperature_to_color(0.9 + core * 0.1);

        // Capa media explosiva (Simplex)
        let explosion = turbulence(
            expanded_pos * 3.0 + Vec3::new(time * 0.2, time * 0.15, time * 0.1),
            5,
            1,
        );
        let explosion_color = Vec3::new(1.0, 0.6, 0.2) * (1.0 + explosion * 2.0);

        // Capa externa fragmentada (Cellular)
        let fragments = cellular_noise(
            expanded_pos.x * 8.0 + time * 0.3,
            expanded_pos.y * 8.0,
            expanded_pos.z * 8.0 + time * 0.4,
        );
        let fragment_color = Vec3::new(1.0, 0.3, 0.1) * fragments * 1.5;

        // Combinar capas
        let layer_mix = (normalized_pos.magnitude() + (time * 0.5).sin() * 0.3).fract();
        let mid_color = mix_vec3(core_color * 2.0, explosion_color, layer_mix);
        let final_blend = mix_vec3(mid_color, fragment_color, fragments * 0.4);

        // Flare extremo en los bordes
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let flare = (1.0 - normal.dot(&view_dir).abs()).powf(1.5);
        let flare_intensity = (time * 4.0).sin() * 0.3 + 0.7;
        let flare_color = Vec3::new(1.0, 0.9, 0.5) * flare * flare_intensity * 3.0;

        // Picos de energía radiantes
        let radial_burst = (time * 3.0 + normalized_pos.y * 10.0).sin() * 0.5 + 0.5;
        let burst_color = Vec3::new(1.0, 0.8, 0.3) * radial_burst * 0.5;

        let final_color = final_blend + flare_color + burst_color;
        Color::from_vec3(final_color)
    }
}
