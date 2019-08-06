pub struct Match<'a> {
    pub line: &'a str,
    pub indexes: Vec<usize>,
}