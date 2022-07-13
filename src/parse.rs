use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MultiChoiceMultiCorrectQuestionHtml {
    // prompt: String,
    truth_removed: String,
    truth: String,
    truth_values: Vec<bool>,
    // hint: String,
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
    // delimit according to h1s, best out of all the tags in common mark specs, ux wise
    // let h1_start_idxes: Vec<_> = events
    //     .iter()
    //     .enumerate()
    //     .filter_map(|(idx, event)| match event {
    //         Event::Start(Tag::Heading(HeadingLevel::H1, None, ..)) => Some(idx),
    //         _ => None,
    //     })
    //     .collect();
    // h1s as expected?
    // let found_h1s = h1_start_idxes
    //     .iter()
    //     // for every start end is guranteed => this will not be out of bounds
    //     .map(|idx| events[idx + 1].clone())
    //     .collect::<Vec<_>>();
    // let required_h1s = vec![
    //     Event::Text(CowStr::Borrowed("prompt")),
    //     Event::Text(CowStr::Borrowed("truth")),
    //     Event::Text(CowStr::Borrowed("hint")),
    // ];
    // let required_h1s_are_found = required_h1s == found_h1s;
    // if !required_h1s_are_found {
    //     return Err(format!(
    //         "required h1s: {:?}\nfound h1s   : {:?}",
    //         required_h1s, found_h1s,
    //     ));
    // }
    // delimit based on h1s
    // let prompt_range = h1_start_idxes[0]..h1_start_idxes[1];
    // // parsed -> html string
    // let prompt_html_string = {
    //     let mut html_string = String::new();
    //     html::push_html(
    //         &mut html_string,
    //         events[prompt_range].iter().map(|event| event.clone()),
    //     );
    //     html_string
    // };
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
    let truth_removed_html_string = {
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
        // let start_idxes = truth_removed
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, event)| match event {
        //         Event::Start(Tag::Item) => Some(idx),
        //         _ => None,
        //     });
        // let end_idxes = truth_removed
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, event)| match event {
        //         Event::End(Tag::Item) => Some(idx),
        //         _ => None,
        //     });
        // let mut options = vec![Event::Start(Tag::List(None))];
        // for (start_idx, end_idx) in start_idxes.zip(end_idxes) {
        //     options.append(&truth_removed[start_idx..=end_idx].clone())
        // }
        // let options = start_idxes
        //     .zip(end_idxes)
        //     .into_iter()
        //     .map(|(start_idx, end_idx)| )
        //     .map(|event| event.clone());
        let mut html_string = String::new();
        // for (start_idx, end_idx) in start_idxes.zip(end_idxes) {
        //     html::push_html(
        //         &mut html_string,
        //         ,
        //     );
        // }
        html::push_html(
            &mut html_string,
            truth_removed.into_iter().map(|event| event),
        );
        html_string
    };
    let truth_html_string = {
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
    // let hint_range = h1_start_idxes[2]..;
    // let hint_html_string = {
    //     let mut html_string = String::new();
    //     html::push_html(
    //         &mut html_string,
    //         events[hint_range].iter().map(|event| event.clone()),
    //     );
    //     html_string
    // };
    Ok(MultiChoiceMultiCorrectQuestionHtml {
        // prompt: prompt_html_string,
        truth_removed: truth_removed_html_string,
        truth: truth_html_string,
        truth_values,
        // hint: hint_html_string,
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
1 + 1 + 1 + 1

# explaination
2 x 2 == 2 + 2 == 4 == -2 x -2.
"
        .to_string();
        assert!(parse(md_string).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad0() {
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
}
