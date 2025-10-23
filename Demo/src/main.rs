mod seb;

use crate::seb::gui::gui::Clip;
use crate::seb::gui::panel::{self, Panel, PanelRenderer};
use crate::seb::gui::window::{GuiBuilder, GuiRenderer};
use crate::seb::planet::Planet;
use crate::seb::player::Player;
use crate::seb::primitives::{
    Line, LineRenderer, Rectangle, RectangleRenderer, Sphere, SphereRenderer, Vector,
    VectorRenderer,
};
use crate::seb::skybox::Skybox;
use gl::BLEND;
use nalgebra_glm as glm;
use seb::collision::{Collider, collide};
use seb::gui::text::{TextBoxD, TextBoxRenderer, TextFont};
use seb::gui::window;
use seb::model::*;
use seb::primitives;
use seb::test::BlackHole;
use seb::window::Window;

#[cfg(feature = "demo1")]
fn demo1() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      CONTROALE JOC                             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [TAB]     - Intrare/Ieșire din modul player                   ║");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║  [SPACE]   - Resetare obiecte în cădere                        ║");
    println!("║                                                                ║");
    println!("║  Modul Player:                                                 ║");
    println!("║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║");
    println!("║    Mouse   - Rotire cameră                                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    let mut window = seb::window::Window::new();
    window.create(1000, 800, "Window");

    let mut player = Player::new(0.1);

    let skybox = Skybox::new([
        "./assets/spaceskybox/right.png",  // +X
        "./assets/spaceskybox/left.png",   // -X
        "./assets/spaceskybox/top.png",    // +Y
        "./assets/spaceskybox/bottom.png", // -Y
        "./assets/spaceskybox/front.png",  // +Z
        "./assets/spaceskybox/back.png",   // -Z
    ]);

    let mut rc1 = Rectangle::new();
    rc1.color = glm::vec4(1.0, 0.0, 0.0, 1.0);
    let mut rc2 = Rectangle::new();
    rc2.color = glm::vec4(0.0, 1.0, 0.0, 1.0);
    let mut rc3 = Rectangle::new();
    rc3.color = glm::vec4(0.0, 0.0, 1.0, 1.0);
    let mut rc4 = Rectangle::new();
    rc4.color = glm::vec4(1.0, 1.0, 0.0, 1.0);

    rc1.scale = glm::vec3(1.0, 0.05, 1.0);
    rc2.scale = glm::vec3(0.05, 0.5, 0.5);
    rc2.position = glm::vec3(-0.5, 0.5, 0.0);
    rc2.x_angle = 13.0;
    rc2.z_angle = 45.0;
    rc3.scale = glm::vec3(0.1, 0.2, 0.3);
    rc3.position = glm::vec3(0.5, 1.5, 0.0);
    rc3.x_angle = 123.0;
    rc3.z_angle = 135.0;
    rc4.position = glm::vec3(0.5, 0.5, 0.0);
    rc4.scale = glm::vec3(0.2, 0.4, 0.2);
    rc4.y_angle = 12.0;
    rc4.z_angle = 124.0;

    let rcr = RectangleRenderer::new();

    let mut sc1 = Sphere::new();
    sc1.scale = 0.4;
    sc1.position = glm::vec3(0.5, 0.5, 0.0);
    sc1.color = glm::vec4(1.0, 0.0, 1.0, 1.0);

    let mut sc2 = Sphere::new();
    sc2.scale = 0.2;
    sc2.position = glm::vec3(-0.5, 1.5, 0.0);
    sc2.color = glm::vec4(0.0, 1.0, 1.0, 1.0);

    let scr = SphereRenderer::new();

    let mut time = 0.0f32;
    while window.is_open() {
        window.set_color(0.0, 0.0, 0.0, 1.0);
        window.poll_events();

        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            window.close();
        }

        player.bind(&mut window, 0.01);
        skybox.draw(player.projection, player.view);

        // if window.get_key(glfw::Key::Space) == glfw::Action::Press {
        //     rc2.position = glm::vec3(0.0, 1.5, 0.0);
        // }
        // rc3.z_angle = time.sin() * 1.5;
        // rc1.x_angle = time.cos() * 1.5;

        // rcr.draw(player.projection, player.view, rc1.get_model(), glm::vec3(0.0, 1.0, 0.0));
        // rcr.draw(player.projection, player.view, rc2.get_model(), glm::vec3(0.0, 0.0, 1.0));
        // rcr.draw(player.projection, player.view, rc3.get_model(), glm::vec3(1.0, 0.0, 0.0));

        // rc2.position -= glm::vec3(0.0, 0.01, 0.0);
        // if let Some(vec) =  collide(Collider::Rectangle(rc1), Collider::Rectangle(rc2)) {
        //     rc2.position += vec;
        // }
        // if let Some(vec) =  collide(Collider::Rectangle(rc3), Collider::Rectangle(rc2)) {
        //     rc2.position += vec;
        // }

        //
        // scr.draw(player.projection, player.view, sc1.get_model(), glm::vec3(1.0, 0.0, 0.0));
        // scr.draw(player.projection, player.view, sc2.get_model(), glm::vec3(0.0, 0.0, 1.0));
        // sc1.position -= glm::vec3(0.0, 0.01, 0.0);
        // if let Some(vec) = collide(Collider::Sphere(sc1), Collider::Sphere(sc2)) {
        //     sc1.position += vec;
        // }

        if window.get_key(glfw::Key::Space).unwrap() == glfw::Action::Press {
            rc3.position = glm::vec3(0.5, 1.5, 0.0);
            sc2.position = glm::vec3(-0.5, 1.5, 0.0);
        }

        rc3.position -= glm::vec3(0.0, 0.01, 0.0);
        sc2.position -= glm::vec3(0.0, 0.02, 0.0);

        sc1.position.x += time.sin() * 0.005;
        sc1.position.z += time.cos() * 0.005;

        rc1.z_angle = time.sin() * 5.0;
        rc1.x_angle = time.cos() * 5.0;

        rc2.y_angle += time.sin();

        if let Some(mtv) = collide(Collider::Rectangle(rc3), Collider::Rectangle(rc1)) {
            rc3.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Rectangle(rc3), Collider::Rectangle(rc2)) {
            rc3.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Rectangle(rc3), Collider::Sphere(sc1)) {
            rc3.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Sphere(sc2), Collider::Rectangle(rc1)) {
            sc2.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Sphere(sc2), Collider::Rectangle(rc2)) {
            sc2.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Sphere(sc2), Collider::Sphere(sc1)) {
            sc2.position += mtv;
        }
        if let Some(mtv) = collide(Collider::Sphere(sc2), Collider::Rectangle(rc3)) {
            sc2.position += mtv / 2.0;
            rc3.position -= mtv / 2.0;
        }

        rcr.draw(player.projection, player.view, &mut [rc1, rc2, rc3]);
        scr.draw(player.projection, player.view, &mut [sc1, sc2]);

        time += 0.01;

        window.swap_buffers();
    }
}

