# Super Smash Bros Rust — Proyecto 2

**Autor:** Martin Villatoro<br>
**Carné:** 24033<br>
**Curso:** Gráficas por Computadora<br>
**Proyecto:** Diorama con Raytracing

## Demostración

> **Pendiente:** insertar aquí el video final de demostración del proyecto.

<!-- Reemplazar el enlace cuando el video esté subido a GitHub, Drive o YouTube. -->

[Ver video de demostración](PEGAR_AQUI_EL_ENLACE_DEL_VIDEO)

El video debe mostrar el flujo completo: pantalla de inicio, selección de personaje,
entrada a un diorama, movimiento de cámara y los efectos dinámicos de las escenas.

## Descripción

Aplicación interactiva en tiempo real inspirada en los ocho personajes iniciales de
Super Smash Bros. La aplicación permite seleccionar un personaje y explorar su
diorama 3D mediante una cámara orbital. Cada escenario utiliza una combinación
distinta de geometría, materiales, texturas, iluminación y animaciones.

Los ocho dioramas son:

| Personaje | Diorama y elementos principales |
|---|---|
| Mario | Isla flotante del Reino Champiñón, castillo, camino, tuberías, bloques y monedas animadas. |
| Donkey Kong | Casa de jungla con techo de paja, letrero DK, palmeras, arbustos, bananas y hojas animadas. |
| Link | Árbol Deku, espada clavada en una roca, arbustos, rupias animadas y Navi con iluminación local. |
| Samus | Isla industrial, torres de energía, luces de piso, barriles con líquido fosforescente, huevos alienígenas y Ridley animado. |
| Yoshi | Isla natural con nido, huevo ovalado texturizado, flores, arbustos y movimiento suave de levitación. |
| Kirby | Arena espacial con agua reflectiva/refractiva, cascadas, gotas, espuma, anillos y estrella flotante luminosa. |
| Fox | Nave Arwing modelada con geometría 3D, alas, cabina, aleta dorsal y propulsores emisivos. |
| Pikachu | Pokémon Stadium con plataforma metálica, campo, Poké Ball, rayos, fuego y rocas temblando. |

## Aspectos técnicos destacados

### Raytracer propio en CPU

- Cada pixel genera un **primary ray** desde la cámara hacia la escena.
- Las intersecciones se calculan directamente contra primitivas implementadas en el proyecto.
- El shading combina iluminación difusa, especularidad, sombras, emisión y color de textura.
- La imagen se renderiza en un framebuffer de `320 × 180` y se escala en la ventana para conservar un flujo interactivo.
- No se utiliza un game engine, un modelo 3D externo ni una librería de rendering.

### Reflection, refraction y Fresnel

El renderer implementa rayos secundarios para materiales reflectivos y transparentes:

- **Reflection** mediante la dirección reflejada respecto a la normal.
- **Refraction** mediante la ley de Snell.
- **Fresnel-Schlick** para controlar cuánto se refleja una superficie dependiendo del ángulo de visión.
- Detección de **total internal reflection** cuando la refracción no es físicamente posible.
- Profundidad máxima de `2` bounces para conservar rendimiento en tiempo real.

Estos efectos se utilizan en agua, cristales, vitrales, rupias, estrellas, metales,
tuberías y otros materiales especiales.

### Material system

Cada `Material` contiene parámetros independientes para controlar su respuesta a la luz:

- `albedo`
- `specular`
- `transparency`
- `reflectivity`
- `refractive_index`
- `emission`
- textura asociada

Se combinan materiales con texturas PPM externas, colores sólidos y texturas
procedurales. Algunos ejemplos son piedra, grama, tierra, metal, cristal, agua,
vidrio, tubería, moneda, lava, corteza, energía azul, líquido verde fosforescente,
rupias y huevo de Yoshi.

### Textures

El proyecto utiliza dos tipos de textura:

1. **External PPM textures:** pantalla de inicio, selección de personajes, moneda y vitral.
2. **Procedural textures:** piedra, grama, techo, tierra, tubería, cristal, lava,
   bambú, paja, corteza, huevo de Yoshi y agua.

Las texturas se muestrean durante el shading usando las coordenadas UV entregadas
por cada intersección.

### Geometría 3D implementada

Además de cubos, esferas y cilindros, se implementaron primitivas especializadas para
crear formas reconocibles sin importar modelos externos:

