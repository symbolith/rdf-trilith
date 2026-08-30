#[derive(Debug, Clone, Copy, PartialEq, Eq, snafu::Snafu)]
pub enum ValidationError {
    #[snafu(display("invalid blank node label"))]
    InvalidBlankNodeLabel,
    #[snafu(display("invalid language tag"))]
    InvalidLanguageTag,
}
