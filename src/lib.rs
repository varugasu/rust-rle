pub fn lre_encode(data: String) -> String {
    let mut encoded_data = String::new();

    if data.is_empty() {
        return encoded_data;
    }

    let mut count = 0;
    let mut current_char = data.chars().next().unwrap();
    for c in data.chars() {
        if c == current_char {
            count += 1;
        } else {
            encoded_data.push(current_char);
            encoded_data.push_str(&count.to_string());
            current_char = c;
            count = 1;
        }
    }

    encoded_data.push(current_char);
    encoded_data.push_str(&count.to_string());

    encoded_data
}

pub fn lre_decode(data: String) -> String {
    data
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
