use std::{collections::HashMap, fmt::Display, sync::Arc};

pub enum BbcodeElement {
    Node(BbcodeNode),
    Text(String),
}

impl Display for BbcodeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BbcodeElement::Node(node) => node.fmt(f),
            BbcodeElement::Text(text) => text.fmt(f),
        }
    }
}

pub struct BbcodeNode {
    /// The name of the tag, e.g. `tag` for `[tag]something[/tag]`.
    tag: String,

    /// A simple parameter for the tag, e.g. `value` for `[tag=value]something[/tag]`.
    tag_param: Option<String>,

    /// Complex parameters, e.g. the map `value1` -> `xxx`, `value2` -> `yyy` for `[tag value1=”xxx” value2=”yyy”]something[/tag]`.
    params: HashMap<String, String>,

    /// The child nodes (or text) contained inside this node.
    children: Vec<Arc<BbcodeElement>>,
}

impl BbcodeNode {
    /// Create a new, empty node.
    pub fn new<S: Into<String>>(tag: S) -> Self {
        Self {
            tag: tag.into(),
            tag_param: None,
            params: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Add a tag parameter.
    pub fn with_tag_param<P: Into<String>>(mut self, tag_param: P) -> Self {
        self.tag_param = Some(tag_param.into());
        self
    }

    /// Add a key/value parameter.
    pub fn with_param<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Add a nested node inside this one.
    pub fn with_nested_node(mut self, node: BbcodeNode) -> Self {
        self.children.push(Arc::new(BbcodeElement::Node(node)));
        self
    }

    /// Add text inside of the node.
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Self {
        self.children
            .push(Arc::new(BbcodeElement::Text(text.into())));
        self
    }
}

impl Display for BbcodeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_param(param: &str) -> String {
            if param.contains(' ') {
                format!("\"{param}\"")
            } else {
                param.to_owned()
            }
        }

        write!(f, "[{}", self.tag)?;

        if let Some(tag_param) = &self.tag_param {
            write!(f, "={}", fmt_param(tag_param))?;
        }

        for (key, value) in &self.params {
            write!(f, " {key}={}", fmt_param(value))?;
        }

        write!(f, "]")?;

        for child in &self.children {
            child.fmt(f)?;
        }

        write!(f, "[/{}]", self.tag)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_bold() {
        let node = BbcodeNode::new("b").with_text("test");
        assert_eq!(format!("{node}"), "[b]test[/b]");
    }

    #[test]
    fn display_color() {
        let node = BbcodeNode::new("color")
            .with_tag_param("#FF00FF")
            .with_text("test");
        assert_eq!(format!("{node}"), "[color=#FF00FF]test[/color]");
    }

    #[test]
    fn display_image() {
        let node = BbcodeNode::new("img")
            .with_param("alt", "example image")
            .with_text("test");
        assert_eq!(format!("{node}"), r#"[img alt="example image"]test[/img]"#);
    }
}
