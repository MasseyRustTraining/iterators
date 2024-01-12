fn min_max<T, I>(vals: I) -> Option<(T, T)>
where
    T: Ord + Copy,
    I: IntoIterator<Item=T>,
{
    let mut iter = vals.into_iter();
    let first = iter.next()?;
    let mut min = first;
    let mut max = first;
    for v in iter {
        min = v.min(min);
        max = v.max(max);
    }
    Some((min, max))
}

fn some_numbers() -> impl Iterator<Item=u8> {
    let mut val = 7;
    std::iter::from_fn(move || {
        if val >= 10 {
            None
        } else {
            val += 1;
            Some(val)
        }
    })
}

fn main() {
    #[allow(clippy::useless_conversion)]
    for i in (0u8..3).into_iter() {
        println!("{}", i);
    }

    for i in 0u8..3 {
        println!("{}", i);
    }

    let v = vec!["a", "b"];
    for s in &v {
        println!("{}", s);
    }

    let mut v_iter = v.iter();
    #[allow(clippy::while_let_on_iterator)]
    while let Some(s) = v_iter.next() {
        println!("{}", s);
    }

    let evens = [2u8, 16, 56];
    assert!(evens.iter().all(|x| x % 2 == 0));

    let (min, max) = min_max(evens).unwrap();
    println!("{} {}", min, max);

    for i in some_numbers() {
        println!("{}", i);
    }
}
