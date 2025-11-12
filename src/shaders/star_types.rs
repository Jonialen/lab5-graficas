//! `shaders/star_types.rs`
//!
//! Implementaciones de diferentes tipos de shaders de estrellas.

use crate::framebuffer::Color;
use nalgebra_glm::Vec3;

use super::noise::{cellular_noise, perlin_noise, simplex_noise, turbulence};
use super::utils::{hue_to_rgb, mix_vec3, pulse_pow, smoothstep, temperature_to_color};
use super::StarShader;

// ===================================================================================
// ========== SHADER 1: SOL CLÁSICO (PERLIN NOISE) ==========
// ===================================================================================

/// Un shader que simula la superficie turbulenta de una estrella similar al Sol.
///
/// Características:
/// - Usa ruido Perlin multi-octava para granulación solar
/// - Manchas solares oscuras dinámicas
/// - Sistema de temperatura con gradiente realista
/// - Pulsación suave sincronizada
/// - Corona brillante en los bordes (efecto Fresnel)
pub struct ClassicSunShader;

impl StarShader for ClassicSunShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Turbulencia base animada
        let turb_offset = Vec3::new(time * 0.1, time * 0.05, 0.0);
        let turbulence_val = turbulence(normalized_pos * 3.0 + turb_offset, 5, 0);

        // Manchas solares (áreas más frías y oscuras)
        let spot_noise = perlin_noise(
            normalized_pos.x * 8.0 + time * 0.2,
            normalized_pos.y * 8.0,
            normalized_pos.z * 8.0,
        );
        let solar_spots = smoothstep(0.65, 0.75, spot_noise);

        // Temperatura base con variación
        let base_temp = 0.7 + turbulence_val * 0.15 - solar_spots * 0.3;
        let temp_color = temperature_to_color(base_temp);

        // Emisión de luz pulsante
        let pulse = (time * 2.0).sin() * 0.05 + 0.95;
        let emission = temp_color * (1.5 + turbulence_val * 0.5) * pulse;

        // Efecto de corona brillante (Fresnel)
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let fresnel = (1.0 - normal.dot(&view_dir).abs()).powf(3.0);
        let corona = Vec3::new(1.0, 0.8, 0.3) * fresnel * 0.5;

        // Combina emisión y corona con tinte cálido
        let final_color = (emission + corona).component_mul(&Vec3::new(1.2, 1.0, 0.8));
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== SHADER 2: PÚLSAR (SIMPLEX NOISE) ==========
// ===================================================================================

/// Un shader que simula una estrella de neutrones en rápida rotación (púlsar).
///
/// Características:
/// - Pulsación rítmica intensa
/// - Patrones rotatorios usando Simplex Noise
/// - Bandas magnéticas animadas
/// - Jets de energía en los polos
/// - Colores azul-púrpura de alta energía
pub struct PulsarShader;

impl StarShader for PulsarShader {
    fn fragment(&self, pos: &Vec3, _normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Pulsación principal con curva exponencial
        let pulse = pulse_pow(time, 3.0, 2.0);

        // Rotación del sistema de coordenadas
        let angle = time * 0.5;
        let rot_x = normalized_pos.x * angle.cos() - normalized_pos.z * angle.sin();
        let rot_z = normalized_pos.x * angle.sin() + normalized_pos.z * angle.cos();

        // Patrón base usando Simplex Noise
        let pattern = simplex_noise(rot_x * 5.0, normalized_pos.y * 5.0, rot_z * 5.0);

        // Bandas de energía verticales
        let bands = (normalized_pos.y * 10.0 + time * 2.0).sin() * 0.5 + 0.5;
        let combined = pattern * bands;

        // Color interpolado entre azul caliente y púrpura frío
        let intensity = (combined + pulse) * 0.5;
        let hot_color = Vec3::new(0.2, 0.5, 1.0);
        let cold_color = Vec3::new(0.8, 0.2, 1.0);
        let base_color = mix_vec3(cold_color, hot_color, intensity);

        // Emisión variable con pulsación
        let emission = base_color * (2.0 + pulse * 1.5);

        // Jets de energía en los polos
        let pole_intensity = (1.0 - normalized_pos.y.abs()).powf(4.0);
        let pole_burst = Vec3::new(1.0, 1.0, 1.0) * pole_intensity * pulse * 2.0;

        let final_color = emission + pole_burst;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== SHADER 3: ESTRELLA DE PLASMA (VÓRTICES) ==========
// ===================================================================================

/// Un shader que simula una estrella de plasma caliente e inestable.
///
/// Características:
/// - Vórtices de plasma usando Simplex Noise
/// - Filamentos eléctricos procedurales
/// - Color iridiscente que cambia con el tiempo
/// - Bordes eléctricos pulsantes
/// - Múltiples capas de turbulencia
pub struct PlasmaStarShader;

impl StarShader for PlasmaStarShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Dos capas de vórtices con diferente escala y velocidad
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

        // Filamentos eléctricos de alta frecuencia
        let filaments = perlin_noise(
            normalized_pos.x * 10.0,
            normalized_pos.y * 10.0 + time * 2.0,
            normalized_pos.z * 10.0,
        );
        let filament_boost = smoothstep(0.6, 0.8, filaments) * 1.5;

        // Color iridiscente cíclico
        let hue = (plasma_pattern * 2.0 + time * 0.5) % 1.0;
        let plasma_color = hue_to_rgb(hue);

        // Emisión combinando plasma y filamentos
        let emission = plasma_color * (2.0 + plasma_pattern + filament_boost);

        // Borde eléctrico parpadeante
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let edge = (1.0 - normal.dot(&view_dir).abs()).powf(2.0);
        let electric_edge = Vec3::new(0.5, 1.0, 1.0) * edge * (1.0 + (time * 10.0).sin() * 0.3);

        let final_color = emission + electric_edge;
        Color::from_vec3(final_color)
    }
}

