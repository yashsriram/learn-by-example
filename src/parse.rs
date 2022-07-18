use crate::MultiChoiceMultiCorrectContext;
use pulldown_cmark::{html, CowStr, Event, Options, Parser};

pub fn parse(md_string: String) -> Result<MultiChoiceMultiCorrectContext, String> {
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
    Ok(MultiChoiceMultiCorrectContext {
        truth_removed_html,
        truth_html,
        truth_values,
    })
}
