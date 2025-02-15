use std::fmt::Display;

fn do_things<T>(a: T, b: T)
where
    T: IntoIterator,
    T::Item: Display,
{
    for (e, f) in a.into_iter().zip(b.into_iter()) {
        println!("{} {}", e, f);
    }
}
