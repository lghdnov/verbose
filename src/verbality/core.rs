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

    let max_scale = n.ilog(base);
    let mut divisor = base.pow(max_scale);
    let mut writer = WordWriter::new(out);

    for scale_idx in (0..=max_scale).rev() {
        let chunk = (n / divisor) % base;
        divisor /= base;

        if chunk == 0 {
            continue;
        }

        let g = lang.unit_gender_for_scale(scale_idx as usize);
        verbalize_chunk(lang, chunk, g, &mut writer)?;
        verbalize_scale(lang, chunk, scale_idx as usize, &mut writer)?;
    }

    Ok(())
}

fn verbalize_chunk<L: Verbalizer, W: Write>(
    lang: &L,
    n: u64,
    unit_gender: Gender,
    writer: &mut WordWriter<W>,
) -> Result<(), VerbalizeError> {
    let hundreds = (n / 100) as usize;
    let tens = ((n / 10) % 10) as usize;
    let units = (n % 10) as usize;

    if hundreds > 0 {
        writer.write(lang.hundred(hundreds))?;
    }

    match (tens, units) {
        (1, u) => writer.write(lang.teen(u))?,
        (t, u) => {
            if t >= 2 {
                writer.write(lang.ten(t))?;
            }
            if u != 0 {
                writer.write(lang.unit(u, unit_gender))?;
            }
        }
    }

    Ok(())
}

fn verbalize_scale<L: Verbalizer, W: Write>(
    lang: &L,
    ch: u64,
    scale_idx: usize,
    writer: &mut WordWriter<W>,
) -> Result<(), VerbalizeError> {
    if scale_idx > 0 {
        let form = lang.plural_for_chunk(ch, scale_idx);
        writer.write(lang.scale_form(scale_idx, form))?;
    }

    Ok(())
}

struct WordWriter<'a, W: Write> {
    out: &'a mut W,
    first: bool,
}

impl<'a, W: Write> WordWriter<'a, W> {
    fn new(out: &'a mut W) -> Self {
        Self { out, first: true }
    }

    fn write(&mut self, s: &str) -> Result<(), VerbalizeError> {
        if !self.first {
            self.out.write_char(' ').map_err(VerbalizeError::Fmt)?;
        }
        self.out.write_str(s).map_err(VerbalizeError::Fmt)?;
        self.first = false;
        Ok(())
    }
}
