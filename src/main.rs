#![allow(non_snake_case)]
mod components;
mod defaults;
mod map_generator;
mod settings;

use defaults::*;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use map_generator::{ColorRamp, Generator};
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

    let mut generator_type = use_signal(|| DEFAULT_GENERATOR_TYPE);
    let mut generator = use_signal(|| {
        Generator::builder()
            .seed(DEFAULT_SEED)
            .interpolation(DEFAULT_INTERPOLATION)
            .noise(DEFAULT_NOISE)
            .width(DEFAULT_WIDTH)
            .height(DEFAULT_HEIGHT)
            .heightmap(DEFAULT_HEIGHTMAP)
            .octave(DEFAULT_OCTAVE)
            .lacunarity(DEFAULT_LACUNARITY)
            .persistence(DEFAULT_PERSISTENCE)
            .color_ramp(ColorRamp::from(DEFAULT_COLOR_RAMP.to_vec()))
            .build()
    });

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

        let data = generator.read().generate(*generator_type.read());

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
            onseedchange: move |seed| generator.write().set_seed(seed as u64),
            oninterpolationchange: move |interpolation| generator.write().set_interpolation(interpolation),
            onnoisechange: move |noise| generator.write().set_noise(noise),
            onwidthchange: move |w| {
                generator.write().set_width(w as usize);
                *width.write() = w;
            },
            onheightchange: move |h| {
                generator.write().set_height(h as usize);
                *height.write() = h;
            },
            onheightmapchange: move |heightmap| generator.write().set_heightmap(heightmap),
            onoctavechange: move |octave| generator.write().set_octave(octave as u64),
            onpersistencechange: move |persistence| generator.write().set_persistence(persistence),
            onlacunaritychange: move |lacunarity| generator.write().set_lacunarity(lacunarity),
            generator_type: *generator_type.read(),
            seed: DEFAULT_SEED as i64,
            interpolation: DEFAULT_INTERPOLATION,
            noise: DEFAULT_NOISE,
            width: DEFAULT_WIDTH as i64,
            height: DEFAULT_HEIGHT as i64,
            heightmap: DEFAULT_HEIGHTMAP,
            octave: DEFAULT_OCTAVE as i64,
            persistence: DEFAULT_PERSISTENCE,
            lacunarity: DEFAULT_LACUNARITY,
        }
        div { class: "vertical-divider" }
        div {
            id: "content",
            components::EnumNavbar {
                value: *generator_type.read(),
                onchange: move |variant| *generator_type.write() = variant,
            }
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
}
