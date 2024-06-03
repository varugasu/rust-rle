pub fn lre_encode(data: String) -> String {
    data
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
