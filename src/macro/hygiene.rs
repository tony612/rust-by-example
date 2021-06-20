macro_rules! hygiene {
    () => {
        let x = 1;
        let _ = x;
    };
}

fn hygiene() -> usize {
    hygiene!();
    // no x in this scope
    // x += 1;
    let x = 1;
    x
}

macro_rules! across_scope {
    ($x:ident) => {
        $x += 1;
    };
}

fn across_scope() -> usize {
    let mut x = 1;
    across_scope!(x);
    x
}

#[test]
fn test_hygiene() {
    assert!(hygiene() == 1);
}

#[test]
fn test_across_scope() {
    assert!(across_scope() == 2);
}
