use anyhow::{anyhow, bail, Result};
use std::{char::from_u32_unchecked, env::args, ops::RangeInclusive};

const TAG_MASK: u32 = 0xE0000;
const TAG_RANGE: RangeInclusive<u32> = 0x20..=0x7E;
const CANCEL_TAG: char = '\u{E007F}';

fn tagify(prefix: &str, text: &str) -> Result<String> {
    let prefix: Vec<char> = prefix.chars().collect();
    let text_as_tags: String = text
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if TAG_RANGE.contains(&(c as u32)) {
                let tag = TAG_MASK | (c as u32);
                Ok(unsafe { from_u32_unchecked(tag) })
            } else {
                Err(anyhow!(
                    "character {:?} at {} is not non-control 7bit ascii",
                    c,
                    i
                ))
            }
        })
        .collect::<Result<String>>()?;
    let prefix_char = match prefix.as_slice() {
        [c] => c,
        prefix => bail!("prefix {:?} is not a single codepoint", prefix),
    };
    Ok(format!("{}{}{}", prefix_char, text_as_tags, CANCEL_TAG))
}

fn main() -> Result<()> {
    let cmdline: Vec<String> = args().collect();
    if let [_pgm, prefix, text] = cmdline.as_slice() {
        let text = tagify(prefix, text)?;
        eprintln!("Created a sequence {} bytes long", text.bytes().len());
        println!("{}", text);
        Ok(())
    } else {
        bail!("USAGE: unicode-tagger CHAR TEXT\n{:?}", cmdline.as_slice())
    }
}
