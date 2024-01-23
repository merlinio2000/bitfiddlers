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

#[component]
pub fn ErrorList(errors: ReadSignal<Vec<anyhow::Error>>) -> impl IntoView {
    view! {
        <div id="errors">
            <p>"Errors: "</p>
            // we can render a list of errors as strings, if we'd like
            <ul>
                {move || {
                    errors
                        .with(|es| {
                            es.into_iter()
                                .map(|e| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        })
                }}

            </ul>
        </div>
    }
}
