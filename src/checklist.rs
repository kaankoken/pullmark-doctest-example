use std::collections::HashSet;

use anyhow::Result;
use lazy_static::lazy_static;
use pulldown_cmark::{Event, HeadingLevel, Tag};
use regex::Regex;

lazy_static! {
    pub static ref CHECKLIST_ID: Regex = Regex::new(r"id:\d+").unwrap();
    pub static ref CHECKLIST: Regex = Regex::new(r"(?m)^\s*-\s*\[\s*[xX]\s*]\s*(.*)").unwrap();
}

/// Detects a checklist in a markdown document and returns it as a string.
/// ```rust
/// use pulldown_cmark::HeadingLevel;
/// use std::collections::HashSet;
/// use pullmark_doctest_example::checklist::*;
///
/// let markdown_content = r#"
/// # My Document
///
/// ## Checklist Section
///
/// - [x] Item 1 <!--id:1-->
/// - [x] Item 2 <!--id:2-->
/// - [ ] Item 3 <!--id:3-->
///
/// ## Other Section
/// "#;
///
/// let must_verified = vec!["id:1", "id:2"]
///    .into_iter()
///    .map(String::from)
///    .collect::<HashSet<String>>();
/// let start_heading = (HeadingLevel::H2, "Checklist Section".to_string());
/// let end_heading = Some((HeadingLevel::H2, "Other Section".to_string()));
/// let result = checklist_check(
///   markdown_content.to_string(),
///   must_verified,
///   start_heading,
///   end_heading,
///   true,
///   );
///
///   eprintln!("{:?}", result);
///   assert!(result.is_err());
///   // I intentionally failed the test by changing the is_ok to is_err
///   // This way we can see the each events.
/// ```
pub fn checklist_check(
    raw_text: String,
    must_verified: HashSet<String>,
    start_heading: (HeadingLevel, String),
    end_heading: Option<(HeadingLevel, String)>,
    contains_id: bool,
) -> Result<()> {
    let checklist = grab_checklist(raw_text, start_heading, end_heading, contains_id);

    if checklist.is_empty() {
        return Err(anyhow::anyhow!("Checklist is empty"));
    }

    let items: HashSet<String> = CHECKLIST
        .captures_iter(&checklist)
        .filter_map(|capture| capture.get(1).map(|m| m.as_str().to_owned()))
        .filter_map(|item| {
            if contains_id {
                CHECKLIST_ID.find(&item).map(|m| m.as_str().to_owned())
            } else {
                Some(item.replace(['[', ']', 'x', '-'], "").trim().to_owned())
            }
        })
        .collect();

    let not_verified: Vec<String> = must_verified
        .difference(&items)
        .map(|item| item.to_owned())
        .collect();

    if !not_verified.is_empty() {
        return Err(anyhow::anyhow!(
            "Checklist is missing the following items: {:?}",
            not_verified
        ));
    }

    Ok(())
}

fn grab_checklist(
    raw_text: String,
    start: (HeadingLevel, String),
    end: Option<(HeadingLevel, String)>,
    contains_id: bool,
) -> String {
    let parser = pulldown_cmark::Parser::new(&raw_text);

    let mut checklist = String::new();
    let mut in_checklist = false;
    let mut current_item = String::new();

    for event in parser {
        println!("{:?}", event);
        match event {
            Event::Start(Tag::Heading { level, .. }) if in_checklist => {
                if let Some((end_level, _)) = &end {
                    if level == *end_level {
                        break;
                    }
                }
            }
            Event::Text(text) if !in_checklist => {
                if start.1 == text.to_string() {
                    in_checklist = true;
                    continue;
                }
            }
            Event::Text(text) if in_checklist => current_item.push_str(&text),
            Event::End { .. } if in_checklist => {
                checklist.push_str(&current_item);
                checklist.push('\n');
                current_item.clear();
            }
            Event::Start(Tag::Item) if in_checklist => {
                current_item.clear();
                current_item.push_str("- ");
            }
            Event::TaskListMarker(checked) if in_checklist => {
                current_item.push_str(if checked { "[x] " } else { "[ ] " });
            }
            Event::Code(text) if in_checklist => {
                current_item.push('`');
                current_item.push_str(&text);
                current_item.push('`');
            }
            Event::InlineHtml(html) if in_checklist && contains_id => {
                current_item.push_str(&html);
            }
            Event::SoftBreak | Event::HardBreak if in_checklist => current_item.push(' '),
            _ => {}
        }
    }

    checklist.trim().to_owned()
}

#[test]
fn test_checklist_check() {
    let markdown_content = r#"
# My Document

## Checklist Section

- [x] Item 1 <!--id:1-->
- [x] Item 2 <!--id:2-->
- [ ] Item 3 <!--id:3-->

## Other Section
"#;

    let must_verified = vec!["id:1", "id:2"]
        .into_iter()
        .map(String::from)
        .collect::<HashSet<String>>();
    let start_heading = (HeadingLevel::H2, "Checklist Section".to_string());
    let end_heading = Some((HeadingLevel::H2, "Other Section".to_string()));

    let result = checklist_check(
        markdown_content.to_string(),
        must_verified,
        start_heading,
        end_heading,
        true,
    );

    assert!(result.is_err());

    // I intentionally failed the test by changing the is_ok to is_err
    // This way we can see the each events.
}
