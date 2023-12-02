use reqwest;

fn main() -> Result<(), reqwest::Error> {
    let url = "https://adventofcode.com/2023/day/1/input";

    let client = reqwest::blocking::Client::new();
    let response = client.get(url)
        .header(reqwest::header::COOKIE, "session=<session>")
        .send()?;

    let data = response.text()?;
    let mut fdigit: u32 = 0;
    let mut sdigit: u32 = 0;
    let mut result: u32 = 0;

    for line in data.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                fdigit = c.to_digit(10).unwrap_or(0) * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                sdigit = c.to_digit(10).unwrap_or(0);
                break;
            }
        }
        result += fdigit + sdigit;
    }
    println!("Result: {}", result);
    Ok(())
}