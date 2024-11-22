use dioxus::prelude::*;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props {
    value: Option<u32>,
    onchange: EventHandler<u32>,
}

#[component]
pub fn element(props: Props) -> Element {
    let mut value = use_signal(|| props.value.unwrap_or(0));

    rsx! {
        input {
            r#type: "number",
            value: "{value}",
            style: "width: 10em;",
            oninput: move |input| {
                if let Ok(s) = input.value().parse::<u32>() {
                    value.set(s);
                    props.onchange.call(s);
                } else {
                    let current = *value.read();
                    value.set(current)
                }
            }
        }
        button {
            onclick: move |_| {
                let s = thread_rng().gen();
                value.set(s);
                props.onchange.call(s);
            },
            "🔀"
        }
    }
}
