use std::fmt::Display;

// use three_d::egui::Slider;
use three_d::{
    egui::{Sense, SidePanel},
    AmbientLight, Camera, ClearState, FrameOutput, Gm, Mesh, OrbitControl, PhysicalMaterial,
    Skybox, Window, WindowSettings,
};
use three_d_asset::{degrees, vec3, PbrMaterial, Srgba, TriMesh, Viewport};

use three_d::egui::{Color32, ProgressBar, Ui};

use crate::product::Component;

pub async fn render(part: Component) {
    let window = Window::new(WindowSettings {
        title: "Product Config".to_string(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(-3.0, 1.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    // Source: https://polyhaven.com/
    let mut loaded = if let Ok(loaded) =
        three_d_asset::io::load_async(&["../assets/chinese_garden_4k.hdr"]).await
    {
        loaded
    } else {
        three_d_asset::io::load_async(&[
            "https://asny.github.io/three-d/assets/chinese_garden_4k.hdr",
        ])
        .await
        .expect("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(
        &context,
        &loaded.deserialize("chinese_garden_4k").unwrap(),
    );
    let light = AmbientLight::new_with_environment(&context, 1.0, Srgba::WHITE, skybox.texture());

    let mut part = Component::placeholder();

    let mut model = Gm::new(
        Mesh::new(&context, &TriMesh::sphere(32)),
        PhysicalMaterial::new_opaque(
            &context,
            &PbrMaterial {
                roughness: 0.2,
                metallic: 0.8,
                ..Default::default()
            },
        ),
    );
    let mut gui = three_d::GUI::new(&context);

    let mut color: [u8; 3] = part.material().rgb;
    // main loop
    window.render_loop(move |mut frame_input| {
        let mut panel_width = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                SidePanel::left("side_panel").show(gui_context, |ui| {
                    ui.heading("Config Panel");
                    // ui.horizontal();
                    part.material_picker(ui, &mut color);
                    ui.horizontal(|ui| {
                        ui.label("Metallic:");
                        ui.add(ProgressBar::new(part.material().metallic))
                    });
                    ui.horizontal(|ui| {
                        ui.label("Roughness:");
                        ui.add(ProgressBar::new(part.material().roughness));
                    });
                    let color = Color32::from_rgb(color[0], color[1], color[2]);
                    let (response, painter) =
                        ui.allocate_painter([30., 30.].into(), Sense::hover());
                    painter.rect(response.rect, 10., color, (5.0, Color32::LIGHT_GRAY));
                });
                panel_width = gui_context.used_rect().width();
            },
        );
        model.material.albedo = Srgba::from(part.material().rgb);
        model.material.metallic = part.material().metallic;
        model.material.roughness = part.material().roughness;

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, skybox.into_iter().chain(&model), &[&light])
            .write(|| gui.render());

        FrameOutput::default()
    });
}