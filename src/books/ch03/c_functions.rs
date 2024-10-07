pub fn functions_signature(x: u32) -> u32 {
    // statement and expression
    /*
    Function bodies are made up of a series of statements optionally ending in an expression.
    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value. Let’s look at some example
     */
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::books::ch03::c_functions::functions_signature;

    #[test]
    fn functions_signature_test() {
        let x = 5;
        assert_eq!(functions_signature(x), 6);
    }
}