#[cfg(feature = "demo2")]
fn demo2() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      CONTROALE JOC                             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [TAB]     - Intrare/Ieșire din modul player                   ║");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║                                                                ║");
    println!("║  Modul Player:                                                 ║");
    println!("║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║");
    println!("║    Mouse   - Rotire cameră                                     ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    let mut window = Window::new();
    window.create(1000, 800, "Window");

    let mut player = Player::new(0.0);

    let skybox = Skybox::new([
        "./assets/spaceskybox/right.png",  // +X
        "./assets/spaceskybox/left.png",   // -X
        "./assets/spaceskybox/top.png",    // +Y
        "./assets/spaceskybox/bottom.png", // -Y
        "./assets/spaceskybox/front.png",  // +Z
        "./assets/spaceskybox/back.png",   // -Z
    ]);

    let mut lr = LineRenderer::new();

    let mut line1 = Line::from(glm::vec3(0.0, 0.0, 0.0), glm::vec3(1.0, 1.0, 1.0));
    line1.color = glm::vec4(1.0, 0.0, 0.0, 1.0);

    let mut line2 = Line::from(glm::vec3(1.0, 0.0, 0.0), glm::vec3(2.0, 1.0, 0.0));
    line2.color = glm::vec4(0.0, 1.0, 0.0, 1.0);

    let mut line3 = Line::from(glm::vec3(0.0, 1.0, 0.0), glm::vec3(1.0, 2.0, 1.0));
    line3.color = glm::vec4(0.0, 0.0, 1.0, 1.0);

    let mut line4 = Line::from(glm::vec3(1.0, 1.0, 0.0), glm::vec3(2.0, 2.0, 1.0));
    line4.color = glm::vec4(1.0, 1.0, 0.0, 1.0);

    let mut line5 = Line::from(glm::vec3(0.5, 0.5, 0.0), glm::vec3(1.5, 1.5, 1.0));
    line5.color = glm::vec4(1.0, 0.0, 1.0, 1.0);

    let mut vr = VectorRenderer::new();

    let mut vector1 = Vector::from(glm::vec3(0.0, 0.0, 1.0), glm::vec3(1.0, 0.5, 1.0));
    vector1.color = glm::vec4(0.5, 0.0, 0.5, 1.0);

    let mut vector2 = Vector::from(glm::vec3(1.0, 1.0, 0.5), glm::vec3(2.0, 1.5, 0.5));
    vector2.color = glm::vec4(0.0, 0.5, 0.5, 1.0);

    let mut vector3 = Vector::from(glm::vec3(0.5, 2.0, 0.0), glm::vec3(1.5, 2.5, 0.5));
    vector3.color = glm::vec4(0.5, 0.5, 0.0, 1.0);

    let mut vector4 = Vector::from(glm::vec3(2.0, 0.5, 0.0), glm::vec3(3.0, 1.0, 0.5));
    vector4.color = glm::vec4(1.0, 0.5, 0.0, 1.0);

    let mut vector5 = Vector::from(glm::vec3(1.0, 2.0, 1.0), glm::vec3(2.0, 3.0, 1.5));
    vector5.color = glm::vec4(0.0, 1.0, 0.5, 1.0);

    let mut rr = RectangleRenderer::new();

    let mut rectangle1 = Rectangle::new();
    rectangle1.position = glm::vec3(1.0, 2.0, 0.0);
    rectangle1.scale = glm::vec3(1.0, 2.0, 0.5);
    rectangle1.color = glm::vec4(1.0, 0.0, 0.0, 1.0);
    rectangle1.x_angle = 12.0;
    rectangle1.y_angle = 45.0;
    rectangle1.z_angle = 0.0;

    let mut rectangle2 = Rectangle::new();
    rectangle2.position = glm::vec3(2.0, 1.0, 0.5);
    rectangle2.scale = glm::vec3(0.5, 1.5, 0.5);
    rectangle2.color = glm::vec4(0.0, 1.0, 0.0, 1.0);
    rectangle2.x_angle = 30.0;
    rectangle2.y_angle = 10.0;
    rectangle2.z_angle = 5.0;

    let mut rectangle3 = Rectangle::new();
    rectangle3.position = glm::vec3(-1.0, 1.0, 1.0);
    rectangle3.scale = glm::vec3(1.5, 0.5, 0.5);
    rectangle3.color = glm::vec4(0.0, 0.0, 1.0, 1.0);
    rectangle3.x_angle = 0.0;
    rectangle3.y_angle = 60.0;
    rectangle3.z_angle = 20.0;

    let mut rectangle4 = Rectangle::new();
    rectangle4.position = glm::vec3(0.5, -1.0, 0.5);
    rectangle4.scale = glm::vec3(2.0, 1.0, 1.0);
    rectangle4.color = glm::vec4(1.0, 1.0, 0.0, 1.0);
    rectangle4.x_angle = 45.0;
    rectangle4.y_angle = 30.0;
    rectangle4.z_angle = 10.0;

    let mut rectangle5 = Rectangle::new();
    rectangle5.position = glm::vec3(-2.0, 0.0, 0.0);
    rectangle5.scale = glm::vec3(1.0, 1.0, 0.5);
    rectangle5.color = glm::vec4(1.0, 0.0, 1.0, 1.0);
    rectangle5.x_angle = 15.0;
    rectangle5.y_angle = 20.0;
    rectangle5.z_angle = 45.0;

    let mut sr = SphereRenderer::new();

    let mut sphere1 = Sphere::new();
    sphere1.position = glm::vec3(1.0, 2.0, 0.0);
    sphere1.scale = 0.6;
    sphere1.color = glm::vec4(1.0, 0.0, 0.0, 1.0);

    let mut sphere2 = Sphere::new();
    sphere2.position = glm::vec3(2.0, 1.0, 0.5);
    sphere2.scale = 0.5;
    sphere2.color = glm::vec4(0.0, 1.0, 0.0, 1.0);

    let mut sphere3 = Sphere::new();
    sphere3.position = glm::vec3(-1.0, 1.5, 0.2);
    sphere3.scale = 0.1;
    sphere3.color = glm::vec4(0.0, 0.0, 1.0, 1.0);

    let mut sphere4 = Sphere::new();
    sphere4.position = glm::vec3(0.5, -1.0, 0.3);
    sphere4.scale = 1.0;
    sphere4.color = glm::vec4(1.0, 1.0, 0.0, 1.0);

    let mut sphere5 = Sphere::new();
    sphere5.position = glm::vec3(-2.0, 0.0, 0.1);
    sphere5.scale = 1.3;
    sphere5.color = glm::vec4(1.0, 0.0, 1.0, 1.0);

    while window.is_open() {
        window.set_color(0.0, 0.0, 0.2, 1.0);
        window.poll_events();
        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            window.close();
        }

        player.bind(&mut window, 0.1);

        lr.draw(
            player.projection,
            player.view,
            &mut [line1, line2, line3, line4, line5],
        );
        vr.draw(
            player.projection,
            player.view,
            &mut [vector1, vector2, vector3, vector4, vector5],
        );
        rr.draw(
            player.projection,
            player.view,
            &mut [rectangle1, rectangle2, rectangle3, rectangle4, rectangle5],
        );
        sr.draw(
            player.projection,
            player.view,
            &mut [sphere1, sphere2, sphere3, sphere4, sphere5],
        );

        skybox.draw(player.projection, player.view);
        window.swap_buffers();
    }
}

