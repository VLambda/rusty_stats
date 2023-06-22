mod inner_derivative {
        pub fn definition(f: fn(f64) -> f64, x: f64) -> f64 {
                let definition = (f(x + f64::EPSILON) - f(x)) / f64::EPSILON;
                if (definition.floor() + 1e-6) > definition || (definition.ceil() - 1e-6) < definition {
                        definition.round()
                } else {
                        definition
                }
        }
}

pub use self::inner_derivative::definition;
