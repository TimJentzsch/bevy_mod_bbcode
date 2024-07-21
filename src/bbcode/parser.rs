use std::sync::Arc;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};

use super::{BbcodeNode, BbcodeTag};

pub fn parse_bbcode(input: &str) -> IResult<&str, Vec<Arc<BbcodeNode>>> {
    many0(map(parse_node, |element| element.into()))(input)
}

fn parse_node(input: &str) -> IResult<&str, BbcodeNode> {
    alt((
        map(parse_text, |text| BbcodeNode::Text(text.into())),
        map(parse_tag, BbcodeNode::Tag),
    ))(input)
}

fn parse_tag(input: &str) -> IResult<&str, BbcodeTag> {
    let (input, mut tag) = parse_opening_tag(input)?;
    let (input, children) = parse_bbcode(input)?;
    let (input, _) = parse_closing_tag(input, &tag.name)?;

    tag.children = children;

    Ok((input, tag))
}

fn parse_opening_tag(input: &str) -> IResult<&str, BbcodeTag> {
    map(delimited(char('['), alpha1, char(']')), |tag| {
        BbcodeTag::new(tag)
    })(input)
}

fn parse_closing_tag<'a>(input: &'a str, tag_name: &str) -> IResult<&'a str, ()> {
    map(
        delimited(tag("[/"), tag_no_case(tag_name), char(']')),
        |_| (),
    )(input)
}

fn parse_text(input: &str) -> IResult<&str, &str> {
    take_while1(|ch| !['[', ']'].contains(&ch))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let input = "[b]test[/b]";
        let expected_tag = BbcodeTag::new("b").with_text("test");

        assert_eq!(
            parse_bbcode(input),
            Ok(("", vec![BbcodeNode::Tag(expected_tag).into()]))
        )
    }

    #[test]
    fn test_parse_nested() {
        let input = "[b]test [i]nested[/i][/b]";
        let expected_tag = BbcodeTag::new("b")
            .with_text("test ")
            .with_tag(BbcodeTag::new("i").with_text("nested"));

        assert_eq!(
            parse_bbcode(input),
            Ok(("", vec![BbcodeNode::Tag(expected_tag).into()]))
        )
    }
}