#[cfg(feature = "demo3")]
fn demo3() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      CONTROALE JOC                             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [TAB]     - Intrare/Ieșire din modul player                   ║");
    println!("║  [ENTER]   - Intrare/Ieșire din modul gravitație planetă       ║");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║                                                                ║");
    println!("║  Modul Player:                                                 ║");
    println!("║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║");
    println!("║    Mouse   - Rotire cameră                                     ║");
    println!("║                                                                ║");
    println!("║  Modul Gravitație Planetă:                                     ║");
    println!("║    [SPACE] - Săritură pe planetă                               ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    let mut window = seb::window::Window::new();
    window.create(1000, 800, "Window");

    let mut player = Player::new(10.4);

    let skybox = Skybox::new([
        "./assets/spaceskybox/right.png",  // +X
        "./assets/spaceskybox/left.png",   // -X
        "./assets/spaceskybox/top.png",    // +Y
        "./assets/spaceskybox/bottom.png", // -Y
        "./assets/spaceskybox/front.png",  // +Z
        "./assets/spaceskybox/back.png",   // -Z
    ]);

    let mut planet = Planet::new();
    planet
        .from_map("./assets/planet/worldgen1.jpg", 1.0, 10.0)
        .unwrap();
    planet.generate_terrain();
    planet
        .load_texture("./assets/planet/worldgen1.jpg")
        .unwrap();
    planet.init();

    let mut on_planet: bool = false;
    let mut h: f32 = 0.0;

    let mut jump = false;
    while window.is_open() {
        window.set_color(0.0, 0.0, 0.0, 1.0);
        window.poll_events();

        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            break;
        }

        if let Some(key) = window.keyboard.find_key(glfw::Key::Enter)
            && key.action == glfw::Action::Press
        {
            on_planet = !on_planet;
        }

        if on_planet {
            if window.get_key(glfw::Key::Space).unwrap() == glfw::Action::Press {
                if !jump {
                    h += 0.02;
                    jump = true;
                }
            }
            h -= 0.001;

            let ground_pos = planet.get_position_on_sphere(player.cam_pos, 0.1);

            player.add_height(planet.position, h);

            let d1 = glm::length(&(ground_pos - planet.position));
            let d2 = glm::length(&(player.cam_pos - planet.position));

            if d2 < d1 {
                h = 0.0;
                jump = false;
                player.set_height(ground_pos, planet.position);
            }

            player.bind_sphere(&mut window, 0.01, planet.position, 1.0);
        } else {
            let dist = glm::length(&(player.cam_pos - planet.position));
            if dist < planet.scale {
                let ground_pos = planet.get_position_on_sphere(player.cam_pos, 0.1);
                player.set_height(ground_pos, planet.position);
            }
            player.cam_up = glm::vec3(0.0, 1.0, 0.0);
            player.bind(&mut window, 0.1);
        }

        planet.draw(player.projection, player.view, player.cam_pos);

        skybox.draw(player.projection, player.view);
        window.swap_buffers();
    }
}

