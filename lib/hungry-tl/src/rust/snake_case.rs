pub(crate) fn snake_case(s: &str) -> String {
    let chars = s.chars();

    let mut s = String::with_capacity(s.len() + 20);

    let mut underscore = true;

    let mut previous = '_';

    for c in chars {
        match c {
            '_' => {
                if !underscore {
                    s.push('_');

                    underscore = true;
                }
            }
            c @ '0'..='9' => {
                if !underscore && !previous.is_ascii_digit() {
                    s.push('_');

                    underscore = true;
                } else {
                    underscore = false;
                }

                s.push(c);
            }
            c @ 'a'..='z' => {
                s.push(c);

                underscore = false;
            }
            c @ 'A'..='Z' => {
                if !underscore {
                    s.push('_');

                    underscore = true;
                }

                s.push(c.to_ascii_lowercase());
            }
            _ => todo!(),
        }

        previous = c;
    }

    s.shrink_to_fit();

    s
}
