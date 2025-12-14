// Naira Format API
/// Format a number into Nigerian Naira format.
///
/// Examples:

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    InvalidFormat,
}

pub fn format_naira<T>(amount: T) -> String
where
    T: Into<f64>,
{
    let value = amount.into();
    let sign = if value < 0.0 { "-" } else { "" };
    let absolute_value = value.abs();

    // splitting the whole and fractional parts

    let whole = absolute_value.trunc() as i64;
    let decimal_part = ((absolute_value - absolute_value.trunc()) * 100.0).round() as u32;

    let formatted_whole = format_with_commas(whole);

    format!("₦{}{}.{:02}", sign, formatted_whole, decimal_part)
}

pub fn format_kobo_to_naira<T>(amount: T) -> String
where
    T: Into<f64>,
{
    let value = amount.into();
    let sign = if value < 0.0 { "-" } else { "" };
    let abs_kobo = value.abs().round() as i64;

    // Convert kobo to naira + decimal
    let whole_naira = abs_kobo / 100;
    let decimal_part = abs_kobo % 100;

    // Format whole naira with commas
    let formatted_whole = format_with_commas(whole_naira);

    format!("₦{}{}.{:02}", sign, formatted_whole, decimal_part)
}

pub fn format_naira_to_kobo<T>(amount: T) -> String
where
    T: Into<f64>,
{
    let value = amount.into();
    let sign = if value < 0.0 { "-" } else { "" };
    let kobo_amount = (value.abs() * 100.0).round() as i64;
    format!("{}{}", sign, kobo_amount)
}

pub fn format_naira_compact<T>(amount: T) -> String
where
    T: Into<f64>,
{
    let value = amount.into();
    let sign = if value < 0.0 { "-" } else { "" };
    let absolute_value = value.abs();
    if absolute_value < 1_000.0 {
        // Less than 1K: show full number
        let whole = absolute_value.trunc() as i64;
        let decimal_part = ((absolute_value - absolute_value.trunc()) * 100.0).round() as u32;
        if decimal_part == 0 {
            format!("₦{}{}", sign, whole)
        } else {
            format!("₦{}{}.{:02}", sign, whole, decimal_part)
        }
    } else if absolute_value < 1_000_000.0 {
        // Thousands: show with K suffix
        let thousands = absolute_value / 1_000.0;
        let whole_k = thousands.trunc() as i64;
        let decimal_k = ((thousands - thousands.trunc()) * 10.0).round() as u32;
        if decimal_k == 0 {
            format!("₦{}{}K", sign, whole_k)
        } else {
            format!("₦{}{}.{}K", sign, whole_k, decimal_k)
        }
    } else if absolute_value < 1_000_000_000.0 {
        // Millions: show with M suffix
        let millions = absolute_value / 1_000_000.0;
        let whole_m = millions.trunc() as i64;
        let decimal_m = ((millions - millions.trunc()) * 10.0).round() as u32;
        if decimal_m == 0 {
            format!("₦{}{}M", sign, whole_m)
        } else {
            format!("₦{}{}.{}M", sign, whole_m, decimal_m)
        }
    } else {
        // Billions: show with B suffix
        let billions = absolute_value / 1_000_000_000.0;
        let whole_b = billions.trunc() as i64;
        let decimal_b = ((billions - billions.trunc()) * 10.0).round() as u32;
        if decimal_b == 0 {
            format!("₦{}{}B", sign, whole_b)
        } else {
            format!("₦{}{}.{}B", sign, whole_b, decimal_b)
        }
    }
}

pub fn parse_naira(input: &str) -> Result<f64, ParseError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    // Detect sign
    let (sign, value) = if let Some(stripped) = trimmed.strip_prefix('-') {
        (-1.0, stripped)
    } else {
        (1.0, trimmed)
    };

    // Remove Naira symbol and commas
    let cleaned = value.replace(['₦', ','], "");

    // Parse to f64
    let number: f64 = cleaned.parse().map_err(|_| ParseError::InvalidFormat)?;

    Ok(sign * number)
}

fn format_with_commas(n: i64) -> String {
    let s = n.to_string();
    let sbyte = s.as_bytes();
    let mut result = String::new();

    let len = sbyte.len();
    for (i, &b) in sbyte.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(b as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_naira() {
        assert_eq!(format_naira(1234567.89), "₦1,234,567.89");
        assert_eq!(format_naira(-55000), "₦-55,000.00");
    }

    #[test]
    fn test_commas() {
        assert_eq!(format_with_commas(1), "1");
        assert_eq!(format_with_commas(1234567), "1,234,567");
        assert_eq!(format_with_commas(-55000), "-55,000");
    }

    #[test]
    fn test_format_kobo_to_naira() {
        assert_eq!(format_kobo_to_naira(1), "₦0.01");
        assert_eq!(format_kobo_to_naira(50), "₦0.50");
        assert_eq!(format_kobo_to_naira(100), "₦1.00");
        assert_eq!(format_kobo_to_naira(150000), "₦1,500.00");
        assert_eq!(format_kobo_to_naira(-7550), "₦-75.50");
    }

    #[test]
    fn test_format_naira_to_kobo() {
        assert_eq!(format_naira_to_kobo(0.01), "1");
        assert_eq!(format_naira_to_kobo(1234567.89), "123456789");
        assert_eq!(format_naira_to_kobo(-75.50), "-7550");
    }

    #[test]
    fn test_format_naira_compact() {
        assert_eq!(format_naira_compact(950), "₦950");
        assert_eq!(format_naira_compact(1200), "₦1.2K");
        assert_eq!(format_naira_compact(2500000), "₦2.5M");
    }
    #[test]
    fn test_parse_naira() {
        assert_eq!(parse_naira("₦1,500.50"), Ok(1500.5));
        assert_eq!(parse_naira("1500"), Ok(1500.0));
        assert_eq!(parse_naira("1,234"), Ok(1234.0));
        assert_eq!(parse_naira("-₦75.50"), Ok(-75.5));
        assert_eq!(parse_naira(""), Err(ParseError::EmptyInput));
        assert_eq!(parse_naira("abc"), Err(ParseError::InvalidFormat));
    }
}
