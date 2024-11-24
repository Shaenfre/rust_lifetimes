struct StrSplit {
    remainder: &str,
    delim: &str
}

impl StrSplit {
    fn new(input_str: &str, delim: &str) -> Self {
        Self {
            remainder: input_str,
            delim,
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str
    fn next(&mut self) -> Option<Self::Item> {

    }
}

#[test]
fn it_works() {
    let input_str = "a b c d e";
    let letters = StrSplit::new(input_str, " ").collect;
    asserteq!(input_str, letters);

}