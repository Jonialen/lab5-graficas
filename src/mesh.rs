//! Módulo de manejo de mallas 3D y vértices para renderizado software.
//
// Este archivo define las estructuras y funciones necesarias para la generación procedural de esferas y la carga de modelos desde archivos OBJ.

use nalgebra_glm::Vec3; // Vector 3D de la biblioteca nalgebra_glm.
use std::f32::consts::PI; // Constante PI para cálculos trigonométricos.

/// Representa un vértice en el espacio 3D, incluyendo su posición y normal.
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Posición del vértice en coordenadas 3D.
    pub position: Vec3,
    /// Vector normal del vértice, utilizado para iluminación y sombreado.
    pub normal: Vec3,
}

/// Estructura que representa una malla 3D compuesta por vértices e índices de triángulos.
#[derive(Clone)]
pub struct ObjMesh {
    /// Lista de vértices de la malla.
    pub vertices: Vec<Vertex>,
    /// Lista de índices que definen los triángulos de la malla.
    pub indices: Vec<u32>,
}

impl ObjMesh {
    /// Genera una esfera UV de forma procedural, manejando correctamente los polos.
    ///
    /// # Argumentos
    /// * `radius` - Radio de la esfera.
    /// * `rings` - Número de divisiones horizontales (latitud).
    /// * `sectors` - Número de divisiones verticales (longitud).
    ///
    /// # Retorna
    /// Una instancia de `ObjMesh` representando la esfera generada.
    pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Polo norte: único vértice superior.
        vertices.push(Vertex {
            position: Vec3::new(0.0, radius, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        });

        // Vértices intermedios (excluyendo polos), generados por anillos y sectores.
        for r in 1..rings {
            for s in 0..=sectors {
                let theta = PI * r as f32 / rings as f32; // Ángulo de latitud.
                let phi = 2.0 * PI * s as f32 / sectors as f32; // Ángulo de longitud.

                let x = theta.sin() * phi.cos();
                let y = theta.cos();
                let z = theta.sin() * phi.sin();

                let position = Vec3::new(x * radius, y * radius, z * radius);
                let normal = Vec3::new(x, y, z);

                vertices.push(Vertex { position, normal });
            }
        }

        // Polo sur: único vértice inferior.
        vertices.push(Vertex {
            position: Vec3::new(0.0, -radius, 0.0),
            normal: Vec3::new(0.0, -1.0, 0.0),
        });

        // Triángulos que conectan el polo norte con el primer anillo.
        for s in 0..sectors {
            indices.push(0); // Índice del polo norte.
            indices.push(1 + s);
            indices.push(1 + s + 1);
        }

        // Triángulos de los anillos intermedios (dos triángulos por quad).
        for r in 0..(rings - 2) {
            for s in 0..sectors {
                let current = 1 + r * (sectors + 1) + s;
                let next = current + sectors + 1;

                // Primer triángulo del quad.
                indices.push(current);
                indices.push(next);
                indices.push(current + 1);

                // Segundo triángulo del quad.
                indices.push(current + 1);
                indices.push(next);
                indices.push(next + 1);
            }
        }

        // Triángulos que conectan el último anillo con el polo sur.
        let south_pole_index = vertices.len() as u32 - 1;
        let last_ring_start = south_pole_index - (sectors + 1);

        for s in 0..sectors {
            indices.push(last_ring_start + s);
            indices.push(south_pole_index);
            indices.push(last_ring_start + s + 1);
        }

        ObjMesh { vertices, indices }
    }

    /// Carga una malla desde un archivo en formato OBJ.
    ///
    /// # Argumentos
    /// * `path` - Ruta al archivo .obj a cargar.
    ///
    /// # Retorna
    /// `Ok(ObjMesh)` si la carga fue exitosa, o un mensaje de error en caso contrario.
    pub fn load_from_obj(path: &str) -> Result<Self, String> {
        // Carga el archivo OBJ usando la biblioteca tobj.
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
            .map_err(|e| format!("Error loading OBJ: {}", e))?;

        if models.is_empty() {
            return Err("No models found in OBJ file".to_string());
        }

        let mesh = &models[0].mesh;
        let mut vertices = Vec::new();

        // Recorre los vértices del archivo y los convierte a la estructura interna.
        for i in 0..mesh.positions.len() / 3 {
            let position = Vec3::new(
                mesh.positions[i * 3],
                mesh.positions[i * 3 + 1],
                mesh.positions[i * 3 + 2],
            );

            // Si el archivo contiene normales, las usa; si no, normaliza la posición.
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

            vertices.push(Vertex { position, normal });
        }

        Ok(ObjMesh {
            vertices,
            indices: mesh.indices.clone(),
        })
    }
}
