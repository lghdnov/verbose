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

    let mut temp = n;
    let mut max_scale = 0usize;
    while temp >= lang.chunk_base() {
        temp /= lang.chunk_base();
        max_scale += 1;
    }

    let mut divisor = 1u64;
    for _ in 0..max_scale {
        divisor *= lang.chunk_base();
    }

    let mut first = true;
    for scale_idx in (0..=max_scale).rev() {
        let chunk = (n / divisor) % lang.chunk_base();
        divisor /= lang.chunk_base();

        if chunk == 0 {
            continue;
        }

        if !first {
            out.write_char(' ')?;
        }
        first = false;

        write_chunk::<L, W>(lang, chunk, lang.unit_gender_for_scale(scale_idx), out)?;

        if scale_idx > 0 {
            out.write_char(' ')?;
            let form = lang.plural_for_chunk(chunk, scale_idx);
            out.write_str(lang.scale_form(scale_idx, form))?;
        }
    }
    Ok(())
}

fn write_word<W: Write>(
    out: &mut W,
    word: &str,
    need_space: &mut bool,
) -> Result<(), VerbalizeError> {
    if *need_space {
        out.write_char(' ')?;
    }
    out.write_str(word)?;
    *need_space = true;
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

    let mut need_space = false;
    let mut write = |w: &str| -> Result<(), VerbalizeError> { write_word(out, w, &mut need_space) };

    if h > 0 {
        write(lang.hundred(h))?;
    }
    if t == 1 {
        write(lang.teen(u))?;
    } else {
        if t > 1 {
            write(lang.ten(t))?;
        }
        if u > 0 {
            write(lang.unit(u, unit_gender))?;
        }
    }
    Ok(())
}
