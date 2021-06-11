use web_sys::{Element};
use crate::utils::{html_elem, append};

#[allow(dead_code)]
pub enum Tag {
    Div,
    H1,
    H2,
    P,
    Span,
}

impl Tag {
    pub fn node(self) -> Node {
        Node { id: None, tag: self, class_list: vec![], content: None, styles: vec![], child_nodes: vec![] }
    }

    pub fn as_html_tag(&self) -> &str {
        match self {
            Tag::Div => "div",
            Tag::H1 => "h1",
            Tag::H2 => "h2",
            Tag::P => "p",
            Tag::Span => "span",
        }
    }
}

pub struct Node {
    pub id: Option<String>,
    pub tag: Tag,
    pub class_list: Vec<String>,
    pub content: Option<String>,
    pub styles: Vec<Box<dyn Style>>,
    pub child_nodes: Vec<Node>,
}

impl Node {
    pub fn children(mut self, children: Vec<Node>) -> Self {
        self.child_nodes.extend(children);
        self
    }

    pub fn child(mut self, child: Node) -> Self {
        self.child_nodes.push(child);
        self
    }

    #[allow(dead_code)]
    pub fn class(mut self, class: &str) -> Self {
        self.class_list.push(class.to_string());
        self.styles.push(Box::new(FontSize::Px(245)));
        self
    }

    pub fn content<S: std::fmt::Display>(mut self, content: S) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn style(mut self, style: impl Style + 'static) -> Self {
        self.styles.push(Box::new(style));
        self
    }

    pub fn realize(&self, root: &Element) {
        let element = html_elem(&self.tag);
        self.class_list.iter().for_each(|class| element.set_class_name(class));
        self.content.as_ref().map(|content| element.set_inner_html(content.as_ref()));

        self.styles.iter().for_each(|style| {
            let (prop, value) = style.to_css_string();
            element.style().set_property(&prop, &value).unwrap();
        });

        append(root, &element);

        self.child_nodes.iter().for_each(|node| node.realize(&element));
    }
}

pub trait Style {
    fn to_css_string(&self) -> (String, String);
}

#[allow(dead_code)]
pub enum FontSize {
    Px(u32),
    Str(String)
}

impl Style for FontSize {
    fn to_css_string(&self) -> (String, String) {
        match self {
            FontSize::Px(px) => ("font-size".to_string(), format!("{}px", px)),
            FontSize::Str(s) => ("font-size".to_string(), s.to_string()),
        }
    }
}

#[allow(dead_code)]
pub struct Flex {
    flex_growth: i32,
    flex_shrink: i32,
    flex_width: i32
}

impl Flex {
    pub fn new(flex_growth: i32, flex_shrink: i32, flex_width: i32) -> Self {
        Flex { flex_growth, flex_shrink, flex_width }
    }
}

impl Style for Flex {
    fn to_css_string(&self) -> (String, String) {
        ("flex".to_string(), format!("{} {} {}px", self.flex_growth, self.flex_shrink, self.flex_width))
    }
}

#[allow(dead_code)]
pub enum Display {
    Flex,
    Inline,
    InlineBlock
}

impl Style for Display {
    fn to_css_string(&self) -> (String, String) {
        match self {
            Display::Flex => ("display".to_string(), "flex".to_string()),
            Display::Inline => ("display".to_string(), "inline".to_string()),
            Display::InlineBlock => ("display".to_string(), "inline-block".to_string()),
        } 
    }
}

#[allow(dead_code)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse
}

impl Style for FlexDirection {
    fn to_css_string(&self) -> (String, String) {
        match self {
            FlexDirection::Row => ("flex-direction".to_string(), "row".to_string()),
            FlexDirection::RowReverse => ("flex-direction".to_string(), "row-reverse".to_string()),
            FlexDirection::Column => ("flex-direction".to_string(), "column".to_string()),
            FlexDirection::ColumnReverse => ("flex-direction".to_string(), "column-reverse".to_string()),
        } 
    }
}

#[allow(dead_code)]
pub enum AlignItems {
    Baseline
}

impl Style for AlignItems {
    fn to_css_string(&self) -> (String, String) {
        match self {
            AlignItems::Baseline => ("align-items".to_string(), "baseline".to_string()),
        }
        
    }
}

#[allow(dead_code)]
pub enum Margin {
    Px(i32)
}

impl Style for Margin {
    fn to_css_string(&self) -> (String, String) {
        match self {
            Margin::Px(px) => ("margin".to_string(), format!("{}px", px))
        }
    }
}

#[allow(dead_code)]
pub enum MarginBottom {
    Px(i32)
}

impl Style for MarginBottom {
    fn to_css_string(&self) -> (String, String) {
        match self {
            MarginBottom::Px(px) => ("margin-bottom".to_string(), format!("{}px", px))
        }
    }
}

#[allow(dead_code)]
pub enum TextAlign {
    Center,
    Left,
    Right
}

impl Style for TextAlign {
    fn to_css_string(&self) -> (String, String) {
        match self {
            TextAlign::Center => ("text-align".to_string(), "center".to_string()),
            TextAlign::Left => ("text-align".to_string(), "left".to_string()),
            TextAlign::Right => ("text-align".to_string(), "right".to_string())
        }
    }
}

#[allow(dead_code)]
pub enum PaddingRight {
    Px(i32)
}

impl Style for PaddingRight {
    fn to_css_string(&self) -> (String, String) {
        match self {
            PaddingRight::Px(px) => ("padding-right".to_string(), format!("{}px", px))
        }
    }
}

#[allow(dead_code)]
pub enum FontFamily {
    Val(String, String)
}

impl Style for FontFamily {
    fn to_css_string(&self) -> (String, String) {
        match self {
            FontFamily::Val(name, attr) => ("font-family".to_string(), format!("'{}', {}", name, attr))
        }
    }
}

#[allow(dead_code)]
pub enum MinWidth {
    Px(u32)
}

impl Style for MinWidth {
    fn to_css_string(&self) -> (String, String) {
        match self {
            MinWidth::Px(px) => ("min-width".to_string(), format!("{}px", px))
        }
    }
}