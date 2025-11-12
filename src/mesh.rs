// Importa los tipos Vec2 y Vec3 de la biblioteca nalgebra_glm para manejar vectores de 2D y 3D.
use nalgebra_glm::{Vec3};
// Importa la constante PI para cálculos matemáticos.
use std::f32::consts::PI;

// Define la estructura de un vértice, que contiene su posición, normal y coordenadas de textura (UV).
#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3, // Posición del vértice en el espacio 3D.
    pub normal: Vec3,   // Vector normal del vértice, usado para la iluminación.
}

// Define una malla de objeto, que consiste en una lista de vértices y una lista de índices que forman las caras.
#[derive(Clone)]
pub struct ObjMesh {
    pub vertices: Vec<Vertex>, // Lista de todos los vértices en la malla.
    pub indices: Vec<u32>,     // Lista de índices que definen los triángulos de la malla.
}

impl ObjMesh {
    // Genera una esfera UV de manera procedural, con un manejo adecuado de los polos.
    pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Agrega el vértice del polo norte.
        vertices.push(Vertex {
            position: Vec3::new(0.0, radius, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        });

        // Genera los vértices intermedios de la esfera, excluyendo los polos.
        for r in 1..rings {
            for s in 0..=sectors {
                let theta = PI * r as f32 / rings as f32;
                let phi = 2.0 * PI * s as f32 / sectors as f32;

                let x = theta.sin() * phi.cos();
                let y = theta.cos();
                let z = theta.sin() * phi.sin();

                let position = Vec3::new(x * radius, y * radius, z * radius);
                let normal = Vec3::new(x, y, z);

                vertices.push(Vertex { position, normal});
            }
        }

        // Agrega el vértice del polo sur.
        vertices.push(Vertex {
            position: Vec3::new(0.0, -radius, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
        });

        // Genera los índices para los triángulos que conectan con el polo norte.
        for s in 0..sectors {
            indices.push(0); // Polo norte.
            indices.push(1 + s);
            indices.push(1 + s + 1);
        }

        // Genera los índices para las bandas de quads (dos triángulos) intermedias.
        for r in 0..(rings - 2) {
            for s in 0..sectors {
                let current = 1 + r * (sectors + 1) + s;
                let next = current + sectors + 1;

                indices.push(current);
                indices.push(next);
                indices.push(current + 1);

                indices.push(current + 1);
                indices.push(next);
                indices.push(next + 1);
            }
        }

        // Genera los índices para los triángulos que conectan con el polo sur.
        let south_pole_index = vertices.len() as u32 - 1;
        let last_ring_start = south_pole_index - (sectors + 1);

        for s in 0..sectors {
            indices.push(last_ring_start + s);
            indices.push(south_pole_index);
            indices.push(last_ring_start + s + 1);
        }

        ObjMesh { vertices, indices }
    }

    // Carga una malla desde un archivo en formato .obj.
    pub fn load_from_obj(path: &str) -> Result<Self, String> {
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
            .map_err(|e| format!("Error loading OBJ: {}", e))?;

        if models.is_empty() {
            return Err("No models found in OBJ file".to_string());
        }

        let mesh = &models[0].mesh;
        let mut vertices = Vec::new();

        for i in 0..mesh.positions.len() / 3 {
            let position = Vec3::new(
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            );

            let normal = if !mesh.normals.is_empty() {
                Vec3::new(
                    mesh.normals[i * 3],
                    mesh.normals[i * 3 + 1],
                    mesh.normals[i * 3 + 2],
                )
                .normalize()
            } else {
                position.normalize()
            };

            vertices.push(Vertex { position, normal});
        }

        Ok(ObjMesh {
            vertices,
            indices: mesh.indices.clone(),
        })
    }

}
