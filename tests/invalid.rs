#![cfg(feature = "invalidtests")]
mod x {
    #![flexpect::e(clippy::clone_on_copy)]

    fn foo() {
        #[flexpect::e(clippy::clone_on_copy)]
        let _ = 42.clone();
    }

    fn bar() {
        #[flexpect::e(clippy::clone_on_copy)]
        {
            let _ = 42.clone();
        }
    }
}
