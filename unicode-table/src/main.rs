use clap::Parser;
use selector::Selector;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Error, Write};
use std::ops::RangeInclusive;
use std::path::PathBuf;

mod list;
mod selector;
mod table;

#[derive(Parser)]
struct Args {
    profile: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

/// A character range with associated property
struct Entry<'a> {
    range: RangeInclusive<u32>,
    property: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The Unicode database is alvailable at:
    // https://www.unicode.org/Public/UCD/latest/ucd/
    let charsets = BTreeMap::from([
        ("props", list::parse(include_str!("PropList.txt"))),
        ("scripts", list::parse(include_str!("Scripts.txt"))),
        ("categs", table::parse(include_str!("UnicodeData.txt"))),
    ]);

    let Args { profile, output } = Args::parse();
    let profile = File::open(profile)?;
    let selectors: BTreeMap<String, Selector> = serde_yml::from_reader(profile)?;

    let charsets: Vec<_> = selectors
        .into_iter()
        .map(|(name, selector)| {
            let charset = selector.select(&charsets);
            let mut charset: Vec<u32> = charset.into_iter().collect();
            charset.sort_unstable();
            (name.to_uppercase(), name, range_compression(&charset))
        })
        .collect();

    let output: &mut dyn Write = if let Some(output) = output {
        &mut File::create(output)?
    } else {
        &mut std::io::stdout().lock()
    };

    writeln!(output, "{}", include_str!("template.c"))?;
    for (_, name, charset) in &charsets {
        print_table(output, &charset, &name)?;
    }
    writeln!(output)?;
    for (name_uppercase, _, charset) in &charsets {
        print_len(output, charset.len(), &name_uppercase)?;
    }
    writeln!(output)?;
    for (_, name, charset) in &charsets {
        print_macro(output, charset.len(), &name)?;
    }

    Ok(())
}

/// Replace consecutive values by an inclusive range
fn range_compression(values: &[u32]) -> Vec<RangeInclusive<u32>> {
    assert!(values.is_sorted());
    let [head, tail @ ..] = values else {
        return Vec::new();
    };

    let mut ranges = Vec::new();
    let mut min = *head;
    let mut max = *head;

    for &elem in tail {
        if max + 1 < elem {
            ranges.push(min..=max);
            min = elem;
        }
        max = elem;
    }

    ranges.push(min..=max);
    ranges
}

fn print_macro(output: &mut dyn Write, len: usize, name: &str) -> Result<(), Error> {
    writeln!(
        output,
        "#define is_{name}(c) unicode_classify(unicode_table_{name}, 0, {len}, c)",
    )
}

fn print_len(output: &mut dyn Write, len: usize, name_uppercase: &str) -> Result<(), Error> {
    writeln!(output, "#define UNICODE_TABLE_LEN_{name_uppercase} {len}",)
}

fn print_table(
    output: &mut dyn Write,
    ranges: &[RangeInclusive<u32>],
    name: &str,
) -> Result<(), Error> {
    let len = ranges.len();
    writeln!(
        output,
        "static struct unicode_range unicode_table_{name}[{len}] = {{",
    )?;
    for range in ranges {
        writeln!(
            output,
            "    {{0x{:0>8x}, 0x{:0>8x}}},",
            range.start(),
            range.end()
        )?;
    }
    writeln!(output, "}};")?;
    Ok(())
}
