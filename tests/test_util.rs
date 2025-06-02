#[macro_export]
macro_rules! note_vec {
    ($($note:expr),*) => {
        vec![$(note!($note)),*]
    };
}
