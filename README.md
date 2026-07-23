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

Proiectul conține **7 demonstrații diferite**, fiecare ilustrând aspecte diferite ale programării grafice.

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

# Demo 7
cargo run --features demo7
```

---

## 🎯 Demo-uri disponibile

**Demo 1:**
```
╔════════════════════════════════════════════════════════════════╗
║                      CONTROALE JOC                             ║
╠════════════════════════════════════════════════════════════════╣
║  [TAB]     - Intrare/Ieșire din modul player                   ║
║  [ESC]     - Închidere program                                 ║
║  [SPACE]   - Resetare obiecte în cădere                        ║
║                                                                ║
║  Modul Player:                                                 ║
║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║
║    Mouse   - Rotire cameră                                     ║
╚════════════════════════════════════════════════════════════════╝
```


https://github.com/user-attachments/assets/026e5384-234d-46fb-84af-1ad299439d13


**Demo 2:**
```
╔════════════════════════════════════════════════════════════════╗
║                      CONTROALE JOC                             ║
╠════════════════════════════════════════════════════════════════╣
║  [TAB]     - Intrare/Ieșire din modul player                   ║
║  [ESC]     - Închidere program                                 ║
║                                                                ║
║  Modul Player:                                                 ║
║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║
║    Mouse   - Rotire cameră                                     ║
╚════════════════════════════════════════════════════════════════╝
```
<img width="786" height="865" alt="demo2" src="https://github.com/user-attachments/assets/2a2fb8ff-c0f2-40d5-9be2-921e28c6d3dc" />

**Demo 3:**
```
╔════════════════════════════════════════════════════════════════╗
║                      CONTROALE JOC                             ║
╠════════════════════════════════════════════════════════════════╣
║  [TAB]     - Intrare/Ieșire din modul player                   ║
║  [ENTER]   - Intrare/Ieșire din modul gravitație planetă       ║
║  [ESC]     - Închidere program                                 ║
║                                                                ║
║  Modul Player:                                                 ║
║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║
║    Mouse   - Rotire cameră                                     ║
║                                                                ║
║  Modul Gravitație Planetă:                                     ║
║    [SPACE] - Săritură pe planetă                               ║
╚════════════════════════════════════════════════════════════════╝
```


https://github.com/user-attachments/assets/de90ccb6-36f3-45d2-817b-03ad00ce65b9


**Demo 4:**
```
╔════════════════════════════════════════════════════════════════╗
║                      CONTROALE JOC                             ║
╠════════════════════════════════════════════════════════════════╣
║  [TAB]     - Intrare/Ieșire din modul player                   ║
║  [ESC]     - Închidere program                                 ║
║                                                                ║
║  Modul Player:                                                 ║
║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║
║    Mouse   - Rotire cameră                                     ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
```



https://github.com/user-attachments/assets/7f98ab15-473d-4a4f-8604-047d8f5ae93c



**Demo 5:**
```
╔════════════════════════════════════════════════════════════════╗
║                      CONTROALE JOC                             ║
╠════════════════════════════════════════════════════════════════╣
║  [TAB]     - Intrare/Ieșire din modul player                   ║
║  [ESC]     - Închidere program                                 ║
║                                                                ║
║  Modul Player:                                                 ║
║    W/A/S/D     - Mișcare (înainte/stânga/înapoi/dreapta)       ║
║    Mouse       - Rotire cameră                                 ║
║    Click stânga - Tragere pistol                               ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
```


https://github.com/user-attachments/assets/23e34c91-2015-4e0d-81a8-eb61133114d3


**Demo 6:**
```
╔════════════════════════════════════════════════════════════════╗
║  Asta nu e joc, e o incercare de a crea ferestre si poligoane  ║
╠════════════════════════════════════════════════════════════════╣
║  [ESC]     - Închidere program                                 ║
║                                                                ║
║  Notă: Acesta este un prototip pentru crearea de ferestre      ║
║        stil ImGui. Work in progress!                           ║
╚════════════════════════════════════════════════════════════════╝
```

**Demo 7:**
```
╔════════════════════════════════════════════════════════════════╗
║                          TEXT BOXURI                           ║
╠════════════════════════════════════════════════════════════════╣
║  [ESC]     - Închidere program                                 ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
```
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

## 📜 Licență

Acest proiect este protejat prin **Non-Commercial License**. 
Vezi fișierul [LICENSE](LICENSE) pentru termenii completi.

**Utilizare comercială interzisă** fără permisiunea scrisă a autorului.

© 2025 [Coviță Sebastian Marian]

---


**Mulțumesc pentru evaluare! 🙏**
