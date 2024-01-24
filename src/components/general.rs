use std::num::ParseIntError;

use leptos::*;

// TODO: only take Signal<u8> not Result
#[component]
pub fn BitOption(is: u8, value: ReadSignal<Result<u8, ParseIntError>>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get().is_ok_and(|v| v == is)>
            {is}
            -Bit
        </option>
    }
}
