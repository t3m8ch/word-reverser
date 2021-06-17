#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_text_words_simple_text() {
        let text = "Hello ,   world!";

        let actual = flip_text_words(text);
        let expected = "olleH ,   dlrow!";

        assert_eq!(actual, expected);
    }

    #[test]
    fn flip_text_words_multiline_text() {
        let text = "\
! Hello, world!
I came from   Mars   and brought  peace...";

        let actual = flip_text_words(text);
        let expected = "\
! olleH, dlrow!
I emac morf   sraM   dna thguorb  ecaep...";

        assert_eq!(actual, expected);
    }

    #[test]
    fn flip_text_words_unicode_text() {
        let text = "Hi, people!    Привет ,  мир!";

        let actual = flip_text_words(text);
        let expected = "iH, elpoep!    тевирП ,  рим!";

        assert_eq!(actual, expected);
    }
}

pub fn flip_text_words(text: &str) -> String {
    // TODO: Add to the list
    let separators = [
        ' ', '.', ',', ';', ':', '!',
        '?', '-', '"', '(', ')', '\n',
    ];
    let chars: Vec<char> = text.chars().into_iter().collect();

    let mut is_separator = false;
    let mut indexes: Vec<usize> = chars.iter().enumerate()
        .filter_map(|(i, l)| {
            let previous_state = is_separator;
            is_separator = separators.contains(&l);
            if previous_state != is_separator { Some(i) } else { None }
        })
        .collect();
    indexes.push(chars.len());

    let mut is_separator = false;
    let mut previous_i = 0;

    indexes.into_iter()
        .map(|i| {
            let mut item = Vec::new();
            &chars[previous_i..i].iter()
                .for_each(|l| item.push(l.clone()));

            if !is_separator {
                item.reverse();
            }

            let item = item.iter().collect::<String>();

            is_separator = !is_separator;
            previous_i = i;

            item
        })
        .collect::<Vec<_>>()
        .iter()
        .flat_map(|s| s.chars())
        .collect::<String>()
}
