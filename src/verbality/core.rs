use std::fmt::Write;

use crate::verbality::Gender;
use crate::*;

pub fn verbalize_number<L: Verbalizer, W: Write>(
    lang: &L,
    n: u64,
    out: &mut W,
) -> Result<(), VerbalizeError> {
    if n == 0 {
        return out.write_str(lang.zero()).map_err(VerbalizeError::Fmt);
    }

    let base = lang.chunk_base();
    assert!(base > 1, "base should be greater than 1");
    let mut max_scale = 0;
    let mut temp = n;
    while temp >= base {
        temp /= base;
        max_scale += 1;
    }

    let mut divisor = base.pow(max_scale as u32);
    let mut need_sep = false;

    for scale_idx in (0..=max_scale).rev() {
        let chunk = (n / divisor) % base;
        divisor /= base;

        if chunk == 0 {
            continue;
        }

        if need_sep {
            out.write_char(' ')?;
        }
        need_sep = true;

        write_chunk(lang, chunk, lang.unit_gender_for_scale(scale_idx), out)?;

        if scale_idx > 0 {
            out.write_char(' ')?;
            let form = lang.plural_for_chunk(chunk, scale_idx);
            out.write_str(lang.scale_form(scale_idx, form))?;
        }
    }

    Ok(())
}

fn write_chunk<L: Verbalizer, W: Write>(
    lang: &L,
    n: u64,
    unit_gender: Gender,
    out: &mut W,
) -> Result<(), VerbalizeError> {
    let h = (n / 100) as usize;
    let t = ((n / 10) % 10) as usize;
    let u = (n % 10) as usize;

    let mut need_sep = false;
    let mut write = |word: &str| -> Result<(), VerbalizeError> {
        if need_sep {
            out.write_char(' ')?;
        }
        out.write_str(word)?;
        need_sep = true;
        Ok(())
    };

    if h > 0 {
        write(lang.hundred(h))?;
    }

    match t {
        1 => write(lang.teen(u))?,
        2..=9 => {
            write(lang.ten(t))?;
            if u > 0 {
                write(lang.unit(u, unit_gender))?;
            }
        }
        _ if u > 0 => write(lang.unit(u, unit_gender))?,
        _ => {}
    }

    Ok(())
}
