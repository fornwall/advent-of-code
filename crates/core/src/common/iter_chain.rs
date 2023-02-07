#[macro_export]
macro_rules! chain {
    ($first:expr $(, $rest:expr )* $(,)?) => {
        {
            let iter = core::iter::IntoIterator::into_iter($first);
            $(
                let iter =
                    core::iter::Iterator::chain(
                        iter,
                        core::iter::IntoIterator::into_iter($rest));
            )*
            iter
        }
    };
}

