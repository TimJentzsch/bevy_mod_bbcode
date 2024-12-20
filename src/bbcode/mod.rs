use std::{borrow::Cow, collections::HashMap, fmt::Display, sync::Arc};

pub mod parser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BbcodeNode<'a> {
    Tag(BbcodeTag<'a>),
    Text(Cow<'a, str>),
}

impl Display for BbcodeNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BbcodeNode::Tag(node) => node.fmt(f),
            BbcodeNode::Text(text) => text.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BbcodeTag<'a> {
    /// The name of the tag, e.g. `tag` for `[tag]something[/tag]`.
    name: &'a str,

    /// A simple parameter for the tag, e.g. `value` for `[tag=value]something[/tag]`.
    simple_param: Option<Cow<'a, str>>,

    /// Complex parameters, e.g. the map `value1` -> `xxx`, `value2` -> `yyy` for `[tag value1=”xxx” value2=”yyy”]something[/tag]`.
    complex_params: HashMap<&'a str, Cow<'a, str>>,

    /// The child nodes (or text) contained inside this node.
    children: Vec<Arc<BbcodeNode<'a>>>,
}

impl<'a> BbcodeTag<'a> {
    /// Create a new, empty tag.
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            simple_param: None,
            complex_params: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Add a simple parameter to the tag.
    #[cfg(test)]
    pub fn with_simple_param<P: Into<Cow<'a, str>>>(mut self, tag_param: P) -> Self {
        self.simple_param = Some(tag_param.into());
        self
    }

    /// Add a simple parameter to the tag.
    pub fn add_simple_param<P: Into<Cow<'a, str>>>(&mut self, tag_param: P) -> &mut Self {
        self.simple_param = Some(tag_param.into());
        self
    }

    /// Add a key/value parameter.
    #[cfg(test)]
    pub fn with_param<K: Into<&'a str>, V: Into<Cow<'a, str>>>(mut self, key: K, value: V) -> Self {
        self.complex_params.insert(key.into(), value.into());
        self
    }

    /// Add a nested tag inside this one.
    #[cfg(test)]
    pub fn with_tag(mut self, tag: BbcodeTag<'a>) -> Self {
        self.children.push(Arc::new(BbcodeNode::Tag(tag)));
        self
    }

    /// Add text inside of the node.
    #[cfg(test)]
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Self {
        self.children
            .push(Arc::new(BbcodeNode::Text(Cow::Owned(text.into()))));
        self
    }

    /// The name of this tag.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The child nodes of this tag.
    pub fn children(&self) -> &Vec<Arc<BbcodeNode<'a>>> {
        &self.children
    }

    /// If it exists, the simple tag parameter of this tag.
    pub fn simple_param(&self) -> &Option<Cow<'a, str>> {
        &self.simple_param
    }
}

impl Display for BbcodeTag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_param(param: &str) -> String {
            if param.contains(' ') {
                format!("\"{param}\"")
            } else {
                param.to_owned()
            }
        }

        write!(f, "[{}", self.name)?;

        if let Some(tag_param) = &self.simple_param {
            write!(f, "={}", fmt_param(tag_param))?;
        }

        for (key, value) in &self.complex_params {
            write!(f, " {key}={}", fmt_param(value))?;
        }

        write!(f, "]")?;

        for child in &self.children {
            child.fmt(f)?;
        }

        write!(f, "[/{}]", self.name)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_bold() {
        let node = BbcodeTag::new("b").with_text("test");
        assert_eq!(format!("{node}"), "[b]test[/b]");
    }

    #[test]
    fn display_color() {
        let node = BbcodeTag::new("color")
            .with_simple_param("#FF00FF")
            .with_text("test");
        assert_eq!(format!("{node}"), "[color=#FF00FF]test[/color]");
    }

    #[test]
    fn display_image() {
        let node = BbcodeTag::new("img")
            .with_param("alt", "example image")
            .with_text("test");
        assert_eq!(format!("{node}"), r#"[img alt="example image"]test[/img]"#);
    }
}