- `Ellipsoid` para huevos, cuerpos, propulsores y elementos orgánicos.
- `VerticalCylinder` y `OrientedCylinder` para torres, troncos, tuberías y cuellos.
- `HalfCylinder` para construir los dos hemisferios de la Poké Ball.
- Prismas triangulares y prismas inclinados para rayos, alas, techos y estructuras.
- `LongitudinalTriangularPrism` para la aleta dorsal de la nave de Fox.
- Estrella de cinco puntas para el escenario de Kirby.
- Cubos con dimensiones independientes para plataformas, castillos y estructuras.

### Animation system

Las animaciones se actualizan con tiempo real mediante una `AnimationClock` y funciones
trigonométricas suaves. Los objetos estáticos se conservan separados de los
`dynamic_objects`, que se reconstruyen en cada frame.

Ejemplos técnicos:

- Monedas de Mario y rupias de Link con movimiento vertical.
- Navi orbitando la isla de Link y funcionando como luz local.
- Hojas de palmeras de Donkey Kong oscilando.
- Líquido verde de Samus con pulso de emisión.
- Alas de Ridley con movimiento periódico.
- Huevo de Yoshi con sway suave.
- Estrella de Kirby levitando y emitiendo luz.
- Cascadas de Kirby con variación de ancho, gotas descendentes y salpicaduras ascendentes.
- Propulsores de Fox con emisión pulsante.
- Fuego, rayos y rocas temblando en el escenario de Pikachu.

### Camera system

La cámara es orbital y permite visualizar la profundidad real de cada diorama:

- Rotación horizontal con `Left/Right` o `A/D`.
- Inclinación vertical con `Up/Down`.
- Zoom con `W/S`.
- Reinicio de cámara con `R`.

## Relación con la rúbrica

| Criterio | Implementación |
|---|---|
| Complejidad de la escena | Ocho dioramas 3D temáticos con geometría, composición y elementos animados propios. |
| Atractivo visual | Estilos diferenciados: Reino Champiñón, jungla, noche de Zelda, zona industrial, naturaleza, espacio, nave y estadio. |
| Rotación y zoom | Cámara orbital interactiva con yaw, pitch y distance configurables. |
| Materiales | Más de cinco materiales diferenciados, cada uno con textura y parámetros propios de shading. |
| Refracción | Agua de Kirby, cristal y vitral con transparencia, índice de refracción y rayos refractados. |
| Reflexión | Agua, metales, tuberías, cristales, rupias y otros materiales con rayos reflejados. |
| Skybox / background | Fondos procedurales dependientes de la dirección del rayo: cielo, noche, espacio, lava y Stadium iluminado. |

## Arquitectura del código

```text
src/
├── renderer.rs    Raytracing, shading, shadows, reflection y refraction
├── scene.rs       Escena, objetos estáticos y objetos dinámicos
├── diorama.rs     Construcción y animación de los ocho escenarios
├── material.rs    MaterialLibrary y parámetros de materiales
├── texture.rs     Texturas PPM y texturas procedurales
├── cube.rs        Intersección con cubos
├── sphere.rs      Esferas y elipsoides
├── cylinder.rs    Cilindros verticales, orientados y semicilindros
├── prism.rs       Prismas triangulares y formas inclinadas
├── star.rs        Estrella de cinco puntas
├── camera.rs      Cámara orbital y controles de zoom
├── ui.rs          Title screen y character select
└── main.rs        Ventana, input loop y estados de la aplicación
```

## Stack

- Rust 2021
- `minifb 0.26.0`: ventana, input y presentación del framebuffer.
- `nalgebra-glm 0.18.0`: vectores, normales y operaciones matemáticas.
- Implementación propia de raytracing, materiales, primitivas, framebuffer y escenas.

## Ejecutar

Se requiere Rust y Cargo instalados.

```bash
cargo run --release
```

## Controles

### Title screen

- `Enter` o `Space`: abrir selección de personaje.
- `Escape`: salir.

### Character select

- Flechas o `W/A/S/D`: mover la selección.
- `Enter`: abrir el diorama seleccionado.
- `Escape`: regresar al title screen.

### Diorama

- `Left/Right` o `A/D`: rotar la cámara.
- `Up/Down`: inclinar la cámara.
- `W/S`: acercar o alejar la cámara.
- `R`: resetear la cámara.
- `Escape`: regresar a selección de personaje.

## Restricciones respetadas

El proyecto no utiliza Macroquad ni un engine 3D. La geometría, el renderer, los
materiales, las intersecciones, las animaciones, la cámara y el framebuffer fueron
implementados dentro del proyecto. Las únicas dependencias son las utilizadas para
crear la ventana/recibir input y realizar álgebra vectorial.
