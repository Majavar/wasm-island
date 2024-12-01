use dioxus::prelude::*;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Props)]
pub struct Props<T: 'static + Default + PartialEq + Clone> {
    min: T,
    max: T,
    value: Option<T>,
    step: Option<T>,
    disabled: Option<bool>,
    onchange: EventHandler<T>,
}

#[component]
pub fn element<
    T: 'static + Default + PartialEq + Copy + Clone + ToString + FromStr + IntoAttributeValue,
>(
    props: Props<T>,
) -> Element {
    let mut value = use_signal(|| props.value.unwrap_or(props.min));

    rsx! {
        input {
            r#type: "range",
            class: "full-width",
            min: props.min,
            max: props.max,
            value: value,
            step: props.step,
            disabled: props.disabled,
            oninput: move |input| {
                if let Ok(s) = input.value().parse::<T>() {
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
