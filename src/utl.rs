use std::num::ParseIntError;

use leptos::*;

use anyhow::Result;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("couldn't parse <{what}> in base {base}: {cause:?}")]
    Parsing {
        what: String,
        base: usize,
        cause: ParseIntError,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[component]
pub fn BitOption(is: u8, value: ReadSignal<Result<u8, ParseIntError>>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get().is_ok_and(|v| v == is)>
            {is}
            -Bit
        </option>
    }
}
