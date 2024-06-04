pub fn lre_encode(data: String) -> String {
    let mut encoded_data = String::new();
    let mut chars = data.chars().peekable();

    if let Some(mut current_char) = chars.next() {
        let mut count = 1;

        while let Some(&c) = chars.peek() {
            if c == current_char {
                count += 1
            } else {
                encoded_data.push(current_char);
                encoded_data.push_str(&count.to_string());
                current_char = c;
                count = 1;
            }
            chars.next();
        }

        encoded_data.push(current_char);
        encoded_data.push_str(&count.to_string());
    }

    encoded_data
}

pub fn lre_decode(data: String) -> String {
    let mut decoded_data = String::new();
    let mut chars = data.chars().peekable();

    while let Some(current_char) = chars.next() {
        let mut count = String::new();

        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                count.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if let Ok(n) = count.parse::<usize>() {
            decoded_data.push_str(&current_char.to_string().repeat(n));
        } else {
            panic!("Invalid count: {}", count);
        }
    }

    decoded_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lre_encode() {
        assert_eq!(lre_encode("AAABBBCCDAA".to_string()), "A3B3C2D1A2")
    }

    #[test]
    fn test_lre_decode() {
        assert_eq!(lre_decode("A3B3C2D1A2".to_string()), "AAABBBCCDAA")
    }
}
