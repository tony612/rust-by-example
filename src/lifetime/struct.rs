struct MyStruct<'a> {
    refer: &'a String,
}

impl<'a> MyStruct<'a> {
    fn empty(&self, _msg: &str) -> &str {
        ""
    }

    fn msg<'b>(&self, msg: &'b str) -> &'b str {
        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetime() {
        let s = String::from("str");
        let m = MyStruct { refer: &s };

        // m borrows s
        // drop(s);
        assert_eq!(m.refer, "str");

        let msg = String::from("msg");
        let result = m.empty(&msg);
        // result lifetime is same with m, so it's borrowed below
        // drop(s);
        assert_eq!(result, "");

        let result = m.msg(&msg);
        // it's safe to drop s because we specify result lifetime is same with msg
        // but can't drop msg
        drop(s);
        // drop(msg);
        assert_eq!(result, "msg");
    }
}