#[cfg(feature = "demo4")]
fn demo4() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      CONTROALE JOC                             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [TAB]     - Intrare/Ieșire din modul player                   ║");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║                                                                ║");
    println!("║  Modul Player:                                                 ║");
    println!("║    W/A/S/D - Mișcare (înainte/stânga/înapoi/dreapta)           ║");
    println!("║    Mouse   - Rotire cameră                                     ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    let mut window = seb::window::Window::new();
    window.create(1000, 800, "Window");

    let mut model = Model::new("./assets/model/casa.glb");
    model.init();

    let mut player = Player::new(0.1);

    let mut light = Light::new();
    light.init_shadow(4096, 4096);
    light.add_light(glm::vec3(1.0, 1.0, 1.0), glm::vec3(-1.0, -1.0, -1.0));

    let skybox = Skybox::new([
        "./assets/spaceskybox/right.png",  // +X
        "./assets/spaceskybox/left.png",   // -X
        "./assets/spaceskybox/top.png",    // +Y
        "./assets/spaceskybox/bottom.png", // -Y
        "./assets/spaceskybox/front.png",  // +Z
        "./assets/spaceskybox/back.png",   // -Z
    ]);

    let mut time: f32 = 0f32;
    while window.is_open() {
        light.add_light(
            glm::vec3(time.sin() * 2.0, 1.0, time.cos() * 2.0),
            glm::vec3(0.0, 0.0, 0.0),
        );
        light.bind_shadow();

        model.draw_for_shadow(&light);

        light.unbind_shadow();

        window.set_color(0.0, 0.0, 0.0, 1.0);
        window.poll_events();

        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            break;
        }

        player.bind(&mut window, 0.1);

        model.draw(player.projection, player.view, player.cam_pos, light);
        time += 0.01;
        skybox.draw(player.projection, player.view);
        window.swap_buffers();
    }
}

