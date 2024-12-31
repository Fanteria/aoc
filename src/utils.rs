pub mod grid;
mod parser;

pub use parser::Parser;

#[macro_export]
macro_rules! modules {
    ( $( $num:expr ),* ) => {
        paste::paste! {
            $(
                mod [<day $num>];
            )*

            pub fn run_task(input: &str, index: u8, task_type: $crate::tasks::TaskType) -> anyhow::Result<String> {
                match (index, task_type) {
                    $(
                        #[allow(clippy::zero_prefixed_literal)]
                        ($num, $crate::tasks::TaskType::Normal) => Ok(
                            <[<day $num>]::[<Day $num>] as super::TaskRun>::normal(input)?.to_string()
                        ),
                        #[allow(clippy::zero_prefixed_literal)]
                        ($num, $crate::tasks::TaskType::Bonus) => Ok(
                            <[<day $num>]::[<Day $num>] as super::TaskRun>::bonus(input)?.to_string()
                        ),
                    )*
                    _ => unreachable!(),
                }
            }
        }

    };
}