// ===================================================================================
// ========== SHADER 4: SUPERNOVA (MULTI-CAPA) ==========
// ===================================================================================

/// Un shader que simula la explosión catastrófica de una supernova.
///
/// Características:
/// - Combinación de Perlin, Simplex y Cellular Noise
/// - Simulación de expansión estelar
/// - Núcleo denso, capa explosiva y fragmentos externos
/// - Flares extremos con distorsión visual
/// - Picos de energía radiales
pub struct SupernovaShader;

impl StarShader for SupernovaShader {
    fn fragment(&self, pos: &Vec3, normal: &Vec3, time: f32) -> Color {
        let normalized_pos = pos.normalize();

        // Simulación de expansión de onda de choque
        let expansion = (time * 0.5).sin() * 0.2 + 1.0;
        let expanded_pos = normalized_pos * expansion;

        // CAPA 1: Núcleo interno denso (Perlin)
        let core = turbulence(expanded_pos * 5.0, 4, 0);
        let core_color = temperature_to_color(0.9 + core * 0.1);

        // CAPA 2: Explosión intermedia caótica (Simplex)
        let explosion = turbulence(
            expanded_pos * 3.0 + Vec3::new(time * 0.2, time * 0.15, time * 0.1),
            5,
            1,
        );
        let explosion_color = Vec3::new(1.0, 0.6, 0.2) * (1.0 + explosion * 2.0);

        // CAPA 3: Fragmentos externos eyectados (Cellular)
        let fragments = cellular_noise(
            expanded_pos.x * 8.0 + time * 0.3,
            expanded_pos.y * 8.0,
            expanded_pos.z * 8.0 + time * 0.4,
        );
        let fragment_color = Vec3::new(1.0, 0.3, 0.1) * fragments * 1.5;

        // Mezcla de capas con profundidad
        let layer_mix = (normalized_pos.magnitude() + (time * 0.5).sin() * 0.3).fract();
        let mid_color = mix_vec3(core_color * 2.0, explosion_color, layer_mix);
        let final_blend = mix_vec3(mid_color, fragment_color, fragments * 0.4);

        // Flare extremo en los bordes
        let view_dir = Vec3::new(0.0, 0.0, 1.0);
        let flare = (1.0 - normal.dot(&view_dir).abs()).powf(1.5);
        let flare_intensity = (time * 4.0).sin() * 0.3 + 0.7;
        let flare_color = Vec3::new(1.0, 0.9, 0.5) * flare * flare_intensity * 3.0;

        // Picos de energía radiales
        let radial_burst = (time * 3.0 + normalized_pos.y * 10.0).sin() * 0.5 + 0.5;
        let burst_color = Vec3::new(1.0, 0.8, 0.3) * radial_burst * 0.5;

        let final_color = final_blend + flare_color + burst_color;
        Color::from_vec3(final_color)
    }
}