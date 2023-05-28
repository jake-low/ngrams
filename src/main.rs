use std::collections::{BTreeMap, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter;
use std::path::PathBuf;

use clap::Parser;
use counter::Counter;
use utf8_read::Reader as Utf8Reader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Size of ngram to collect (2 for bigrams, 3 for trigrams, etc)
    #[arg(short)]
    n: usize,

    /// Output decimal fractions for each ngram (rather than counts)
    #[arg(short = 'f', long)]
    as_fraction: bool,

    /// Output percentages for each ngram (rather than counts)
    #[arg(short = 'p', long, conflicts_with = "as_fraction")]
    as_percent: bool,

    /// Include ngrams that contain whitespace characters.
    #[arg(short = 'w', long)]
    include_whitespace: bool,

    /// Output ngrams and their frequences as JSON
    #[arg(short = 'j', long)]
    json: bool,

    /// File to read from. If no file is provided, STDIN is used.
    filename: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut reader: Box<dyn Read> = match args.filename.clone() {
        Some(filename) => Box::new(File::open(filename)?),
        None => Box::new(io::stdin()),
    };

    // ngrams contains n Counters counting each length-n sequence.
    // ngrams[0] is a counter for single characters, ngrams[1] is
    // a counter for bigrams, etc.
    let mut ngrams: Vec<Counter<String>> = vec![Counter::new(); args.n];

    // prev contains the previous n characters; prev[0] is the immediately
    // previous character, prev[1] the one before that, etc.
    let mut prev: VecDeque<char> = VecDeque::with_capacity(args.n + 1);

    // count ngrams by looping over the UTF-8 glyphs in the input
    for c in Utf8Reader::new(&mut reader).into_iter() {
        let c = c.unwrap();

        // for each desired size, use 'prev' to construct the ngram of that size
        // which ends at the current character, and increment the count for that
        // ngram in the counters vector.
        for n in 1..=args.n {
            if prev.len() < n - 1 {
                break;
            };

            let ngram: String = prev.range(0..n - 1).rev().chain(iter::once(&c)).collect();

            // skip ngrams that contain whitespace unless the -w argument was used
            if !args.include_whitespace && ngram.contains(char::is_whitespace) {
                break;
            }

            ngrams[n - 1][&ngram] += 1;
        }

        // update 'prev' by pushing in the current character and discarding
        // the oldest character (keeping the length <= n).
        prev.push_front(c);
        prev.truncate(args.n);
    }

    let res = match args.json {
        true => output_as_json(ngrams),
        false => output_as_text(&ngrams, &args),
    };

    // ignore EPIPE errors; if STDOUT is a pipe and has been closed by the reader
    // then this program should politely exit.
    match res {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Result::<(), io::Error>::Ok(())?,
        r => r?,
    };

    Ok(())
}

fn output_as_text(ngrams: &[Counter<String>], args: &Cli) -> Result<(), io::Error> {
    let grams = &ngrams[args.n - 1];

    let max_count = grams.k_most_common_ordered(1)[0].1;
    let max_count_digits = max_count.to_string().len();
    let total_count: usize = grams.total();

    for (seq, count) in grams.most_common_ordered() {
        if args.as_percent {
            let percent = 100.0 * (count as f32) / (total_count as f32);
            writeln!(io::stdout(), "{:>7.3}% {}", percent, seq)?;
        } else if args.as_fraction {
            let fraction = (count as f32) / (total_count as f32);
            writeln!(io::stdout(), "{:>7.5} {}", fraction, seq)?;
        } else {
            writeln!(io::stdout(), "{:>w$} {}", count, seq, w = max_count_digits)?;
        }
    }

    Ok(())
}

fn output_as_json(ngrams: Vec<Counter<String>>) -> Result<(), io::Error> {
    let ngram_hashmaps: Vec<BTreeMap<String, usize>> = ngrams
        .into_iter()
        .map(|grams| grams.into_iter().collect())
        .collect();
    serde_json::to_writer_pretty(io::stdout(), &ngram_hashmaps)?;
    Ok(())
}
