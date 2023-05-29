`ngrams` computes the frequency of sequences of characters ([n-grams](https://en.wikipedia.org/wiki/N-gram)) in a text file.

## Example

Here's an example of using `ngrams` to count the number of occurences of each two-character sequence in a sentence.

```
$ echo "she sells seashells by the seashore" | ngrams -n 2
3 he
3 se
3 sh
2 as
2 ea
2 el
2 ll
2 ls
1 by
1 ho
1 or
1 re
1 th
```

## Usage

```
Usage: ngrams [OPTIONS] -n <N> [FILENAME]

Arguments:
  [FILENAME]  File to read from. If no file is provided, STDIN is used

Options:
  -n <N>                    Size of ngram to collect (2 for bigrams, 3 for trigrams, etc)
  -f, --as-fraction         Output decimal fractions for each ngram (rather than counts)
  -p, --as-percent          Output percentages for each ngram (rather than counts)
  -w, --include-whitespace  Include ngrams that contain whitespace characters
  -j, --json                Output ngrams and their frequences as JSON
  -h, --help                Print help
  -V, --version             Print version
```

## JSON output

The JSON output format is an array of length N, containing frequencies for ngrams of each length from 1 (individual characters) up to N. Each entry in the array is an object whose keys are ngrams and whose values are either counts, percentages, or decimal fractions (depending on whether `-p` or `-f` was used).

```
echo "she sells seashells by the seashore" | ngrams -n 2 --json
[
  {
    "a": 2,
    "b": 1,
    "e": 7,
    "h": 4,
    "l": 4,
    "o": 1,
    "r": 1,
    "s": 8,
    "t": 1,
    "y": 1
  },
  {
    "as": 2,
    "by": 1,
    "ea": 2,
    "el": 2,
    "he": 3,
    "ho": 1,
    "ll": 2,
    "ls": 2,
    "or": 1,
    "re": 1,
    "se": 3,
    "sh": 3,
    "th": 1
  }
]
```

## Bar charts

It can be useful to visualize the relative frequencies of ngrams graphically. You can pipe the output of `ngrams` into [`barchart`](https://github.com/jake-low/barchart) to accomplish this.

Here's a barchart showing the 10 most common trigrams in Herman Melville's novel _Moby-Dick_.

```
$ ngrams -n 3 -p moby-dick.txt | head | barchart
  3.252% the ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  1.392% ing ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  1.298% and ■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.685% his ■■■■■■■■■■■■■■■
  0.673% hat ■■■■■■■■■■■■■■
  0.602% her ■■■■■■■■■■■■■
  0.597% tha ■■■■■■■■■■■■■
  0.540% ere ■■■■■■■■■■■■
  0.496% all ■■■■■■■■■■■
  0.430% for ■■■■■■■■■
```

## License

The source code for `ngrams` is available under the ISC license. See the LICENSE file for details.