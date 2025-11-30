pub(crate) enum Casing {
    Upper,
    Lower,
    None,
}

impl Casing {
    #[inline]
    pub(crate) fn apply_to(&self, c: char) -> char {
        match self {
            Casing::Upper => c.to_ascii_uppercase(),
            Casing::Lower => c.to_ascii_lowercase(),
            Casing::None => c,
        }
    }
}
