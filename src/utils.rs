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

            pub fn run_task(input: &str, index: u8, task_type: crate::tasks::TaskType) -> String {
                match (index, task_type) {
                    $(
                        ($num, crate::tasks::TaskType::Normal) => <[<task_ $num>]::[<Task $num>] as super::TaskRun>::normal(input).to_string(),
                        ($num, crate::tasks::TaskType::Bonus) => <[<task_ $num>]::[<Task $num>] as super::TaskRun>::bonus(input).to_string(),
                    )*
                    _ => unreachable!(),
                }
            }
        }

    };
}
