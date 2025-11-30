use std::io::{Write, Result};

use crate::{Cfg, F};
use crate::code::write_name;
use crate::meta::{Data, GenericArg, Name, Typ};

pub(super) fn write_typ(
    f: &mut F,
    cfg: &Cfg,
    data: &Data,
    generic_args: &[GenericArg],
    typ: &Typ,
    turbofish: bool
) -> Result<()> {
    let typ: &[u8] = match typ {
        Typ::Type { index, params } => {
            if !params.is_empty() {
                unimplemented!()
            }

            let x = &data.types[*index];

            return write_name(f, "types", &x.combinator.name);
        }
        Typ::Enum { index, params } => {
            if !params.is_empty() {
                unimplemented!()
            }

            let x = &data.enums[*index];
            
            return write_name(f, "enums", &x.name);
        }
        Typ::Int => b"i32",
        Typ::Long => b"i64",
        Typ::Double => b"f64",
        Typ::Bytes => if turbofish { b"Vec::<u8>" } else { b"Vec<u8>" },
        Typ::String => b"String",
        Typ::Bool => b"bool",
        Typ::BareVector(typ) => {
            f.write_all(if turbofish { b"_tl::BareVec::<" } else { b"_tl::BareVec<" })?;
            write_typ(f, cfg, data, generic_args, typ, false)?;
            b">"
        }
        Typ::Vector(typ) => {
            f.write_all(if turbofish { b"Vec::<" } else { b"Vec<" })?;
            write_typ(f, cfg, data, generic_args, typ, false)?;
            b">"
        }
        Typ::Int128 => b"_tl::Int128",
        Typ::Int256 => b"_tl::Int256",
        Typ::Generic { index } => generic_args[*index].name.as_bytes(),
    };

    f.write_all(typ)
}
