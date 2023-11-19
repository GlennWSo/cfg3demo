use log::info;
use three_d::{
    egui::SidePanel, AmbientLight, Camera, ClearState, FrameOutput, OrbitControl, Skybox, Window,
    WindowSettings,
};
use three_d_asset::{degrees, vec3, Srgba, TriMesh, Viewport};

use crate::product::Product;

pub async fn render(mut product: Product) {
    let window = Window::new(WindowSettings {
        title: "Product Config".to_string(),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 2410., 580.),
        vec3(0.0, 410., 580.),
        vec3(0.0, 0.0, 1.0),
        degrees(45.0),
        0.1,
        10000.0,
    );
    let mut control = OrbitControl::new(*camera.target(), 1000.0, 5000.0);

    let asset_paths = [
        "./chinese_garden_4k.hdr", // Source: https://polyhaven.com/
                                   // "./chair/skeleton.obj",
                                   // "./chair/skeleton.mtl",
    ];
    let mut loaded = if let Ok(loaded) = three_d_asset::io::load_async(&asset_paths).await {
        info!("loaded skybox from assets");
        loaded
    } else {
        panic!("failed to download the necessary assets, to enable running this example offline, place the relevant assets in a folder called 'assets' next to the three-d source")
    };

    let skybox = Skybox::new_from_equirectangular(
        &context,
        &loaded.deserialize("chinese_garden_4k").unwrap(),
    );
    let light = AmbientLight::new_with_environment(&context, 1.0, Srgba::WHITE, skybox.texture());

    // let mut part = Component::placeholder();
    // part.init(&context);
    product.init(&context);

    let mut gui = three_d::GUI::new(&context);

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
                    product.add_controls(ui);
                });
                panel_width = gui_context.used_rect().width();
            },
        );
        product.update();

        let viewport = Viewport {
            x: (panel_width * frame_input.device_pixel_ratio) as i32,
            y: 0,
            width: frame_input.viewport.width
                - (panel_width * frame_input.device_pixel_ratio) as u32,
            height: frame_input.viewport.height,
        };
        camera.set_viewport(viewport);
        control.handle_events(&mut camera, &mut frame_input.events);

        // let objects = skybox.into_iter().chain(product.objects());
        let objects = product.objects();
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .render(&camera, objects, &[&light])
            .write(|| gui.render());

        FrameOutput::default()
    });
}
