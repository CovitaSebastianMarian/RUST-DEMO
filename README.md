# 🎮 Rust Graphics Engine Demo

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![OpenGL](https://img.shields.io/badge/OpenGL-%23FFFFFF.svg?style=for-the-badge&logo=opengl)](https://www.opengl.org/)

> 🌟 **Demo de vară 2025** - Explorare grafică 3D în Rust la nivel low-level

Acest proiect reprezintă o colecție de demonstrații grafice 3D dezvoltate vara aceasta în Rust, folosind **OpenGL** și **GLFW** la nivel low-level (fără engine-uri high-level). Proiectul explorează concepte fundamentale de grafică computerizată, de la shadow mapping până la sisteme de particule și interfețe custom.

---

## 📋 Cerințe de sistem

Înainte de a rula proiectul, asigurați-vă că aveți instalate:

- ✅ **Rust** (ultima versiune stabilă) - [Instalare Rust](https://rustup.rs/)
- ✅ **CMake** - Necesar pentru build-ul dependențelor native
  - **Windows**: [Descarcă CMake](https://cmake.org/download/)

---

## 🚀 Cum se rulează

Proiectul conține **6 demonstrații diferite**, fiecare ilustrând aspecte diferite ale programării grafice.

### Rulare demo-uri:
```bash
# Demo 1
cargo run --features demo1

# Demo 2
cargo run --features demo2

# Demo 3
cargo run --features demo3

# Demo 4
cargo run --features demo4

# Demo 5
cargo run --features demo5

# Demo 6
cargo run --features demo6
```

---

## 🎯 Demo-uri disponibile

### 🔦 Demo 1: Shadow Mapping
Implementare de shadow mapping cu framebuffer offscreen și depth texture.

**Controale:**
- `TAB` - Intrare/Ieșire din modul player
- `W/A/S/D` - Mișcare cameră
- `Mouse` - Rotire cameră
- `ESC` - Închidere program

---

### 🌍 Demo 2: Gravitație Planetară
Simulare de gravitație pe suprafața unei planete cu physics custom.

**Controale:**
- `TAB` - Intrare/Ieșire din modul player
- `ENTER` - Intrare/Ieșire din modul gravitație planetă
- `W/A/S/D` - Mișcare
- `SPACE` - Săritură pe planetă
- `Mouse` - Rotire cameră
- `ESC` - Închidere program

---

### 🔫 Demo 3: First Person Shooter
Prototip FPS cu animații de arme și mecanism de tragere.

**Controale:**
- `TAB` - Intrare/Ieșire din modul player
- `W/A/S/D` - Mișcare
- `Mouse` - Rotire cameră
- `Click stâng` - Tragere pistol
- `ESC` - Închidere program

---

### 🪟 Demo 4: Custom GUI System
Prototip de sistem de ferestre tip ImGui (Work in Progress).

**Controale:**
- `ESC` - Închidere program

> **Notă:** Acesta este un experiment de creare a unui sistem de UI custom. Încă în dezvoltare!

---

### 🎨 Demo 5: [Descriere demo 5]
*[Adaugă aici descrierea demo-ului 5]*

---

### ⚡ Demo 6: [Descriere demo 6]
*[Adaugă aici descrierea demo-ului 6]*

---

## 🛠️ Tehnologii utilizate

- **Limbaj**: Rust 🦀
- **Graphics API**: OpenGL (raw bindings via `gl` crate)
- **Windowing**: GLFW
- **Math**: `nalgebra-glm` pentru operații matematice 3D
- **Low-level**: Implementare directă cu unsafe Rust pentru performanță maximă

### De ce low-level?

Acest proiect evită în mod intenționat engine-urile high-level pentru a înțelege:
- 📐 Cum funcționează pipeline-ul grafic la nivel fundamental
- 🎬 Managementul manual al resurselor OpenGL (buffers, textures, shaders)
- ⚡ Optimizări de performanță prin control direct
- 🔧 Integrarea Rust cu API-uri C native

---

## 📂 Structura proiectului
```
│   Cargo.lock
│   Cargo.toml
│
├───assets
│   │   grid.glb
│   │   Heightmap.png
│   │   heightmap2.jpg
│   │   image.png
│   │   ManufacturingConsent-Regular.ttf
│   │   masina.glb
│   │   Roboto-VariableFont_wdth,wght.ttf
│   │   Roboto_Condensed-Black.ttf
│   │   skybox1.jpg
│   │   skybox2.jpg
│   │   Terrain.png
│   │   TerrainHeightMap.png
│   │   worldheightmap.png
│   │
│   ├───blackhole
│   │       fragment.glsl
│   │       fragment2.glsl
│   │       noise.png
│   │       vertex.glsl
│   │
│   ├───model
│   │   │   1911.glb
│   │   │   anim.glb
│   │   │   casa.glb
│   │   │   fragment.glsl
│   │   │   gun.glb
│   │   │   gun2.glb
│   │   │   harta1.glb
│   │   │   heli.glb
│   │   │   helicopter.glb
│   │   │   map.glb
│   │   │   map2.glb
│   │   │   map3.glb
│   │   │   monkey.glb
│   │   │   vertex.glsl
│   │   │
│   │   └───shaders
│   │           frag.glsl
│   │           fragment.glsl
│   │           vert.glsl
│   │           vertex.glsl
│   │
│   ├───planet
│   │       worldgen1.jpg
│   │       worldgen2.jpg
│   │       worldgen3.jpg
│   │
│   ├───skybox
│   │       back.jpg
│   │       bottom.jpg
│   │       front.jpg
│   │       left.jpg
│   │       right.jpg
│   │       top.jpg
│   │
│   └───spaceskybox
│           back.png
│           bottom.png
│           cubemap.png
│           front.png
│           left.png
│           right.png
│           top.png
│
└───src
    │   main.rs
    │
    └───seb
        │   collision.rs
        │   gltfmodel.rs
        │   mod.rs
        │   model.rs
        │   planet.rs
        │   player.rs
        │   primitives.rs
        │   seb.rs
        │   skybox.rs
        │   test.rs
        │   ui.rs
        │   window.rs
        │
        └───gui
                gui.rs
                mod.rs
                panel.rs
                text.rs
                window.rs
```

---

## 📝 Note pentru evaluare

### Aspecte tehnice implementate:

✨ **Graphics Programming:**
- Shadow mapping cu depth textures
- Framebuffer offscreen rendering
- Custom shader pipeline (vertex + fragment)
- Texture mapping și filtering
- Viewport management

🎮 **Game Systems:**
- First-person camera controller
- Input handling (keyboard + mouse)
- Physics simulation (gravitație custom)
- Animation systems

🔧 **Rust specifics:**
- Unsafe bindings pentru OpenGL
- FFI (Foreign Function Interface) cu GLFW
- Memory management manual pentru GPU resources
- Feature flags pentru organizarea demo-urilor

---

## 🐛 Known Issues

- 🚧 GUI system încă în dezvoltare (Demo 6)

---

## 👨‍💻 Autor

Proiect dezvoltat de mine în vara 2025 ca explorare a programării grafice low-level în Rust.

---

## 📄 Licență

[Specifică licența aici - ex: MIT, GPL, etc.]

---


**Mulțumesc pentru evaluare! 🙏**
