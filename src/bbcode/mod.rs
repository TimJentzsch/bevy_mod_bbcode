use std::{collections::HashMap, fmt::Display, sync::Arc};

pub enum BbcodeNode {
    Tag(BbcodeTag),
    Text(String),
}

impl Display for BbcodeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BbcodeNode::Tag(node) => node.fmt(f),
            BbcodeNode::Text(text) => text.fmt(f),
        }
    }
}

pub struct BbcodeTag {
    /// The name of the tag, e.g. `tag` for `[tag]something[/tag]`.
    name: String,

    /// A simple parameter for the tag, e.g. `value` for `[tag=value]something[/tag]`.
    simple_param: Option<String>,

    /// Complex parameters, e.g. the map `value1` -> `xxx`, `value2` -> `yyy` for `[tag value1=”xxx” value2=”yyy”]something[/tag]`.
    complex_params: HashMap<String, String>,

    /// The child nodes (or text) contained inside this node.
    children: Vec<Arc<BbcodeNode>>,
}

impl BbcodeTag {
    /// Create a new, empty tag.
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            simple_param: None,
            complex_params: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Add a simple parameter to the tag.
    pub fn with_simple_param<P: Into<String>>(mut self, tag_param: P) -> Self {
        self.simple_param = Some(tag_param.into());
        self
    }

    /// Add a key/value parameter.
    pub fn with_param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.complex_params.insert(key.into(), value.into());
        self
    }

    /// Add a nested tag inside this one.
    pub fn with_tag(mut self, tag: BbcodeTag) -> Self {
        self.children.push(Arc::new(BbcodeNode::Tag(tag)));
        self
    }

    /// Add text inside of the node.
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Self {
        self.children.push(Arc::new(BbcodeNode::Text(text.into())));
        self
    }
}

impl Display for BbcodeTag {
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
