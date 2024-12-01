use dioxus::prelude::*;
use std::fmt::Debug;
use strum::IntoEnumIterator;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props<T: 'static + IntoEnumIterator + Default + PartialEq + Eq + Clone> {
    value: Option<T>,
    onchange: EventHandler<T>,
}

#[component]
pub fn element<
    T: 'static + IntoEnumIterator + Default + PartialEq + Eq + Copy + Clone + ToString,
>(
    props: Props<T>,
) -> Element {
    let mut current = use_signal(|| props.value.unwrap_or_default());

    rsx! {
        div {
            class: "navbar",
            {
                T::iter().map(|e| rsx! {
                    a { class: if *current.read() == e { "active" } else { "" },
                        onclick: move |_| {
                            props.onchange.call(e);
                            current.set(e);
                        },
                        {e.to_string()} }
                })
            }
        }
    }
}