#[cfg(feature = "demo5")]
fn demo5() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║                      CONTROALE JOC                             ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [TAB]     - Intrare/Ieșire din modul player                   ║");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║                                                                ║");
    println!("║  Modul Player:                                                 ║");
    println!("║    W/A/S/D     - Mișcare (înainte/stânga/înapoi/dreapta)       ║");
    println!("║    Mouse       - Rotire cameră                                 ║");
    println!("║    Click stânga - Tragere pistol                               ║");
    println!("║                                                                ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    let mut window = seb::window::Window::new();
    window.create(1000, 800, "Window");

    let mut model = Model::new("./assets/model/1911.glb");
    model.init();
    let mut model2 = Model::new("./assets/model/map.glb");
    model2.init();

    let mut player = Player::new(0.1);

    let mut light = Light::new();
    light.init_shadow(4096, 4096);
    light.add_light(glm::vec3(1.0, 1.0, 1.0), glm::vec3(-1.0, -1.0, -1.0));

    let skybox = Skybox::new([
        "./assets/spaceskybox/right.png",  // +X
        "./assets/spaceskybox/left.png",   // -X
        "./assets/spaceskybox/top.png",    // +Y
        "./assets/spaceskybox/bottom.png", // -Y
        "./assets/spaceskybox/front.png",  // +Z
        "./assets/spaceskybox/back.png",   // -Z
    ]);

    let mut time = 0.0;
    let mut anim = false;
    while window.is_open() {
        light.bind_shadow();

        model.draw_for_shadow(&light);
        model2.draw_for_shadow(&light);

        light.unbind_shadow();

        window.set_color(0.0, 0.0, 0.0, 1.0);
        window.poll_events();

        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            break;
        }

        player.bind(&mut window, 0.1);

        if window.get_key(glfw::Key::Space).unwrap() == glfw::Action::Press {
            time = 0.0;
        }
        if let Some(action) = window.get_mouse_button(glfw::MouseButton::Button1) {
            if action == glfw::Action::Press {
                anim = true;
            } else if action == glfw::Action::Release {
                anim = false;
            }
        }
        if anim {
            if time > 1.0 {
                time = 0.0;
            }
        }

        for i in 0..8 {
            model.apply_animation(i, time);
        }
        model.draw(player.projection, player.view, player.cam_pos, light);
        model2.draw(player.projection, player.view, player.cam_pos, light);

        time += 0.1;

        skybox.draw(player.projection, player.view);
        window.swap_buffers();
    }
}

