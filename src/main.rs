mod framebuffer;
mod mesh;
mod renderer;
mod shaders;

use framebuffer::{Color, Framebuffer};
use mesh::ObjMesh;
use nalgebra_glm::{Mat4, Vec3, look_at, perspective, rotate};
use raylib::prelude::*;
use renderer::Renderer;
use shaders::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct RenderObject {
    mesh: ObjMesh,
    shader: Box<dyn StarShader>,
    position: Vec3,
    scale: f32,
    rotation_speed: f32,
    rotation_axis: Vec3,
}

impl RenderObject {
    fn new(mesh: ObjMesh, shader: Box<dyn StarShader>, position: Vec3, scale: f32) -> Self {
        RenderObject {
            mesh,
            shader,
            position,
            scale,
            rotation_speed: 0.3,
            rotation_axis: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    fn get_model_matrix(&self, time: f32) -> Mat4 {
        let mut transform = Mat4::identity();
        transform = nalgebra_glm::translate(&transform, &self.position);
        transform = rotate(&transform, time * self.rotation_speed, &self.rotation_axis);
        transform = nalgebra_glm::scale(&transform, &Vec3::new(self.scale, self.scale, self.scale));
        transform
    }
}

fn main() {
    println!("=== Iniciando Star Shader Renderer ===");

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Star Shader - Software Renderer")
        .build();

    rl.set_target_fps(60);

    println!("Generando esfera procedural...");
    let sphere_mesh = ObjMesh::create_sphere(1.0, 64, 64);

    let obj_sphere = match ObjMesh::load_from_obj("assets/sphere.obj") {
        Ok(mesh) => {
            println!("✓ sphere.obj cargado exitosamente");
            Some(mesh)
        }
        Err(e) => {
            println!("⚠ No se pudo cargar sphere.obj: {}", e);
            println!("  Usando esfera procedural");
            None
        }
    };

    let mut use_obj_model = obj_sphere.is_some();

    let get_sphere = |use_obj: bool| -> ObjMesh {
        if use_obj && obj_sphere.is_some() {
            obj_sphere.as_ref().unwrap().clone()
        } else {
            sphere_mesh.clone()
        }
    };

    let create_star = |use_obj: bool, shader_type: usize| -> RenderObject {
        let current_sphere = get_sphere(use_obj);

        let shader: Box<dyn StarShader> = match shader_type {
            0 => Box::new(ClassicSunShader),
            1 => Box::new(PulsarShader),
            2 => Box::new(PlasmaStarShader),
            3 => Box::new(SupernovaShader),
            _ => Box::new(ClassicSunShader),
        };

        RenderObject::new(current_sphere, shader, Vec3::new(0.0, 0.0, 0.0), 1.5)
    };

    let shader_names = vec![
        "1: Sol Clásico (Perlin + Turbulence)",
        "2: Pulsar (Simplex + Pulsación)",
        "3: Estrella de Plasma (Cellular + Vortex)",
        "4: Supernova (Multi-layer + Flare)",
    ];

    let mut current_shader = 0;
    let mut star = create_star(use_obj_model, current_shader);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let renderer = Renderer::new(WIDTH, HEIGHT);

    println!("Creando textura...");
    let initial_image =
        Image::gen_image_color(WIDTH as i32, HEIGHT as i32, raylib::color::Color::BLACK);

    let mut texture = rl
        .load_texture_from_image(&thread, &initial_image)
        .expect("No se pudo crear textura");

    let mut paused = false;
    let mut paused_time = 0.0f32;
    let mut last_active_time = 0.0f32;
    let mut camera_distance = 3.5f32;

    println!("=== Entrando al loop principal ===\n");
    println!("Controles:");
    println!("  1-4: Cambiar shader");
    println!("  M: Toggle modelo .obj / procedural");
    println!("  SPACE: Pausar");
    println!("  UP/DOWN: Zoom cámara");
    println!("  ESC: Salir\n");

    while !rl.window_should_close() {
        let current_real_time = rl.get_time() as f32;

        let time = if paused {
            paused_time
        } else {
            last_active_time + (current_real_time - last_active_time)
        };

        // Cambio de shader
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_shader = 0;
            star = create_star(use_obj_model, current_shader);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_shader = 1;
            star = create_star(use_obj_model, current_shader);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_shader = 2;
            star = create_star(use_obj_model, current_shader);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_shader = 3;
            star = create_star(use_obj_model, current_shader);
        }

        // Toggle modelo
        if rl.is_key_pressed(KeyboardKey::KEY_M) && obj_sphere.is_some() {
            use_obj_model = !use_obj_model;
            star = create_star(use_obj_model, current_shader);
            println!(
                "Cambiando a: {}",
                if use_obj_model {
                    "sphere.obj"
                } else {
                    "Esfera Procedural"
                }
            );
        }

        // Pausa
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if paused {
                let pause_duration = current_real_time - paused_time;
                last_active_time = current_real_time - pause_duration;
                paused = false;
            } else {
                paused_time = time;
                paused = true;
            }
        }

        // Control de cámara
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            camera_distance -= 0.02;
            camera_distance = camera_distance.max(2.0);
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            camera_distance += 0.02;
            camera_distance = camera_distance.min(10.0);
        }

        if !paused {
            last_active_time = time;
        }

        let view_matrix = look_at(
            &Vec3::new(0.0, 0.0, camera_distance),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        );

        let projection_matrix = perspective(
            WIDTH as f32 / HEIGHT as f32,
            60.0_f32.to_radians(),
            0.1,
            100.0,
        );

        framebuffer.clear(Color::new(5, 5, 15));

        let model_matrix = star.get_model_matrix(time);

        renderer.render_mesh(
            &mut framebuffer,
            &star.mesh,
            star.shader.as_ref(),
            &model_matrix,
            &view_matrix,
            &projection_matrix,
            time,
        );

        if let Err(e) = texture.update_texture(framebuffer.as_bytes()) {
            eprintln!("Error actualizando textura: {:?}", e);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(raylib::color::Color::new(5, 5, 15, 255));
        d.draw_texture(&texture, 0, 0, raylib::color::Color::WHITE);

        d.draw_fps(10, 10);

        let status = if paused { " [PAUSADO]" } else { "" };
        d.draw_text(
            &format!("{}{}", shader_names[current_shader], status),
            10,
            35,
            20,
            raylib::color::Color::WHITE,
        );

        let mesh_type = if use_obj_model {
            "Modelo: sphere.obj"
        } else {
            "Modelo: Procedural"
        };
        d.draw_text(mesh_type, 10, 60, 16, raylib::color::Color::YELLOW);

        let controls = if obj_sphere.is_some() {
            "1-4: Shaders | M: Modelo | SPACE: Pausa | ↑↓: Zoom | ESC: Salir"
        } else {
            "1-4: Shaders | SPACE: Pausa | ↑↓: Zoom | ESC: Salir"
        };

        d.draw_text(
            controls,
            10,
            HEIGHT as i32 - 25,
            14,
            raylib::color::Color::LIGHTGRAY,
        );
    }

    println!("=== Cerrando aplicación ===");
}
