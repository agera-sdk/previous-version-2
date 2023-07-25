pub enum Expression {
    Empty,
    SingleCharacter(char),
    Text(String),
    /// Multiple expressions.
    Sequence(Vec<Arc<Expression>>),
    /// `\k<Name>`
    BackReference(String),
}