#[cfg(feature = "demo6")]
fn demo6() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  Asta nu e joc, e o incercare de a crea ferestre si poligoane  ║");
    println!("╠════════════════════════════════════════════════════════════════╣");
    println!("║  [ESC]     - Închidere program                                 ║");
    println!("║                                                                ║");
    println!("║  Notă: Acesta este un prototip pentru crearea de ferestre      ║");
    println!("║        stil ImGui. Work in progress!                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    let mut window = Window::new();
    window.create(1000, 800, "Window");

    let mut gr = GuiRenderer::new("./assets/Roboto-VariableFont_wdth,wght.ttf", 20f32);
    let mut gb = GuiBuilder::new(
        glm::vec2(0.0, 0.0),
        glm::vec2(window.width as f32, window.height as f32),
    );

    let mut win = Panel::new();
    win.position = glm::vec2(60.0, 30.0);
    win.size = glm::vec2(600.0, 400.0);
    win.color = glm::vec4(1.0, 0.5, 0.0, 1.0);
    win.border_thickness = 5f32;
    win.border_color = glm::vec4(1.0, 1.0, 1.0, 1.0);

    gb.add_window(win, |b| {
        let mut p1 = Panel::new();
        p1.color = glm::vec4(1.0, 0.5, 0.9, 1.0);
        p1.position = glm::vec2(10.0, 10.0);
        p1.size = glm::vec2(40.0, 30.0);
        p1.draw_as_circle = true;
        p1.border_thickness = 1f32;
        p1.z_index = 0.1;
        p1.border_color = glm::vec4(0.0, 1.0, 1.0, 1.0);

        b.push_panel(p1);

        let mut win = Panel::new();
        win.position = glm::vec2(50.0, 60.0);
        win.size = glm::vec2(100.0, 80.0);
        win.color = glm::vec4(0.2, 0.5, 0.0, 1.0);
        win.border_thickness = 1f32;
        win.border_color = glm::vec4(0.0, 0.0, 1.0, 1.0);
        win.z_index = 0.1;

        b.add_window(win, |b| {
            let mut p1 = Panel::new();
            p1.color = glm::vec4(1.0, 0.5, 0.5, 1.0);
            p1.position = glm::vec2(10.0, 10.0);
            p1.size = glm::vec2(40.0, 30.0);
            p1.draw_as_circle = true;
            p1.border_thickness = 1f32;
            p1.z_index = 0.2;
            p1.border_color = glm::vec4(0.0, 1.0, 1.0, 1.0);

            b.push_panel(p1);
        });
    });

    while window.is_open() {
        window.set_color(0.0, 0.0, 0.0, 1.0);
        window.poll_events();
        if window.get_key(glfw::Key::Escape).unwrap() == glfw::Action::Press {
            window.close();
        }

        gr.draw(&mut gb, window.width, window.height);

        window.swap_buffers();
    }
}

fn main() {
    #[cfg(feature = "demo1")]
    demo1();

    #[cfg(feature = "demo2")]
    demo2();

    #[cfg(feature = "demo3")]
    demo3();

    #[cfg(feature = "demo4")]
    demo4();

    #[cfg(feature = "demo5")]
    demo5();

    #[cfg(feature = "demo6")]
    demo6();
}
