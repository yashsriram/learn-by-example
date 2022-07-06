use pulldown_cmark::{html, CowStr, Event, HeadingLevel, Options, Parser, Tag};
use serde::Serialize;

#[derive(Serialize)]
pub struct MultiChoiceMultiCorrectQuestionHtml {
    id: String,
    prompt: String,
    options: String,
    hint: String,
    truth: String,
    explaination: String,
    truth_values: Vec<bool>,
}

pub fn parse(md_str: String, id: String) -> Result<MultiChoiceMultiCorrectQuestionHtml, String> {
    // parse
    let mut md_parse_options = Options::empty();
    md_parse_options.insert(Options::ENABLE_TABLES);
    md_parse_options.insert(Options::ENABLE_FOOTNOTES);
    md_parse_options.insert(Options::ENABLE_STRIKETHROUGH);
    md_parse_options.insert(Options::ENABLE_TASKLISTS);
    md_parse_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    md_parse_options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let parser = Parser::new_ext(&md_str, md_parse_options);
    let events: Vec<_> = parser.collect();
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
    let prompt_html_str = {
        let mut html_str = String::new();
        html::push_html(
            &mut html_str,
            events[prompt_range].iter().map(|event| event.clone()),
        );
        html_str
    };
    println!("{:?}", &events[truth_range.clone()]);
    let options_html_str = {
        let mut html_str = String::new();
        html::push_html(
            &mut html_str,
            events[truth_range.clone()].iter().map(|event| match event {
                Event::TaskListMarker(..) => Event::TaskListMarker(false).clone(),
                _ => event.clone(),
            }),
        );
        html_str
    };
    let hint_html_str = {
        let mut html_str = String::new();
        html::push_html(
            &mut html_str,
            events[hint_range].iter().map(|event| event.clone()),
        );
        html_str
    };
    let truth_html_str = {
        let mut html_str = String::new();
        html::push_html(
            &mut html_str,
            events[truth_range.clone()].iter().map(|event| match event {
                Event::Text(CowStr::Borrowed("options")) => Event::Text(CowStr::Borrowed("truth")),
                _ => event.clone(),
            }),
        );
        html_str
    };
    let explaination_html_str = {
        let mut html_str = String::new();
        html::push_html(
            &mut html_str,
            events[explaination_range].iter().map(|event| event.clone()),
        );
        html_str
    };
    let truth_values = events[truth_range.clone()]
        .iter()
        .filter_map(|event| match event {
            Event::TaskListMarker(val) => Some(*val),
            _ => None,
        })
        .collect();
    Ok(MultiChoiceMultiCorrectQuestionHtml {
        id,
        prompt: prompt_html_str,
        options: options_html_str,
        hint: hint_html_str,
        truth: truth_html_str,
        explaination: explaination_html_str,
        truth_values,
    })
}
