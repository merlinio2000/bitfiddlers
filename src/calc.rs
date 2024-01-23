use anyhow::anyhow;
use leptos::logging::log;
use leptos::*;

use crate::AppError;

// TODO: test 64-Bit

type CalcResult = (u64, Flags);

// TODO: not pub
#[derive(Debug, Copy, Clone)]
pub struct Calc {
    pub left: u64,
    pub right: u64,
    pub width: u8,
}

pub struct Flags {
    negative: bool,
    zero: bool,
    overflow: bool,
    carry: bool,
}
pub struct CalcOut {
    adds: CalcResult,
    subs: CalcResult,
}

impl Calc {
    pub fn try_new(left: u64, right: u64, width: u8) -> Result<Self, AppError> {
        let limit = 2u64.pow(width as u32) - 1;
        if left > limit {
            return Err(AppError::Other(anyhow!(
                "{left} is too large for bit {width} bits"
            )));
        }
        if right > limit {
            return Err(AppError::Other(anyhow!(
                "{right} is too large for bit {width} bits"
            )));
        }
        if width > 64 || width < 8 {
            return Err(AppError::Other(anyhow!(
                "{width} bit precisions is unsupported"
            )));
        }

        Ok(Self { left, right, width })
    }

    fn as_sub(&self) -> (u64, u64) {
        (
            self.left,
            to_width((!self.right).overflowing_add(1u64).0, self.width),
        )
    }

    pub fn calc(&self) -> CalcOut {
        let Self { left, right, width } = *self;

        let (sub_l, sub_r) = self.as_sub();

        CalcOut {
            adds: alu_add(left, right, width),
            subs: alu_add(sub_l, sub_r, width),
        }
    }
}

fn to_width(n: u64, width: u8) -> u64 {
    let limit = 2u64.pow(width as u32) - 1;
    let result = n & if width >= 64 {
        u64::MAX
    } else {
        2u64.pow(width as u32) - 1
    };
    assert!(
        result <= limit,
        "{result} is larger than {limit} for {width} bits"
    );

    result
}

fn alu_add(left: u64, right: u64, width: u8) -> CalcResult {
    // adapted from ALU:
    let mut extractor = 0b1u64;
    let mut previous_carry = false;
    let mut carry = false;
    let mut result = 0u64;
    for _ in 0..width {
        let li = left & extractor != 0;
        let ri = right & extractor != 0;

        // simulate three input addition gate
        let bit_sum = carry as u64 + li as u64 + ri as u64;
        let result_i = bit_sum % 2 == 1;

        previous_carry = carry;
        carry = (bit_sum / 2) == 1;

        if result_i {
            result += extractor;
        }

        extractor = extractor << 1;
    }

    log!(
        "MERBUG calc MSB l/r = {}/{}, carry={carry}",
        left & (extractor >> 1),
        right & (extractor >> 1)
    );
    // did we go from pos to neg (or vice-versa)
    let overflow = carry ^ previous_carry;
    // is MSB set?
    let negative = result & 1 << (width - 1) != 0;
    let zero = result == 0;

    (
        result,
        Flags {
            negative,
            zero,
            carry,
            overflow,
        },
    )
}

fn pretty_num_sep(s: &str, every_n: usize) -> String {
    let mut out = String::new();

    let mut sep_counter = every_n - (s.len() % every_n);
    for (idx, char_) in s.chars().enumerate() {
        if sep_counter == 0 && idx != s.len() - 1 {
            out.push('\'');
            sep_counter = every_n;
        }
        out.push(char_);
        sep_counter = sep_counter.saturating_sub(1);
    }
    log!("MERBUG pretty num {out}");
    out
}

fn pretty_bin(n: u64, width: u8) -> String {
    let bin = format!("{n:0width$b}", width = width as usize);
    format!("0b{}", pretty_num_sep(&bin, 4))
}

fn pretty_hex(n: u64, width: u8) -> String {
    let bin = format!("{n:0width$X}", width = (width / 4) as usize);
    format!("0x{}", pretty_num_sep(&bin, 4))
}
fn pretty_dec(n: u64, width: u8) -> String {
    let bin = format!("{n:0width$}", width = (width / 3) as usize);
    format!("{}", pretty_num_sep(&bin, 3))
}

fn as_signed(n: u64, width: u8) -> i64 {
    let msb_set = n & (1 << (width - 1)) != 0;
    // sign extend
    let res = if msb_set {
        let to_keep = 64 - n.leading_zeros();
        log!("MERBUG as_signed to_keep  {to_keep}");
        let set_mask = !(2u64.pow(to_keep) - 1);
        (n | set_mask) as i64
        // to_width(((!n).overflowing_add(1)).0, width) as i64
    } else {
        n as i64
    };

    log!("MERBUG as_signed msb={msb_set} , from {n:0b}");
    log!("MERBUG as_signed res={res}");
    res
}

#[component]
pub fn CalcTable(out: CalcOut, width: u8) -> impl IntoView {
    view! {
        <table>
            <thead>
                <tr>
                    <th></th>
                    <th>Binary</th>
                    <th>Hex</th>
                    <th>Decimal</th>
                    <th>Decimal-Signed</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th>ADDS</th>
                    <td>{pretty_bin(out.adds.0, width)}</td>
                    <td>{pretty_hex(out.adds.0, width)}</td>
                    <td>{pretty_dec(out.adds.0, width)}</td>
                    <td>{as_signed(out.adds.0, width)}</td>
                </tr>
                <tr>
                    <th>SUBS</th>
                    <td>{pretty_bin(out.subs.0, width)}</td>
                    <td>{pretty_hex(out.subs.0, width)}</td>
                    <td>{pretty_dec(out.subs.0, width)}</td>
                    <td>{as_signed(out.subs.0, width)}</td>
                </tr>
            </tbody>
        </table>
        <h3>Flags</h3>
        <table>
            <thead>
                <tr>
                    <th></th>
                    <th>Negative</th>
                    <th>Zero</th>
                    <th>Carry</th>
                    <th>Overflow</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th>ADDS</th>
                    <td>{out.adds.1.negative}</td>
                    <td>{out.adds.1.zero}</td>
                    <td>{out.adds.1.carry}</td>
                    <td>{out.adds.1.overflow}</td>
                </tr>
                <tr>
                    <th>SUBS</th>
                    <td>{out.subs.1.negative}</td>
                    <td>{out.subs.1.zero}</td>
                    <td>{out.subs.1.carry}</td>
                    <td>{out.subs.1.overflow}</td>
                </tr>
            </tbody>
        </table>
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn separators() {
        assert_eq!(pretty_num_sep("0123", 2), "01'23");
        assert_eq!(pretty_num_sep("123", 2), "1'23");
        assert_eq!(pretty_num_sep("23", 2), "23");
        assert_eq!(pretty_num_sep("3", 2), "3");
        assert_eq!(pretty_num_sep("01234", 2), "0'12'34");
        assert_eq!(pretty_num_sep("01234456789", 2), "0'12'34'45'67'89");

        assert_eq!(pretty_num_sep("0123", 3), "0'123");
        assert_eq!(pretty_num_sep("123", 3), "123");
        assert_eq!(pretty_num_sep("23", 3), "23");
        assert_eq!(pretty_num_sep("3", 3), "3");
        assert_eq!(pretty_num_sep("01234", 3), "01'234");
        assert_eq!(pretty_num_sep("01234456789", 3), "01'234'456'789");
    }
}
