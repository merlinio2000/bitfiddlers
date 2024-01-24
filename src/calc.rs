use anyhow::anyhow;
use leptos::logging::log;

use crate::AppError;

#[derive(Debug, Copy, Clone)]
pub struct Calc {
    pub left: u64,
    pub right: u64,
    pub width: u8,
}

pub struct Flags {
    pub negative: bool,
    pub zero: bool,
    pub overflow: bool,
    pub carry: bool,
}
pub struct CalcOutValue {
    pub unsigned: u64,
    pub signed: i64,
}
type CalcOutResult = (CalcOutValue, Flags);
pub struct CalcOut {
    pub adds: CalcOutResult,
    pub subs: CalcOutResult,
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
        // TODO: can probably be replaced with alu_add
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

fn alu_add(left: u64, right: u64, width: u8) -> CalcOutResult {
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
        CalcOutValue {
            unsigned: result,
            signed: as_signed(result, width),
        },
        Flags {
            negative,
            zero,
            carry,
            overflow,
        },
    )
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
