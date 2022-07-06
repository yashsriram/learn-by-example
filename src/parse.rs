use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MultiChoiceMultiCorrectQuestionHtml {
    prompt: String,
    options: String,
    hint: String,
    truth: String,
    explaination: String,
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
    println!("{:?}", events);
    // h1s
    let h1_start_idxes: Vec<_> = events
        .iter()
        .enumerate()
        .filter_map(|(idx, event)| match event {
            Event::Start(Tag::Heading(HeadingLevel::H1, None, ..)) => Some(idx),
            _ => None,
        })
        .collect();
    // h1s as expected?
    let found_h1s = h1_start_idxes
        .iter()
        // for every start end is guranteed => this will not be out of bounds
        .map(|idx| events[idx + 1].clone())
        .collect::<Vec<_>>();
    let required_h1s = vec![
        Event::Text(CowStr::Borrowed("prompt")),
        Event::Text(CowStr::Borrowed("options")),
        Event::Text(CowStr::Borrowed("hint")),
        Event::Text(CowStr::Borrowed("explaination")),
    ];
    let required_h1s_are_found = required_h1s == found_h1s;
    if !required_h1s_are_found {
        return Err(format!(
            "required h1s: {:?}\nfound h1s   : {:?}",
            required_h1s, found_h1s,
        ));
    }
    // delimit based on h1s
    let prompt_range = h1_start_idxes[0]..h1_start_idxes[1];
    let hint_range = h1_start_idxes[2]..h1_start_idxes[3];
    let truth_range = h1_start_idxes[1]..h1_start_idxes[2];
    let explaination_range = h1_start_idxes[3]..;
    // parsed -> html string
    let prompt_html_string = {
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            events[prompt_range].iter().map(|event| event.clone()),
        );
        html_string
    };
    println!("{:?}", &events[truth_range.clone()]);
    let options_html_string = {
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            events[truth_range.clone()].iter().map(|event| match event {
                Event::TaskListMarker(..) => Event::TaskListMarker(false).clone(),
                _ => event.clone(),
            }),
        );
        html_string
    };
    let hint_html_string = {
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            events[hint_range].iter().map(|event| event.clone()),
        );
        html_string
    };
    let truth_html_string = {
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            events[truth_range.clone()].iter().map(|event| match event {
                Event::Text(CowStr::Borrowed("options")) => Event::Text(CowStr::Borrowed("truth")),
                _ => event.clone(),
            }),
        );
        html_string
    };
    let explaination_html_string = {
        let mut html_string = String::new();
        html::push_html(
            &mut html_string,
            events[explaination_range].iter().map(|event| event.clone()),
        );
        html_string
    };
    let truth_values = events[truth_range.clone()]
        .iter()
        .filter_map(|event| match event {
            Event::TaskListMarker(val) => Some(*val),
            _ => None,
        })
        .collect();
    Ok(MultiChoiceMultiCorrectQuestionHtml {
        prompt: prompt_html_string,
        options: options_html_string,
        hint: hint_html_string,
        truth: truth_html_string,
        explaination: explaination_html_string,
        truth_values,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good1() {
        let md_string = "
# prompt
what is 2 + 2?

# options
- [x] 4.
- [x] 2 x 2.
- [ ] 3.
- [x] -2 x -2.

# hint

# explaination
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    fn good2() {
        let md_string = "
# prompt
what is 2 + 2?

# options
- [x] 4.
- [x] 2 x 2.
- [ ] 3.
- [x] -2 x -2.
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad1() {
        let md_string = "
# prompt
what is 2 + 2?
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad2() {
        let md_string = "
# options
- [x] 4.
- [x] 2 x 2.
- [ ] 3.
- [x] -2 x -2.
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad3() {
        let md_string = "
# prompt
what is 2 + 2?

# options
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad4() {
        let md_string = "
# prompt
what is 2 + 2?

# options
- [x] 4.
- [x] 2 x 2.
some non task list content
- [ ] 3.
- [x] -2 x -2.
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }
}
