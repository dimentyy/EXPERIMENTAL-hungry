use std::fmt;

use chumsky::prelude::*;

use crate::read::{Ident, ParserExtras};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OptArgsTyp {
    Type,
}

impl OptArgsTyp {
    fn parser<'src>() -> impl ParserExtras<'src, Self> {
        choice((just("Type").to(OptArgsTyp::Type),))
    }
}

impl fmt::Display for OptArgsTyp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            OptArgsTyp::Type => "Type",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptArgs<'a> {
    pub idents: Vec<&'a str>,
    pub typ: OptArgsTyp,
}

impl fmt::Display for OptArgs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            f.write_str("{")?;
        }
        let mut names = self.idents.iter();
        let Some(name) = names.next() else {
            return Err(fmt::Error);
        };
        f.write_str(name)?;
        for name in names {
            f.write_str(" ")?;
            f.write_str(name)?;
        }
        f.write_str(":")?;
        self.typ.fmt(f)?;
        if !f.alternate() {
            f.write_str("}")?;
        }
        Ok(())
    }
}

impl<'src> OptArgs<'src> {
    pub(super) fn parser() -> impl ParserExtras<'src, Self> {
        let names = Ident::part_parser()
            .padded()
            .separated_by(text::whitespace())
            .at_least(1)
            .collect();
        let typ = OptArgsTyp::parser();

        just('{')
            .ignore_then(names)
            .then_ignore(just(':').padded())
            .then(typ.padded())
            .then_ignore(just('}'))
            .map(|(idents, typ)| Self { idents, typ })
    }
}
