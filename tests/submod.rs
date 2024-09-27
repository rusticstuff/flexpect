#![cfg(feature = "submodtests")]

#[flexpect::flexpect(unused_variables, clippy::clone_on_copy)]
mod submod_correct {
    #[test]
    fn compiler_warning_correctly_flexpected() {
        let x = 1;
    }

    #[test]
    fn clippy_warning_correctly_flexpected() {
        let _ = 32.clone();
    }
}

#[flexpect::flexpect(unused_variables, clippy::clone_on_copy)]
mod submod_incorrect {
    #[test]
    fn compiler_warning_incorrectly_flexpected() {}

    #[test]
    fn clippy_warning_incorrectly_flexpected() {}
}
