fn foo() -> Result<usize, usize> {
    Ok(3)
}

pub fn bar() -> Result<usize, usize> {
    foo()
}
