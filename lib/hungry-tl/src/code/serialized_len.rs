use std::io::{Write, Result};

use crate::{Cfg, F};
use crate::code::write_escaped;
use crate::meta::{Combinator, Name};

pub(super) fn write_serialized_len(f: &mut F, _cfg: &Cfg, name: &Name, size: usize) -> Result<()> {
    f.write_all(b"\nimpl _tl::SerializedLen for ")?;
    write_escaped(f, &name.actual)?;
    f.write_all(b" {\n    const SERIALIZED_LEN: usize = ")?;
    write!(f, "{size}")?;
    f.write_all(b";\n}\n")
}
