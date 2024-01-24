use std::num::ParseIntError;

use leptos::error::Error as LeptosError;
use leptos::logging::log;
use leptos::*;

use crate::calc::Calc;
use crate::components::calc_table::CalcTable;
use crate::components::general::BitOption;
use crate::utl::AppError;

fn parse_hex(s: &str) -> Result<u64, ParseIntError> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    std::primitive::u64::from_str_radix(s, 16)
}
fn parse_bin(s: &str) -> Result<u64, ParseIntError> {
    let s = s.trim_start_matches("0b").trim_start_matches("0B");
    std::primitive::u64::from_str_radix(s, 2)
}
fn parse_dec(s: &str) -> Result<u64, ParseIntError> {
    std::primitive::u64::from_str_radix(s, 10)
}

fn parse_input(input: &str) -> Result<u64, AppError> {
    let without_separators = input.replace('\'', "");
    let s = &without_separators;
    let result = if s.len() < 2 {
        parse_dec(s).map_err(|e| AppError::Parsing {
            what: input.to_string(),
            base: 10,
            cause: e,
        })
    } else {
        match &s[0..=1] {
            "0x" | "0X" => parse_hex(s).map_err(|e| AppError::Parsing {
                what: input.to_string(),
                base: 16,
                cause: e,
            }),
            "0b" | "0B" => parse_bin(s).map_err(|e| AppError::Parsing {
                what: input.to_string(),
                base: 2,
                cause: e,
            }),
            _dec => parse_dec(s).map_err(|e| AppError::Parsing {
                what: input.to_string(),
                base: 10,
                cause: e,
            }),
        }
    };
    log!("parsed {s} into {result:?}");
    result
}

#[component]
pub fn App() -> impl IntoView {
    let initial_left = "0xdead'beef".to_string();
    let initial_right = "0b1000'0000".to_string();
    let initial_width = 32u8;

    let (left_in, set_left_in) = create_signal(initial_left);
    let (right_in, set_right_in) = create_signal(initial_right);
    let (width_in, set_width_in) = create_signal(Ok(initial_width));

    let parsed = Signal::derive(move || -> Result<Calc, LeptosError> {
        Ok(Calc::try_new(
            parse_input(&left_in.get())?,
            parse_input(&right_in.get())?,
            width_in.get()?,
        )?)
    });

    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_left_in.set(event_target_value(&ev));
            }

            prop:value=left_in
        />
        <input
            type="text"
            on:input=move |ev| {
                set_right_in.set(event_target_value(&ev));
            }

            prop:value=right_in
        />
        <select
            name="bits"
            id="bits-sel"
            on:change=move |ev| { set_width_in.set(event_target_value(&ev).parse()) }
        >
            <BitOption value=width_in is=64/>
            <BitOption value=width_in is=32/>
            <BitOption value=width_in is=16/>
            <BitOption value=width_in is=8/>
        </select>
        // the fallback receives a signal containing current errors
        <ErrorBoundary fallback=|errors| {
            view! {
                <div class="error">
                    <p>"Errors: "</p>
                    // we can render a list of errors as strings, if we'd like
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}

                    </ul>
                </div>
            }
        }>
            {move || {
                parsed
                    .get()
                    .map(|calcin| view! { <CalcTable out=calcin.calc() width=calcin.width/> })
            }}

        </ErrorBoundary>
    }
}
