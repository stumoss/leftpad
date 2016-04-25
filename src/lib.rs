/// Pads the front of a string with a user specified char or spaces if no char
/// is provided. The original string will be returned if the pad value is less
/// than the length of the original string.
///
/// # Examples
///
/// ```
/// use leftpad::leftpad;
///
/// let space_padded_string = leftpad("hello", 7, None);
/// let hash_padded_string = leftpad("hello", 7, Some('#'));
/// let original_string = leftpad("hello", 0, None);
/// ```
pub fn leftpad(s: &str, pad: usize, ch: Option<char>) -> String
{
    let mut len = 0;
    if pad > s.chars().count() {
        len = pad - s.chars().count();
    }

    format!("{}{}", std::iter::repeat(ch.unwrap_or(' ')).take(len).collect::<String>(), s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_leftpad() {
        assert_eq!(leftpad("foo", 0, None), "foo");
        assert_eq!(leftpad("foo", 2, None), "foo");
        assert_eq!(leftpad("foo", 5, None), "  foo");
        assert_eq!(leftpad("bar", 0, Some('Y')), "bar");
        assert_eq!(leftpad("foo", 7, Some('X')), "XXXXfoo");
        assert_eq!(leftpad("bar", 5, Some('-')), "--bar");
        assert_eq!(leftpad("bar", 15, Some('-')), "------------bar");
    }
}
