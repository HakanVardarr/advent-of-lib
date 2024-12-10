#[macro_export]
macro_rules! day {
    ($name:ident, $day: expr, $year: expr) => {
        pub struct $name {
            input: String,
        }

        impl advent_of_lib::InputReciever for $name {}

        impl $name {
            pub fn new() -> Result<Self, advent_of_lib::Error> {
                use advent_of_lib::InputReciever;

                let input = Self::recieve_input($day, $year)?;
                Ok(Self { input })
            }
        }
    };
}
