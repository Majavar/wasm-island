use dioxus::prelude::*;
use std::fmt::Debug;
use strum::IntoEnumIterator;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props<T: 'static + IntoEnumIterator + Default + PartialEq + Eq + Clone> {
    value: Option<T>,
    disabled: Option<bool>,
    onchange: EventHandler<T>,
}

#[component]
pub fn element<
    T: 'static + IntoEnumIterator + Default + PartialEq + Eq + Copy + Clone + ToString,
>(
    props: Props<T>,
) -> Element {
    let mut value = use_signal(|| props.value.unwrap_or_default());

    rsx! {
        select {
            class: "full-width",
            disabled: props.disabled,
            onchange: move |event| {
                if let Ok(index) = event.value().parse() {
                    if let Some(variant) = T::iter().nth(index) {
                        value.set(variant);
                        props.onchange.call(variant);
                    } else {
                        let current = *value.read();
                        value.set(current);
                    }
                } else {
                    let current = *value.read();
                    value.set(current);
                }
            },
            {
                T::iter().enumerate().map(|(i, e)| rsx! {
                    option { value: i as i64, selected: e == *value.read(), {e.to_string()} }
                })
            }
        }
    }
}
