use str;


// TODO http://dev.w3.org/csswg/css3-syntax/#the-input-byte-stream
// fn decode(input: &[u8], protocol_encoding: &str, link_encoding: &str,
//           document_encoding: &str) -> ~str
// Use empty strings for “no such encoding information”?


// http://dev.w3.org/csswg/css3-syntax/#preprocessing-the-input-stream
pub pure fn preprocess(input: &str) -> ~str {
    // TODO: Is this faster if done in one pass?
    str::replace(str::replace(str::replace(input,
    "\r\n", "\n"),
    "\r", "\n"),
    "\x00", "\uFFFD")
}


#[test]
fn test_preprocess() {
    assert preprocess("") == ~"";
    assert preprocess("Lorem\r\n\t\x00ipusm\ndoror\uFFFD\r")
        == ~"Lorem\n\t\uFFFDipusm\ndoror\uFFFD\n";
}


const ASCII_LOWER_OFFSET: char = 'a' - 'A';

pub pure fn ascii_lower(string: &str) -> ~str {
    do str::map(string) |c| {
        match c {
            'A'..'Z' => c + ASCII_LOWER_OFFSET,
            _ => c,
        }
    }
}


#[test]
fn test_ascii_lower() {
    assert ascii_lower("url()URL()uRl()Ürl") == ~"url()url()url()Ürl";
    // Dotted capital I, Kelvin sign, Sharp S.
    assert ascii_lower("HİKß") == ~"hİKß";
}
