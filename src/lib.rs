//! A Statistics Library made in pure Rust, for Rust!

mod extra {
    pub mod derivative;
}

#[cfg(test)]
mod tests {
    use super::*;
    use extra::derivative::definition;

    #[test]
    fn it_works() {
        let derivative = definition(|x| x.powi(2), 1.0);
        assert_eq!(derivative, 2.0);
    }
}
