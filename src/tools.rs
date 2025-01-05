use std::sync::OnceLock;

use tifloats::Float;
use titokens::{Token, Tokenizer, Tokens, Version};

pub fn tokenizer() -> &'static Tokenizer {
    static TOKENIZER: OnceLock<Tokenizer> = OnceLock::new();

    TOKENIZER.get_or_init(|| Tokenizer::new(Version::latest(), "en"))
}

pub fn tokenize(text: &str) -> Tokens {
    tokenizer().tokenize(text).unwrap().0
}

pub fn byte_count(tokens: &[Token]) -> usize {
    tokens
        .iter()
        .map(|token| match token {
            Token::OneByte(_) => 1,
            Token::TwoByte(_, _) => 2,
        })
        .sum()
}

pub fn process_submission(submission: String) -> Tokens {
    let filtered = submission
        .lines()
        .filter(|&line| !(line.is_empty() || line.starts_with("//")))
        .collect::<Vec<_>>()
        .join("\n");

    let (tokens, _boundaries) = tokenizer().tokenize(&filtered).unwrap();
    tokens
}

pub fn float_to_tifloat(value: f64) -> Float {
    if value == 0.0 {
        return Float::new_unchecked(false, 0, 0);
    }

    let exponent = value.abs().log10().floor() as i8;

    let mut rescaled = (10.0f64).powi(-exponent as i32) * value.abs();
    let mut digits = Vec::with_capacity(15);
    for _ in 0..15 {
        digits.push(rescaled.trunc() as u8);
        rescaled = rescaled.fract() * 10.0;
    }

    Float::new(
        value.is_sign_negative(),
        exponent,
        Float::mantissa_from(&digits),
    )
    .unwrap()
}
