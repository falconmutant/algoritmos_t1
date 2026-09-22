pub fn is_palindrome(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();

    for i in 0..chars.len() / 2 {
        if chars[i] != chars[chars.len() - 1 - i] {
            return false;
        }
    }
    true
}