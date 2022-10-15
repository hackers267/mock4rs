use crate::char::random_number;
use crate::pick_one;

pub fn phone() -> String {
    let prefixes = [
        "130", "131", "132", "133", "134", "135", "136", "137", "138", "139", "151", "152", "186",
        "182",
    ];
    let prefix = pick_one(&prefixes);
    let mut result = String::from(prefix);
    let mut i = 0;
    while i < 8 {
        let s = random_number();
        result += &s;
        i += 1;
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone() {
        let result = phone();
        let len = result.len();
        assert_eq!(len, 11);
    }
}
