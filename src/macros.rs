#[macro_export]
macro_rules! day {
    ($name:ident, $day: expr, $year: expr) => {
        pub struct $name {
            input: String,
        }

        impl InputReciever for $name {}

        impl $name {
            pub fn new() -> Result<Self, Error> {
                use InputReciever;

                let input = Self::recieve_input($day, $year)?;
                Ok(Self { input })
            }
        }
    };
}
