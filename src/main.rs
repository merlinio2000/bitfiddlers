mod calc;
mod utl;

use std::num::ParseIntError;

use crate::calc::*;
use crate::utl::*;
use leptos::error::Error as LeptosError;
use leptos::logging::log;
use leptos::*;

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
fn App() -> impl IntoView {
    let (in1, set_in1) = create_signal("0xdead'beef".to_string());
    let (in2, set_in2) = create_signal("0b1000'0000".to_string());
    let (bits, set_bits) = create_signal(Ok(32u8));

    let parsed = Signal::derive(move || -> Result<Calc, LeptosError> {
        Ok(Calc::try_new(
            parse_input(&in1.get())?,
            parse_input(&in2.get())?,
            bits.get()?,
        )?)
    });

    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_in1.set(event_target_value(&ev));
            }

            prop:value=in1
        />
        <input
            type="text"
            on:input=move |ev| {
                set_in2.set(event_target_value(&ev));
            }

            prop:value=in2
        />
        <select
            name="bits"
            id="bits-sel"
            on:change=move |ev| { set_bits.set(event_target_value(&ev).parse()) }
        >
            <BitOption value=bits is=64/>
            <BitOption value=bits is=32/>
            <BitOption value=bits is=16/>
            <BitOption value=bits is=8/>
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

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
