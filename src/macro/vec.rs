#[allow(unused_macros)]
macro_rules! avec {
    () => {
        Vec::new()
    };
    // ($elem:expr) => {{
    //     let mut vs = Vec::new();
    //     vs.push($elem);
    //     vs
    // }};
    ($($elem:expr),+ $(,)?) => {{
        const C: usize = $crate::elem_count![@COUNT; $($elem),*];
        let mut vs = Vec::with_capacity(C);

        // let mut vs = Vec::new();
        $(
            vs.push($elem);
        )+
        vs
    }};
    ($elem:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $elem);

        // let mut vs = Vec::with_capacity($count);
        // vs.extend(::std::iter::repeat($elem).take($count));

        // let x = $elem;
        // for _ in 0..$count {
        //     vs.push(x);
        // }
        vs
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! elem_count {
    (@COUNT; $($elem:expr),*) => {
        <[()]>::len(&[$($crate::elem_count![@SUBST; $elem]),*])
    };
    (@SUBST; $elem:expr) => {
        ()
    }
}

#[test]
fn test_empty() {
    let x: Vec<usize> = avec![];
    assert!(x.is_empty());
    let x: Vec<usize> = avec!();
    assert!(x.is_empty());
}

#[test]
fn test_single() {
    let x: Vec<usize> = avec![42];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn test_multiple() {
    let x: Vec<usize> = avec![42, 43];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);

    let x: Vec<usize> = avec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(x.len(), 10);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
    assert_eq!(x[9], 10);
}

#[test]
fn test_tailing() {
    let x: Vec<usize> = avec![42,];
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);

    let x: Vec<usize> = avec![1, 2,];
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 1);
    assert_eq!(x[1], 2);
}

#[test]
fn test_repeat() {
    let x: Vec<usize> = avec![42; 3];
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}

#[test]
fn test_repeat_non_literal() {
    let mut num: Option<usize> = Some(42);
    let x: Vec<usize> = avec![num.take().unwrap(); 3];
    assert_eq!(x.len(), 3);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
    assert_eq!(x[2], 42);
}
