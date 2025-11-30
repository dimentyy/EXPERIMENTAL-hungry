use indexmap::IndexMap;

use crate::meta::{Error, Name, Typ};
use crate::read;

#[derive(Debug)]
pub struct Flag {
    pub arg: usize,
    pub bit: usize,
}

impl Flag {
    pub(crate) fn find(args: &IndexMap<&str, Arg>, flag: &read::Flag) -> Result<Self, Error> {
        let Some(index) = args.get_index_of(flag.ident) else {
            unimplemented!()
        };

        match &args.get_index(index).unwrap().1.typ {
            ArgTyp::Flags => Ok(Self {
                arg: index,
                bit: flag.bit.unwrap_or(0),
            }),
            ArgTyp::Typ { .. } => unimplemented!(),
            ArgTyp::True { .. } => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum ArgTyp {
    Flags,
    Typ { typ: Typ, flag: Option<Flag> },
    True { flag: Flag }
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub typ: ArgTyp,
}

#[derive(Debug)]
pub struct GenericArg {
    pub name: String
}

#[derive(Debug)]
pub struct Combinator {
    pub name: Name,
    pub explicit_id: Option<u32>,
    pub inferred_id: u32,
    pub args: Vec<Arg>,
    pub generic_args: Vec<GenericArg>,
}
