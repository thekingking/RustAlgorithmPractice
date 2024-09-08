struct Solution;

impl Solution {
    pub fn convert_date_to_binary(date: String) -> String {
        let parts: Vec<&str> = date.split('-').collect();
        let year: u16 = parts[0].parse().expect("Invalid year");
        let month: u8 = parts[1].parse().expect("Invalid month");
        let day: u8 = parts[2].parse().expect("Invalid day");
        let year_bin = format!("{:b}", year);
        let month_bin = format!("{:b}", month);
        let day_bin = format!("{:b}", day);
        year_bin + "-" + &month_bin + "-" + &day_bin
    }
}