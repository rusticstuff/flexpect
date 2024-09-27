#[test]
#[flexpect::expect(unused_variables)]
fn compiler_warning_correctly_flexpected() {
    let x = 1;
}

#[flexpect::expect(clippy::clone_on_copy)]
#[test]
fn clippy_warning_correctly_flexpected() {
    let _ = 32.clone();
}

#[test]
#[flexpect::expect(unused_variables)]
fn compiler_warning_incorrectly_flexpected() {}

#[flexpect::expect(clippy::clone_on_copy)]
#[test]
fn clippy_warning_incorrectly_flexpected() {}
