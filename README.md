<img width="793" height="594" alt="imagen" src="https://github.com/user-attachments/assets/821b338d-8085-451e-bab3-54841a99f889" /># Star Shader Renderer (Laboratorio 5)

Este proyecto es un renderer 3D por software implementado en Rust. Su propósito es renderizar una esfera 3D y aplicarle diversos shaders procedurales que simulan diferentes tipos de estrellas. El renderer no utiliza la GPU para el rasterizado, sino que lo implementa en la CPU, escribiendo directamente en un framebuffer.

La visualización se gestiona con la librería `Raylib` para crear la ventana y mostrar el framebuffer resultante como una textura.

## Características

- **Renderer por Software:** Rasterización de triángulos, interpolación de vértices y cálculo de profundidad implementados desde cero.
- **Shaders Procedurales:** Incluye varios shaders para simular estrellas:
    1.  **Sol Clásico:** Utiliza ruido Perlin y turbulencia.
    2.  **Pulsar:** Combina ruido Simplex con una función de pulsación.
    3.  **Estrella de Plasma:** Generada con ruido celular y un efecto de vórtice.
    4.  **Supernova:** Simula una explosión con múltiples capas de ruido y destellos.
- **Carga de Modelos:** Soporta la carga de mallas desde archivos `.obj`. Si no se encuentra el archivo, se genera una esfera procedural por defecto.
- **Interacción en Tiempo Real:**
    - Cambiar entre diferentes shaders.
    - Pausar y reanudar la animación.
    - Control de zoom de la cámara.
    - Alternar entre el modelo `.obj` y la esfera procedural.

## Controles

-   **1-4:** Cambiar entre los diferentes shaders de estrella.
-   **M:** Alternar entre el modelo cargado de `sphere.obj` y la esfera procedural generada.
-   **ESPACIO:** Pausar o reanudar la animación de rotación y del shader.
-   **FLECHA ARRIBA / ABAJO:** Acercar o alejar la cámara.
-   **ESC:** Cerrar la aplicación.

## Instalación y Ejecución

Asegúrate de tener [Rust y Cargo instalados](https://www.rust-lang.org/tools/install).

**Ejecutar en modo de desarrollo:**
```bash
cargo run
```

**Compilar y ejecutar en modo release (optimizado):**
```bash
cargo run --release
```

## Dependencias

El proyecto utiliza las siguientes crates de Rust:

-   `raylib`: Para la gestión de la ventana, entrada del usuario y renderizado de la textura final.
-   `nalgebra-glm`: Para operaciones de álgebra lineal (vectores y matrices) compatibles con GLSL.
-   `tobj`: Para la carga de modelos 3D desde archivos `.obj`.

Estas dependencias se descargarán y compilarán automáticamente al ejecutar `cargo build` o `cargo run`.


<img width="790" height="590" alt="imagen" src="https://github.com/user-attachments/assets/5b2448d6-dff8-47a1-a91b-b55a20a03b07" />
<img width="798" height="597" alt="imagen" src="https://github.com/user-attachments/assets/f43fc8eb-78f4-4ae4-8523-218496f0d7f8" />
<img width="793" height="594" alt="imagen" src="https://github.com/user-attachments/assets/582258c2-03ce-4d08-b0dd-54dfe40232bb" />
<img width="793" height="594" alt="imagen" src="https://github.com/user-attachments/assets/3fe90933-6558-4ba4-8350-c5d5ba5f5c17" />



