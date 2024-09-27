#[test]
#[flexpect::flexpect(unused_variables)]
fn compiler_warning_correctly_flexpected() {
    let x = 1;
}

#[flexpect::flexpect(clippy::clone_on_copy)]
#[test]
fn clippy_warning_correctly_flexpected() {
    let _ = 32.clone();
}

#[test]
#[flexpect::flexpect(unused_variables)]
fn compiler_warning_incorrectly_flexpected() {}

#[flexpect::flexpect(clippy::clone_on_copy)]
#[test]
fn clippy_warning_incorrectly_flexpected() {}
