#[cfg(feature = "words")]
use num2words::{Num2Err, Num2Words};

#[cfg(feature = "words")]
pub fn format_to_word<T>(amount: T) -> Result<String, Num2Err>
where
    T: Into<f64> + Copy,
{
    let value = amount.into();
    let naira_part = value.floor() as i64;

    let kobo_part = ((value - naira_part as f64).abs() * 100.0).round() as i64;

    let naira_words = Num2Words::new(naira_part).to_words()?;
    let mut result = format!("{} Naira", naira_words);

    if kobo_part > 0 {
        let kobo_words = Num2Words::new(kobo_part).to_words()?;
        result = format!("{}, {} Kobo", result, kobo_words);
    }

    Ok(format!("{} Only", result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "words")]
    fn test_format_to_word_basic() {
        let result = format_to_word(1250.50).unwrap();
        assert_eq!(
            result,
            "one thousand two hundred and fifty Naira, fifty Kobo Only"
        );
    }

    #[test]
    #[cfg(feature = "words")]
    fn test_format_to_word_no_kobo() {
        let result = format_to_word(100.00).unwrap();
        assert_eq!(result, "one hundred Naira Only");
    }

    #[test]
    #[cfg(feature = "words")]
    fn test_format_to_word_singular() {
        let result = format_to_word(1.01).unwrap();
        assert_eq!(result, "one Naira, one Kobo Only");
    }
}
