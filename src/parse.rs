use pulldown_cmark::{html, CowStr, Event, Options, Parser};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MultiChoiceMultiCorrectQuestionHtml {
    truth_removed_html: String,
    truth_html: String,
    truth_values: Vec<bool>,
}

pub fn parse(md_string: String) -> Result<MultiChoiceMultiCorrectQuestionHtml, String> {
    // parse
    let mut md_parse_options = Options::empty();
    md_parse_options.insert(Options::ENABLE_TABLES);
    md_parse_options.insert(Options::ENABLE_FOOTNOTES);
    md_parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    md_parse_options.insert(Options::ENABLE_TASKLISTS);
    md_parse_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    md_parse_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(&md_string, md_parse_options);
    let events: Vec<_> = parser.collect();
    let number_of_options = events
        .iter()
        .filter(|event| match event {
            Event::TaskListMarker(..) => true,
            _ => false,
        })
        .count();
    if number_of_options < 1 {
        return Err(format!(
            "required min of 1 options, found {}",
            number_of_options,
        ));
    }
    let truth_removed_html = {
        let truth_removed: Vec<_> = events
            .iter()
            .map(|event| match event {
                Event::Text(CowStr::Borrowed("truth")) => Event::Text(CowStr::Borrowed(
                    "select all true (multiple or zero can be true)",
                )),
                Event::TaskListMarker(..) => Event::TaskListMarker(false).clone(),
                _ => event.clone(),
            })
            .collect();
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            truth_removed.into_iter().map(|event| event),
        );
        html_string
    };
    let truth_html = {
        let mut html_string = String::new();
        html::push_html(&mut html_string, events.iter().map(|event| event.clone()));
        html_string
    };
    let truth_values = events
        .iter()
        .filter_map(|event| match event {
            Event::TaskListMarker(val) => Some(*val),
            _ => None,
        })
        .collect();
    Ok(MultiChoiceMultiCorrectQuestionHtml {
        truth_removed_html,
        truth_html,
        truth_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_bad1() {
        let md_string = "- [ ] 1 + 1 = 3.".to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    fn valid_good1() {
        let md_string = "- [ ] 1 + 1 = 3.".to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    fn valid_good2() {
        let md_string = "- [x] 1 * 1 = 1.".to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    fn valid_good3() {
        let md_string = "what is 2 + 2? \
- [x] 4.
- [ ] 3.
- [x] -2 x -2.
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn invalid() {
        let md_string = "what is 2 + 2?".to_string();
        assert!(parse(md_string).is_ok());
    }
}
