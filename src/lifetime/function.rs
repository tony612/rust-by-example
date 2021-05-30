fn longest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
    }
}

// 9 | fn pick_first(s1: &String, _s2: &String) -> &String {
//   |                   -------       -------     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `_s2`
// help: consider introducing a named lifetime parameter
// fn pick_first(s1: &String, s2: &String) -> &String {
//     return s1;
// }

fn pick_first<'a>(s1: &'a String, _s2: &String) -> &'a String {
    return s1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s1 = String::from("looooong");
        let s2 = String::from("short");
        assert_eq!(longest(&s1, &s2), "looooong");
    }

    #[test]
    fn test_invalid() {
        let s1 = String::from("looooong");
        let s2 = String::from("short");
        let result = longest(&s1, &s2);

        // result lifetime is min(s1, s2), so have to be dropped before
        // s1 and s2
        //
        // move out of `s1` occurs here
        // drop(s1);
        // move out of `s2` occurs here
        // drop(s2);

        assert_eq!(result, "looooong");
    }

    #[test]
    fn test_partial_mark() {
        let s1 = String::from("looooong");
        let s2 = String::from("short");
        let result = pick_first(&s1, &s2);

        // move out of `s1` occurs here
        // drop(s1);

        // ok because result 'a is only same with s1,
        // so s2 can be dropped early
        drop(s2);

        assert_eq!(result, "looooong");
    }
}
