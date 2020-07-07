fn u8_from_hex(c: char) -> Result<u8, String> {
    match c.to_digit(16) {
        Some(i) => Ok(i as u8),
        _ => Err(format!("invalid character {}", c).into()),
    }
}

pub fn from_hex(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err("input length needs to be multiple of 2".into());
    }

    let mut digits = Vec::with_capacity(s.len());
    for c in s.chars() {
        digits.push(u8_from_hex(c)?);
    }
    Ok(digits
        .chunks(2)
        .map(|c| (c[0] << 4) + c[1])
        .collect::<Vec<u8>>())
}

fn u8_to_base64(u: u8) -> char {
    match u {
        0..=25 => (b'A' + u) as char,
        26..=51 => (b'a' + (u - 26)) as char,
        52..=61 => (b'0' + (u - 52)) as char,
        62 => '+',
        63 => '/',
        _ => panic!("input exceeded range"),
    }
}

fn block_to_base64(block: &[u8], base64: &mut String) {
    let (a, b, c) = match block.len() {
        3 => (block[0], block[1], block[2]),
        2 => (block[0], block[1], 0),
        1 => (block[0], 0, 0),
        _ => return,
    };
    base64.push(u8_to_base64(a >> 2)); // Upper 6 bits of a
    base64.push(u8_to_base64(a << 6 >> 2 | b >> 4)); // Lower 2 bits of a, upper 4 bits of b
    base64.push(u8_to_base64((b << 4 | c >> 4) >> 2)); // Lower 4 bits of b, upper 2 bits of c
    base64.push(u8_to_base64(c << 2 >> 2)); // Lower 6 bits of c
}

fn hex_to_base64(hex_string: &str) -> String {
    let hex: Vec<u8> = from_hex(&hex_string).unwrap();
    let mut base64 = String::with_capacity(4 * hex.len() / 3);
    for block in hex.chunks(3) {
        block_to_base64(block, &mut base64);
    }

    if hex.len() % 3 >= 1 {
        base64.pop();
        if hex.len() % 3 == 1 {
            base64.pop();
            base64.push('=');
        }
        base64.push('=');
    }

    return base64;
}

fn main() {
    let hex_string: String = String::from("858685");
    hex_to_base64(&hex_string);
}

#[test]
fn test_s1c1() {
    assert_eq!(
        hex_to_base64(&String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    )
}
