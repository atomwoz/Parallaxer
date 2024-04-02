use std::cmp::min;
pub fn slice_multiline<'a>(lines: &Vec<&'a str>, start: usize, end: usize) -> Vec<String> {
    let mut to_ret = Vec::new();
    for x in lines {
        let chars = x.chars().collect::<Vec<_>>();
        to_ret.push(chars[start..end].iter().cloned().collect::<String>());
    }
    to_ret
}

pub fn get_multiline_width(lines: &Vec<&str>) -> usize {
    let mut min_width = usize::MAX;
    for x in lines {
        min_width = min(x.chars().count(), min_width);
    }
    min_width
}
pub fn load_train() -> Vec<&'static str> {
    include_str!("assets/train.txt").lines().collect()
}

pub fn load_tunnel() -> Vec<&'static str> {
    include_str!("assets/tunnel.txt").lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_multiline() {
        let lines = vec!["Hello,", "World!", "This is", "a test"];
        let start = 2;
        let end = 6;
        let expected = vec!["llo,", "rld!", "is", " test"];
        assert_eq!(slice_multiline(&lines, start, end), expected);
    }

    #[test]
    fn test_get_multiline_width() {
        let lines = vec!["Hello", "World!", "This is a test"];
        let expected = 5;
        assert_eq!(get_multiline_width(&lines), expected);
    }
}
