# `mul`
A Bengali stemmer written in Rust.

`mul` currently implements a stepwise approach to removing inflections from Bengali words <sup>[1](#references)</sup>.

## Example
``` rust
use mul::noun_stemmer;

fn main() {
    assert_eq!(noun_stemmer("বাংলায়"), "বাংলা");
    assert_eq!(noun_stemmer("মানুষকে"), "মানুষ");
}
```

## References
1. M. R. Mahmud, M. Afrin, M. A. Razzaque, E. Miller and J. Iwashige, "A rule based bengali stemmer,"
_2014 International Conference on Advances in Computing, Communications and Informatics (ICACCI)_,
2014, pp. 2750-2756, doi: 10.1109/ICACCI.2014.6968484.
