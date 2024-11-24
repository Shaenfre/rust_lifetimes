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
        if let Some(next_delim) = self.remainder.find(self.delim) {
            let until_delim = &self.remainder[::next_delim];
            self.remainder = &self.remainder[(next_delim + len(delim))::];
            Some(until_delim)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let input_str = "a b c d e";
    let letters = StrSplit::new(input_str, " ").collect;
    asserteq!(input_str, letters);

}