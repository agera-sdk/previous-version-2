pub enum Expression {
    Empty,
    SingleCharacter(char),
    Text(String),
    Sequence(Vec<Arc<Expression>>),
}