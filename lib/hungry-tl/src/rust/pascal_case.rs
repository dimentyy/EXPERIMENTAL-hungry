use crate::casing::Casing;

pub(crate) fn pascal_case(s: &str) -> String {
    let chars = s.chars();
    let mut s = String::with_capacity(s.len());
    let mut casing = Casing::Upper;

    for c in chars {
        match c {
            '_' => {
                casing = Casing::Upper;

                continue;
            }
            c @ '0'..='9' => {
                casing = Casing::Upper;

                s.push(c);

                continue;
            }
            c if !c.is_ascii_alphabetic() => todo!(),
            _ => {}
        }

        s.push(casing.apply_to(c));

        casing = if c.is_uppercase() {
            Casing::Lower
        } else {
            Casing::None
        }
    }

    s.shrink_to_fit();

    s
}
