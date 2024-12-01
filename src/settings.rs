use crate::{
    components::{EnumSelect, RandomNumberInput, Slidebar},
    map_generator::{GeneratorType, HeightmapKind, Interpolation, NoiseKind},
};
use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    generator_type: GeneratorType,
    seed: Option<i64>,
    interpolation: Option<Interpolation>,
    noise: Option<NoiseKind>,
    width: Option<i64>,
    height: Option<i64>,
    heightmap: Option<HeightmapKind>,
    octave: Option<i64>,
    persistence: Option<f64>,
    lacunarity: Option<f64>,
    flatten: Option<bool>,
    use_shading: Option<bool>,
    oninterpolationchange: EventHandler<Interpolation>,
    onseedchange: EventHandler<i64>,
    onnoisechange: EventHandler<NoiseKind>,
    onwidthchange: EventHandler<i64>,
    onheightchange: EventHandler<i64>,
    onheightmapchange: EventHandler<HeightmapKind>,
    onoctavechange: EventHandler<i64>,
    onpersistencechange: EventHandler<f64>,
    onlacunaritychange: EventHandler<f64>,
    onflattenchange: EventHandler<bool>,
    onuseshadingchange: EventHandler<bool>,
}

#[component]
pub fn element(props: Props) -> Element {
    let mut noise_type = use_signal(|| props.noise.unwrap_or_default());
    let mut heightmap_type = use_signal(|| props.heightmap.unwrap_or_default());

    rsx! {
        div {
            id: "settings",
            table {
                tr {
                    td {"Seed"}
                    td {":"}
                    td { RandomNumberInput { value: props.seed, onchange: move|seed| props.onseedchange.call(seed)}}
                }
                tr {
                    td {"Interpolation"}
                    td {":"}
                    td { EnumSelect {
                        value: props.interpolation,
                        disabled: *noise_type.read() == NoiseKind::Simplex ||
                            (props.generator_type != GeneratorType::Noise && *heightmap_type.read() != HeightmapKind::Fractal),
                        onchange: move|interpolation| props.oninterpolationchange.call(interpolation)}
                    }
                }
                tr {
                    td {"Noise type"}
                    td {":"}
                    td { EnumSelect {
                        value: props.noise,
                        disabled: props.generator_type != GeneratorType::Noise && *heightmap_type.read() != HeightmapKind::Fractal,
                        onchange: move|noise| {
                            *noise_type.write() = noise;
                            props.onnoisechange.call(noise);
                        }}
                    }
                }
                tr {
                    td {"Width"}
                    td {":"}
                    td { Slidebar { min: 64, max: 1024, value: props.width, onchange: move|width| props.onwidthchange.call(width)}}
                }
                tr {
                    td {"Height"}
                    td {":"}
                    td { Slidebar { min: 64, max: 1024, value: props.height, onchange: move|height| props.onheightchange.call(height)}}
                }
                {
                    if props.generator_type != GeneratorType::Noise {
                        rsx! {
                            tr {
                                td {"Heightmap"}
                                td {":"}
                                td { EnumSelect {
                                    value: props.heightmap,
                                    onchange: move|heightmap| {
                                        *heightmap_type.write() = heightmap;
                                        props.onheightmapchange.call(heightmap);
                                    }}
                                }
                            }
                            tr {
                                td {"Octave"}
                                td {":"}
                                td { Slidebar { min: 1, max: 12, value: props.octave, disabled: *heightmap_type.read() != HeightmapKind::Fractal ,onchange: move|octave| props.onoctavechange.call(octave)}}
                            }
                            tr {
                                td {"Persistence"}
                                td {":"}
                                td { Slidebar { min: 0.0, max: 1.0, step: 0.05, value: props.persistence, disabled: *heightmap_type.read() != HeightmapKind::Fractal, onchange: move|persistence| props.onpersistencechange.call(persistence)}}
                            }
                            tr {
                                td {"Lacunarity"}
                                td {":"}
                                td { Slidebar { min: 1.0, max: 4.0, step: 0.1, value: props.lacunarity, disabled: *heightmap_type.read() != HeightmapKind::Fractal, onchange: move|lacunarity| props.onlacunaritychange.call(lacunarity)}}
                            }
                            tr {
                                td {"Flatten"}
                                td {":"}
                                td { input {
                                    r#type: "checkbox",
                                    checked: props.flatten,
                                    onchange: move|e| props.onflattenchange.call(e.value().parse::<bool>().unwrap_or_default())
                                }}
                            }
                            {
                                if props.generator_type == GeneratorType::ColoredMap {
                                    rsx! {
                                        tr {
                                            td {"Use shading"}
                                            td {":"}
                                            td { input {
                                                r#type: "checkbox",
                                                checked: props.use_shading,
                                                onchange: move|e| props.onuseshadingchange.call(e.value().parse::<bool>().unwrap_or_default())
                                            }}
                                        }
                                    }
                                } else {
                                    rsx! {}
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        }
    }
}
