// https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&t=2887s
fn ref_mut(mut s: Option<&str>) -> Option<&str> {
    // s_ref: &mut &str
    if let Some(ref mut s_ref) = s {
        *s_ref = "";
        s
    } else {
        None
    }
}

fn ref_mut2(mut s: Option<&str>) -> Option<&str> {
    // s_ref: &mut &str
    // same as ref_mut
    let s_ref = s.as_mut()?;
    *s_ref = "";
    s
}

fn ref_mut3(s: Option<&str>) -> Option<&str> {
    // s_ref: &str
    // s_ref is copy of Value of s
    let mut s2 = s?;
    let _ = s2; // remove warning
    s2 = &"";
    let _ = s2; // remove warning
    s
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Foo {
    id: usize,
}

fn and_mut(input: Option<&mut Foo>) -> Option<Foo> {
    // Some(&mut s) only pattern match with
    // Some(&mut T)
    // then s = T, and T must have Copy because move happens here
    if let Some(&mut s) = input {
        Some(s)
    } else {
        None
    }
}

#[test]
fn it_works() {
    let s = String::from("abc");
    assert_eq!(ref_mut(Some(&s)), Some(""));
    assert_eq!(ref_mut(None), None);

    let s = String::from("abc");
    assert_eq!(ref_mut2(Some(&s)), Some(""));

    let s = String::from("abc");
    assert_eq!(ref_mut3(Some(&s)), Some("abc"));
}

#[test]
fn it_works2() {
    let mut input = Foo { id: 42 };
    assert_eq!(and_mut(Some(&mut input)), Some(Foo { id: 42 }));
    assert_eq!(ref_mut(None), None);
}
