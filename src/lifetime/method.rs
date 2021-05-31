struct MyStruct {}

impl MyStruct {
    fn empty(&self, _msg: &str) -> &str {
        ""
    }

    fn msg<'a>(&self, msg: &'a str) -> &'a str {
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_lifetime() {
        let m = MyStruct {};
        let msg = String::from("msg");
        let result = m.empty(&msg);
        // result lifetime is same with m because of Lifetime Elision rule 3
        // drop(m);
        drop(msg);
        assert_eq!(result, "");
    }

    #[test]
    fn test_other_lifetime() {
        let m = MyStruct {};
        let msg = String::from("msg");
        let result = m.msg(&msg);
        // we change result's lifetime to msg, so it's safe to drop m but can't drop msg
        drop(m);
        // drop(msg);
        assert_eq!(result, "msg");
    }
}
