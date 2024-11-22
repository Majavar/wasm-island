#![allow(non_snake_case)]
mod components;
mod defaults;
mod map_generator;
mod settings;

use defaults::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use map_generator::Generator;
use web_sys::wasm_bindgen::JsCast;

fn main() {
    dioxus_logger::init(Level::INFO).expect("Failed to initialize logger");
    info!("Starting the application");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut width = use_signal(|| 512);
    let mut height = use_signal(|| 512);

    let mut generator = use_signal(|| 
        Generator::build()
            .seed(DEFAULT_SEED)
            .interpolation(DEFAULT_INTERPOLATION)
            .noise(DEFAULT_NOISE)
            .build()
    );

    use_effect(use_reactive((), move |()| {
        let window = web_sys::window().expect("Failed to get window");
        let canvas = window
            .document()
            .expect("Failed to get document")
            .get_element_by_id("canvas")
            .expect("Failed to get canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Failed to convert to HtmlCanvasElement");
        let context = canvas
            .get_context("2d")
            .expect("Failed to get 2d context")
            .expect("Failed to get 2d context (2nd time)")
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .expect("Failed to convert to CanvasRenderingContext2d");

        let data = generator.read().generate(*width.read(), *height.read());

        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
            web_sys::wasm_bindgen::Clamped(&data),
            *width.read() as u32,
            *height.read() as u32,
        )
        .unwrap();
        context
            .put_image_data(&data, 0.0, 0.0)
            .expect("Failed to put image data");
    }));

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        settings::element {
            onseedchange: move |seed| generator.write().update_seed(seed),
            oninterpolationchange: move |interpolation| generator.write().update_interpolation(interpolation),
            onnoisechange: move |noise| generator.write().update_noise(noise),
            onwidthchange: move |w| *width.write() = w,
            onheightchange: move |h| *height.write() = h,
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
        }
        div { class: "vertical-divider" }
        div {
            id: "image",
            canvas {
                id: "canvas",
                width: "{width}",
                height: "{height}",
            }
        }
    }
}
