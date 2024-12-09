pub mod grid;
mod parser;

pub use parser::Parser;

#[macro_export]
macro_rules! modules {
    ( $( $num:expr ),* ) => {
        paste::paste! {
            $(
                mod [<task_ $num>];
            )*

        pub static TASKS_NORMAL: &[fn(input: &str) -> usize] = &[
            $(
                <[<task_ $num>]::[<Task $num>] as super::TaskRun>::normal,
            )*
        ];

        pub static TASKS_BONUS: &[fn(input: &str) -> usize] = &[
            $(
                <[<task_ $num>]::[<Task $num>] as super::TaskRun>::bonus,
            )*
        ];
        }

    };
}
