use dioxus::prelude::*;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    min: usize,
    max: usize,
    value: Option<usize>,
    onchange: EventHandler<usize>,
}

#[component]
pub fn element(props: Props) -> Element {
    let mut value = use_signal(|| props.value.unwrap_or(props.min));

    rsx! {
        input {
            r#type: "range",
            class: "full-width",
            min: "{props.min}",
            max: "{props.max}",
            value: "{value}",
            oninput: move |input| {
                if let Ok(s) = input.value().parse::<usize>() {
                    value.set(s);
                    props.onchange.call(s);
                } else {
                    let current = *value.read();
                    value.set(current)
                }
            }
        }
    }
}
