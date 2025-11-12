// Importa los módulos y tipos necesarios de otros archivos del proyecto y de la biblioteca nalgebra_glm.
use crate::framebuffer::Framebuffer; // Para interactuar con el búfer de fotogramas.
use crate::mesh::{ObjMesh, Vertex}; // Para usar las estructuras de mallas y vértices.
use crate::shaders::StarShader; // Para usar el trait de sombreado de estrellas.
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4}; // Para operaciones matemáticas con vectores y matrices.

// Define el renderizador, que se encarga de dibujar las mallas en el búfer de fotogramas.
pub struct Renderer {
    pub width: f32,  // Ancho de la pantalla.
    pub height: f32, // Alto de la pantalla.
}

impl Renderer {
    // Crea una nueva instancia del renderizador.
    pub fn new(width: usize, height: usize) -> Self {
        Renderer {
            width: width as f32,
            height: height as f32,
        }
    }

    // Renderiza una malla en el búfer de fotogramas usando un sombreador específico.
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
        // Calcula la matriz Modelo-Vista-Proyección (MVP) para transformar los vértices.
        let mvp = projection_matrix * view_matrix * model_matrix;

        // Transforma cada vértice de la malla del espacio del objeto al espacio de la pantalla.
        let transformed_vertices: Vec<_> = mesh
            .vertices
            .iter()
            .map(|v| self.transform_vertex(v, model_matrix, &mvp))
            .collect();

        // Itera sobre los índices de la malla para procesar cada triángulo.
        for i in (0..mesh.indices.len()).step_by(3) {
            let i0 = mesh.indices[i] as usize;
            let i1 = mesh.indices[i + 1] as usize;
            let i2 = mesh.indices[i + 2] as usize;

            // Se asegura de que los índices sean válidos.
            if i0 < transformed_vertices.len()
                && i1 < transformed_vertices.len()
                && i2 < transformed_vertices.len()
            {
                // Rasteriza el triángulo formado por los tres vértices.
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

    // Transforma un solo vértice del espacio del modelo al espacio de la pantalla.
    fn transform_vertex(
        &self,
        vertex: &Vertex,
        model_matrix: &Mat4,
        mvp: &Mat4,
    ) -> TransformedVertex {
        let pos4 = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

        // Calcula la posición y la normal en el espacio del mundo.
        let world_pos = model_matrix * pos4;
        let normal4 = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
        let world_normal = (model_matrix * normal4).xyz().normalize();

        // Proyecta el vértice al espacio de recorte (clip space).
        let clip_pos = mvp * pos4;

        // Realiza la división de perspectiva para obtener las coordenadas normalizadas del dispositivo (NDC).
        let w = clip_pos.w;
        if w.abs() < 1e-6 {
            // Evita la división por cero y descarta vértices problemáticos.
            return TransformedVertex {
                screen_pos: Vec2::new(-1000.0, -1000.0),
                depth: 1.0,
                world_pos: world_pos.xyz(),
                world_normal,
            };
        }
        let ndc = clip_pos.xyz() / w;

        // Convierte las coordenadas NDC al espacio de la pantalla.
        let screen = Vec2::new(
            (ndc.x + 1.0) * 0.5 * self.width,
            (1.0 - ndc.y) * 0.5 * self.height, // Se invierte la coordenada Y.
        );

        TransformedVertex {
            screen_pos: screen,
            depth: ndc.z,
            world_pos: world_pos.xyz(),
            world_normal,
        }
    }

    // Rasteriza un triángulo, dibujando los píxeles que lo componen en el búfer de fotogramas.
    fn rasterize_triangle(
        &self,
        framebuffer: &mut Framebuffer,
        v0: &TransformedVertex,
        v1: &TransformedVertex,
        v2: &TransformedVertex,
        shader: &dyn StarShader,
        time: f32,
    ) {
        // Calcula el cuadro delimitador (bounding box) del triángulo para optimizar el recorrido de píxeles.
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

        // Itera sobre cada píxel dentro del cuadro delimitador.
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);

                // Calcula las coordenadas baricéntricas del píxel actual.
                let (w0, w1, w2) = barycentric(&p, &v0.screen_pos, &v1.screen_pos, &v2.screen_pos);

                // Si el píxel está dentro del triángulo, lo procesa.
                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    // Interpola la profundidad, la posición en el mundo y la normal del vértice.
                    let depth = w0 * v0.depth + w1 * v1.depth + w2 * v2.depth;
                    let world_pos = v0.world_pos * w0 + v1.world_pos * w1 + v2.world_pos * w2;
                    let world_normal =
                        (v0.world_normal * w0 + v1.world_normal * w1 + v2.world_normal * w2)
                            .normalize();

                    // Llama al sombreador de fragmentos para obtener el color del píxel.
                    let color = shader.fragment(&world_pos, &world_normal, time);

                    // Dibuja el píxel en el búfer de fotogramas, realizando la prueba de profundidad.
                    framebuffer.set_pixel(x, y, color, depth);
                }
            }
        }
    }
}

// Estructura auxiliar para almacenar los datos de un vértice después de ser transformado.
struct TransformedVertex {
    screen_pos: Vec2,   // Posición en el espacio de la pantalla.
    depth: f32,         // Profundidad del vértice (coordenada Z en NDC).
    world_pos: Vec3,    // Posición en el espacio del mundo.
    world_normal: Vec3, // Normal en el espacio del mundo.
}

// Calcula las coordenadas baricéntricas de un punto `p` con respecto a un triángulo (a, b, c).
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
        return (0.0, 0.0, 0.0); // Triángulo degenerado.
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    (u, v, w)
}
