use leptos::*;

use crate::CalcOut;

fn pretty_num_sep(s: &str, every_n: usize) -> String {
    let mut out = String::new();

    let mut sep_counter = s.len() % every_n;

    // this would insert a separator at the start
    if sep_counter == 0 {
        sep_counter = every_n;
    }
    for (idx, char_) in s.chars().enumerate() {
        if sep_counter == 0 && idx != s.len() - 1 {
            out.push('\'');
            sep_counter = every_n;
        }
        out.push(char_);
        sep_counter = sep_counter.saturating_sub(1);
    }
    out
}
fn pretty_bin(n: u64, width: u8) -> String {
    let bin = format!("{n:0width$b}", width = width as usize);
    format!("0b{}", pretty_num_sep(&bin, 4))
}

fn pretty_hex(n: u64, width: u8) -> String {
    let hex = format!("{n:0width$X}", width = (width / 4) as usize);
    format!("0x{}", pretty_num_sep(&hex, 4))
}
fn pretty_dec(n: u64, width: u8) -> String {
    let dec = format!("{n:0width$}", width = (width / 3) as usize);
    format!("{}", pretty_num_sep(&dec, 3))
}
fn pretty_dec_signed(n: i64, width: u8) -> String {
    if n >= 0 {
        let dec_uns = format!("{n:0width$}", width = (width / 3) as usize);
        format!("{}", pretty_num_sep(&dec_uns, 3))
    } else {
        let dec_uns = format!(
            "{n_abs:0width$}",
            n_abs = n.abs(),
            width = (width / 3) as usize
        );
        format!("-{}", pretty_num_sep(&dec_uns, 3))
    }
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
                    <th>Decimal (signed)</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th>ADDS</th>
                    <td>{pretty_bin(out.adds.0.unsigned, width)}</td>
                    <td>{pretty_hex(out.adds.0.unsigned, width)}</td>
                    <td>{pretty_dec(out.adds.0.unsigned, width)}</td>
                    <td>{out.adds.0.signed}</td>
                </tr>
                <tr>
                    <th>SUBS</th>
                    <td>{pretty_bin(out.subs.0.unsigned, width)}</td>
                    <td>{pretty_hex(out.subs.0.unsigned, width)}</td>
                    <td>{pretty_dec(out.subs.0.unsigned, width)}</td>
                    <td>{out.subs.0.signed}</td>
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
