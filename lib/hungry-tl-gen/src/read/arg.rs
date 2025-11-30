use std::fmt;

use chumsky::prelude::*;

use crate::read::{Extra, Ident, Typ};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Arg<'a> {
    pub ident: &'a str,
    pub typ: ArgTyp<'a>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgTyp<'a> {
    Typ {
        excl_mark: bool,
        typ: Typ<'a>,
        flag: Option<Flag<'a>>,
    },
    Nat,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Flag<'a> {
    pub ident: &'a str,
    pub bit: Option<usize>,
}

impl fmt::Display for Flag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.ident)?;
        if let Some(bit) = self.bit {
            f.write_str(".")?;
            f.write_str(&bit.to_string())?;
        }
        f.write_str("?")
    }
}

impl<'src> Flag<'src> {
    fn parser() -> impl Parser<'src, &'src str, Self, Extra<'src>> {
        let ident = Ident::part_parser();
        let bit = just('.').ignore_then(text::int(10).from_str().unwrapped());

        ident
            .then(bit.or_not())
            .then_ignore(just('?'))
            .map(|(ident, bit)| Flag { ident, bit })
    }
}

impl fmt::Display for ArgTyp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgTyp::Typ {
                flag,
                typ,
                excl_mark,
            } => {
                if *excl_mark {
                    f.write_str("!")?;
                }
                if let Some(flag) = flag {
                    flag.fmt(f)?;
                }
                typ.fmt(f)
            }
            ArgTyp::Nat => f.write_str("#"),
        }
    }
}

impl<'src> ArgTyp<'src> {
    pub fn parser() -> impl Parser<'src, &'src str, Self, Extra<'src>> {
        let nat = just('#').to(ArgTyp::Nat);

        let excl_mark = just('!').or_not().map(|x| x.is_some());

        let typ = excl_mark
            .then(Flag::parser().or_not())
            .then(Typ::parser(Ident::parser()))
            .map(|((excl_mark, flag), typ)| ArgTyp::Typ {
                excl_mark,
                flag,
                typ,
            });

        choice((nat, typ))
    }
}

impl fmt::Display for Arg<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.ident)?;
        f.write_str(":")?;
        self.typ.fmt(f)
    }
}

impl<'src> Arg<'src> {
    pub fn parser() -> impl Parser<'src, &'src str, Self, Extra<'src>> {
        let ident = Ident::part_parser();

        ident
            .then_ignore(just(':'))
            .then(ArgTyp::parser())
            .map(|(ident, typ)| Arg { ident, typ })
    }
}
