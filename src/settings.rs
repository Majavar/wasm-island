use crate::{
    components::{EnumSelect, RandomNumberInput, SlidebarInteger},
    map_generator::{Interpolation, NoiseKind},
};
use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    oninterpolationchange: EventHandler<Interpolation>,
    onseedchange: EventHandler<u32>,
    onnoisechange: EventHandler<NoiseKind>,
    onwidthchange: EventHandler<usize>,
    onheightchange: EventHandler<usize>,
    width: Option<usize>,
    height: Option<usize>,
}

#[component]
pub fn element(props: Props) -> Element {
    rsx! {
        div {
            id: "settings",
            table {
                tr {
                    td {"Seed"}
                    td {":"}
                    td { RandomNumberInput { onchange: move|seed| props.onseedchange.call(seed) } }
                }
                tr {
                    td {"Interpolation"}
                    td {":"}
                    td { EnumSelect { onchange: move|interpolation| props.oninterpolationchange.call(interpolation) } }
                }
                tr {
                    td {"Noise type"}
                    td {":"}
                    td { EnumSelect { onchange: move|noise| props.onnoisechange.call(noise) } }
                }
                tr {
                    td {"Width"}
                    td {":"}
                    td { SlidebarInteger { min: 64, max: 1024, value: props.width, onchange: move|width| props.onwidthchange.call(width) } }
                }
                tr {
                    td {"Height"}
                    td {":"}
                    td { SlidebarInteger { min: 64, max: 1024, value: props.height, onchange: move|height| props.onheightchange.call(height) } }
                }
            }
        }
    }
}
