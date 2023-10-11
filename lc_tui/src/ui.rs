use html2text::{parse, render::text_renderer::TextDecorator};
use ratatui::{prelude::*, style::Style};

pub struct RatDecorator {}

impl TextDecorator for RatDecorator {
    // annotation may be
    type Annotation = ratatui::style::Style;
    fn finalise(
        &mut self,
        links: Vec<String>,
    ) -> Vec<html2text::render::text_renderer::TaggedLine<Self::Annotation>> {
        Vec::new()
    }
    fn quote_prefix(&mut self) -> String {
        "> ".to_string()
    }
    fn header_prefix(&mut self, level: usize) -> String {
        "#".repeat(level) + " "
    }
    fn decorate_image(&mut self, src: &str, title: &str) -> (String, Self::Annotation) {
        // (title.to_string(), RichAnnotation::Image(src.to_string()))
        (title.to_string(), Style::default())
    }
    fn decorate_em_end(&mut self) -> String {
        "".to_string()
    }
    fn decorate_link_end(&mut self) -> String {
        "".to_string()
    }
    fn decorate_em_start(&mut self) -> (String, Self::Annotation) {
        (
            "".to_string(),
            Style::default().add_modifier(Modifier::ITALIC),
        )
    }
    fn decorate_code_end(&mut self) -> String {
        "".to_string()
    }
    fn decorate_link_start(&mut self, url: &str) -> (String, Self::Annotation) {
        ("".to_string(), Style::default())
    }
    fn decorate_strong_end(&mut self) -> String {
        "".to_string()
    }
    fn decorate_code_start(&mut self) -> (String, Self::Annotation) {
        ("".to_string(), Style::default().bg(Color::DarkGray))
    }
    fn ordered_item_prefix(&mut self, i: i64) -> String {
        format!("{}. ", i)
    }
    fn decorate_strong_start(&mut self) -> (String, Self::Annotation) {
        (
            "".to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        )
    }
    fn unordered_item_prefix(&mut self) -> String {
        "    \u{2022} ".to_string()
    }
    fn decorate_strikeout_end(&mut self) -> String {
        "".to_string()
    }
    fn decorate_preformat_cont(&mut self) -> Self::Annotation {
        Style::default()
    }
    fn make_subblock_decorator(&self) -> Self {
        RatDecorator {}
    }
    fn decorate_strikeout_start(&mut self) -> (String, Self::Annotation) {
        (
            "".to_string(),
            Style::default().add_modifier(Modifier::CROSSED_OUT),
        )
    }
    fn decorate_preformat_first(&mut self) -> Self::Annotation {
        Style::default()
    }
}

pub fn sanitize_html(contents: String) -> Text<'static> {
    // must first replace <sup> since the library that I am using does not
    let decorator = RatDecorator {};
    let out = parse(contents.as_bytes()).render(usize::MAX, decorator);
    let mut text = Vec::new();
    for line in out.into_lines() {
        let mut rat_line = Line::default();
        for span in line.tagged_strings() {
            let mut style = Style::default();
            for tag in span.tag.iter() {
                style = style.set_style(*tag);
            }
            rat_line.spans.push(Span::styled(span.s.clone(), style));
        }
        text.push(rat_line);
    }

    Text::from(text)
}