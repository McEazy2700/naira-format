use naira_format::*;

fn main() {
    assert_eq!(format_naira(1234567.89), "₦1,234,567.89");
    assert_eq!(format_kobo_to_naira(150000), "₦1,500.00");
    assert_eq!(format_naira_to_kobo(75.50), "7550");
    assert_eq!(format_naira_compact(2_500_000), "₦2.5M");
    let amount = parse_naira("₦1,500.50").unwrap();
    assert_eq!(amount, 1500.5);
}
