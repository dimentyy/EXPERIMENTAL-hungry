use std::io::{Write, Result};

use crate::{Cfg, F};
use crate::code::write_escaped;
use crate::meta::Combinator;

pub(super) fn write_identifiable(f: &mut F, _cfg: &Cfg, x: &Combinator) -> Result<()> {
    f.write_all(b"\nimpl _tl::Identifiable for ")?;
    write_escaped(f, &x.name.actual)?;
    f.write_all(b" {\n    const CONSTRUCTOR_ID: u32 = 0x")?;

    write!(f, "{:08x}", x.explicit_id.unwrap_or(x.inferred_id))?;

    f.write_all(b";\n}\n")
}
