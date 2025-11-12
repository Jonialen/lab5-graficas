//! Módulo de renderizado por software para mallas 3D.
//
// Este archivo implementa el pipeline de renderizado básico, incluyendo transformación de vértices,
// rasterización de triángulos y aplicación de shaders personalizados para cada fragmento.

use crate::framebuffer::Framebuffer; // Framebuffer para almacenar color y profundidad.
use crate::mesh::{ObjMesh, Vertex}; // Estructuras de malla y vértice.
use crate::shaders::StarShader; // Trait para shaders de fragmento personalizados.
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4}; // Tipos matemáticos para álgebra lineal.

/// Renderizador principal encargado de dibujar mallas 3D en el framebuffer.
pub struct Renderer {
    /// Ancho de la pantalla en píxeles.
    pub width: f32,
    /// Alto de la pantalla en píxeles.
    pub height: f32,
}

impl Renderer {
    /// Crea una nueva instancia del renderizador.
    ///
    /// # Argumentos
    /// * `width` - Ancho de la pantalla.
    /// * `height` - Alto de la pantalla.
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width: width as f32,
            height: height as f32,
        }
    }

    /// Renderiza una malla en el framebuffer usando un shader de fragmento.
    ///
    /// # Argumentos
    /// * `framebuffer` - Framebuffer destino.
    /// * `mesh` - Malla a renderizar.
    /// * `shader` - Shader de fragmento a aplicar.
    /// * `model_matrix` - Matriz de transformación del modelo.
    /// * `view_matrix` - Matriz de vista de la cámara.
    /// * `projection_matrix` - Matriz de proyección.
    /// * `time` - Tiempo actual para animaciones.
    pub fn render_mesh(
        &self,
        framebuffer: &mut Framebuffer,
        mesh: &ObjMesh,
        shader: &dyn StarShader,
        model_matrix: &Mat4,
        view_matrix: &Mat4,
        projection_matrix: &Mat4,
        time: f32,
    ) {
        // Calcula la matriz Modelo-Vista-Proyección (MVP).
        let mvp = projection_matrix * view_matrix * model_matrix;

        // Transforma todos los vértices de la malla al espacio de pantalla.
        let transformed_vertices: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| self.transform_vertex(v, model_matrix, &mvp))
            .collect();

        // Procesa cada triángulo de la malla usando los índices.
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            // Verifica que los índices sean válidos.
            if i0 < transformed_vertices.len()
                && i1 < transformed_vertices.len()
                && i2 < transformed_vertices.len()
            {
                // Rasteriza el triángulo formado por los tres vértices transformados.
                self.rasterize_triangle(
                    framebuffer,
                    &transformed_vertices[i0],
                    &transformed_vertices[i1],
                    &transformed_vertices[i2],
                    shader,
                    time,
                );
            }
        }
    }

    /// Transforma un vértice del espacio de modelo al espacio de pantalla.
    ///
    /// # Argumentos
    /// * `vertex` - Vértice original.
    /// * `model_matrix` - Matriz de modelo.
    /// * `mvp` - Matriz Modelo-Vista-Proyección.
    ///
    /// # Retorna
    /// Un `TransformedVertex` con la posición en pantalla, profundidad y atributos interpolables.
    fn transform_vertex(
        &self,
        vertex: &Vertex,
        model_matrix: &Mat4,
        mvp: &Mat4,
    ) -> TransformedVertex {
        let pos4 = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

        // Calcula la posición y normal en espacio mundo.
        let world_pos = model_matrix * pos4;
        let normal4 = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
        let world_normal = (model_matrix * normal4).xyz().normalize();

        // Proyecta al espacio de recorte (clip space).
        let clip_pos = mvp * pos4;

        // División de perspectiva para obtener NDC.
        let w = clip_pos.w;
        if w.abs() < 1e-6 {
            // Descarta vértices problemáticos.
            return TransformedVertex {
                screen_pos: Vec2::new(-1000.0, -1000.0),
                depth: 1.0,
                world_pos: world_pos.xyz(),
                world_normal,
            };
        }
        let ndc = clip_pos.xyz() / w;

        // Convierte NDC a coordenadas de pantalla.
        let screen = Vec2::new(
            (ndc.x + 1.0) * 0.5 * self.width,
            (1.0 - ndc.y) * 0.5 * self.height, // Y invertida.
        );

        TransformedVertex {
            screen_pos: screen,
            depth: ndc.z,
            world_pos: world_pos.xyz(),
            world_normal,
        }
    }

    /// Rasteriza un triángulo interpolando atributos y aplicando el shader de fragmento.
    ///
    /// # Argumentos
    /// * `framebuffer` - Framebuffer destino.
    /// * `v0`, `v1`, `v2` - Vértices transformados del triángulo.
    /// * `shader` - Shader de fragmento.
    /// * `time` - Tiempo actual para animaciones.
    fn rasterize_triangle(
        &self,
        framebuffer: &mut Framebuffer,
        v0: &TransformedVertex,
        v1: &TransformedVertex,
        v2: &TransformedVertex,
        shader: &dyn StarShader,
        time: f32,
    ) {
        // Calcula el bounding box del triángulo para limitar el área de rasterización.
        let min_x = v0
            .screen_pos
            .x
            .min(v1.screen_pos.x)
            .min(v2.screen_pos.x)
            .floor()
            .max(0.0) as usize;
        let max_x = v0
            .screen_pos
            .x
            .max(v1.screen_pos.x)
            .max(v2.screen_pos.x)
            .ceil()
            .min(self.width - 1.0) as usize;
        let min_y = v0
            .screen_pos
            .y
            .min(v1.screen_pos.y)
            .min(v2.screen_pos.y)
            .floor()
            .max(0.0) as usize;
        let max_y = v0
            .screen_pos
            .y
            .max(v1.screen_pos.y)
            .max(v2.screen_pos.y)
            .ceil()
            .min(self.height - 1.0) as usize;

        // Recorre cada píxel dentro del bounding box.
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                // Calcula coordenadas baricéntricas para interpolación.
                let (w0, w1, w2) = barycentric(&p, &v0.screen_pos, &v1.screen_pos, &v2.screen_pos);

                // Si el píxel está dentro del triángulo.
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // Interpola profundidad, posición y normal.
                    let depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;
                    let world_pos = v0.world_pos * w0 + v1.world_pos * w1 + v2.world_pos * w2;
                    let world_normal =
                        (v0.world_normal * w0 + v1.world_normal * w1 + v2.world_normal * w2)
                            .normalize();

                    // Aplica el shader de fragmento para obtener el color final.
                    let color = shader.fragment(&world_pos, &world_normal, time);

                    // Escribe el píxel en el framebuffer con prueba de profundidad.
                    framebuffer.set_pixel(x, y, color, depth);
                }
            }
        }
    }
}

/// Estructura auxiliar para almacenar los atributos interpolables de un vértice transformado.
struct TransformedVertex {
    /// Posición en pantalla (2D).
    screen_pos: Vec2,
    /// Profundidad (Z en NDC).
    depth: f32,
    /// Posición en espacio mundo (3D).
    world_pos: Vec3,
    /// Normal en espacio mundo (3D).
    world_normal: Vec3,
}

/// Calcula las coordenadas baricéntricas de un punto respecto a un triángulo.
///
/// # Argumentos
/// * `p` - Punto a evaluar.
/// * `a`, `b`, `c` - Vértices del triángulo.
///
/// # Retorna
/// Tupla con los pesos baricéntricos (u, v, w).
#[inline]
fn barycentric(p: &Vec2, a: &Vec2, b: &Vec2, c: &Vec2) -> (f32, f32, f32) {
    let v0 = *b - *a;
    let v1 = *c - *a;
    let v2 = *p - *a;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);

    let denom = d00 * d11 - d01 * d01;

    if denom.abs() < 1e-8 {
        // Triángulo degenerado.
        return (0.0, 0.0, 0.0);
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}
