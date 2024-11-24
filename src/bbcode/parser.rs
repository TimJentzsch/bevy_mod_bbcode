use std::{borrow::Cow, sync::Arc};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::{alpha1, char},
    combinator::{map, opt, value, verify},
    error::ParseError,
    multi::{fold_many1, many0},
    sequence::{delimited, preceded},
    IResult, Parser,
};

use super::{BbcodeNode, BbcodeTag};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

pub fn parse_bbcode(input: &str) -> IResult<&str, Vec<Arc<BbcodeNode>>> {
    parse_bbcode_internal(input)
}

fn parse_bbcode_internal<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Vec<Arc<BbcodeNode>>, E> {
    many0(map(parse_node, |element| element.into()))(input)
}

fn parse_node<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BbcodeNode<'a>, E> {
    alt((
        map(parse_text, BbcodeNode::Text),
        map(parse_tag, BbcodeNode::Tag),
    ))(input)
}

fn parse_tag<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BbcodeTag<'a>, E> {
    let (input, mut tag) = parse_opening_tag(input)?;
    let (input, children) = parse_bbcode_internal(input)?;
    let (input, _) = parse_closing_tag(input, tag.name)?;

    tag.children = children;

    Ok((input, tag))
}

fn parse_opening_tag<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, BbcodeTag, E> {
    let (mut input, mut tag) = map(preceded(char('['), alpha1), BbcodeTag::new)(input)?;

    if let Ok((new_input, simple_param)) = preceded(char('='), parse_param::<E>)(input) {
        tag.add_simple_param(simple_param);
        input = new_input;
    }

    let (input, _) = char(']')(input)?;

    Ok((input, tag))
}

fn parse_closing_tag<'a, E: ParseError<&'a str>>(
    input: &'a str,
    tag_name: &str,
) -> IResult<&'a str, (), E> {
    map(
        delimited(tag("[/"), tag_no_case(tag_name), char(']')),
        |_| (),
    )(input)
}

fn parse_text<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Cow<'a, str>, E> {
    parse_inner_string("[]\\").parse(input)
}

fn parse_param<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Cow<'a, str>, E> {
    alt((
        parse_quoted_string,
        map(parse_literal("\"\\[]"), Cow::Borrowed),
    ))
    .parse(input)
}

fn parse_quoted_string<'a, E: ParseError<&'a str>>(
    input: &'a str,
) -> IResult<&'a str, Cow<'a, str>, E> {
    delimited(
        char('"'),
        map(opt(parse_inner_string("\"\\")), |string| {
            string.unwrap_or_default()
        }),
        char('"'),
    )
    .parse(input)
}

fn parse_inner_string<'a, E: ParseError<&'a str>>(
    exclude: &'a str,
) -> impl Parser<&'a str, Cow<'a, str>, E> {
    move |input| {
        fold_many1(
            parse_fragment(exclude),
            Cow::<'a, str>::default,
            |mut cow, fragment| {
                match fragment {
                    StringFragment::Literal(s) => {
                        if cow.is_empty() {
                            cow = Cow::Borrowed(s);
                        } else {
                            cow.to_mut().push_str(s);
                        }
                    }
                    StringFragment::EscapedChar(c) => {
                        cow.to_mut().push(c);
                    }
                }
                cow
            },
        )
        .parse(input)
    }
}

fn parse_fragment<'a, E: ParseError<&'a str>>(
    exclude: &'a str,
) -> impl Parser<&'a str, StringFragment<'a>, E> {
    move |input| {
        alt((
            map(parse_literal(exclude), StringFragment::Literal),
            map(parse_escaped_char, StringFragment::EscapedChar),
        ))
        .parse(input)
    }
}

fn parse_literal<'a, E: ParseError<&'a str>>(exclude: &'a str) -> impl Parser<&'a str, &'a str, E> {
    move |input| verify(is_not(exclude), |s: &str| !s.is_empty()).parse(input)
}

fn parse_escaped_char<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, char, E> {
    preceded(
        char('\\'),
        alt((
            value('"', char('"')),
            value('/', char('/')),
            value('[', char('[')),
            value(']', char(']')),
            value('\\', char('\\')),
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
        )),
    )
    .parse(input)
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
    fn test_parse_escaped_text() {
        let input = r#"[b]\[\]\\\"\t\n[/b]"#;
        let expected_tag = BbcodeTag::new("b").with_text("[]\\\"\t\n");

        assert_eq!(
            parse_bbcode(input),
            Ok(("", vec![BbcodeNode::Tag(expected_tag).into()]))
        )
    }

    #[test]
    fn test_parse_simple_param() {
        let input = "[c=#ff00ff]test[/c]";
        let expected_tag = BbcodeTag::new("c")
            .with_simple_param("#ff00ff")
            .with_text("test");

        assert_eq!(
            parse_bbcode(input),
            Ok(("", vec![BbcodeNode::Tag(expected_tag).into()]))
        )
    }

    #[test]
    fn test_parse_quoted_param() {
        let input = r#"[c="dark \"blue\" with yellow"]test[/c]"#;
        let expected_tag = BbcodeTag::new("c")
            .with_simple_param(r#"dark "blue" with yellow"#)
            .with_text("test");

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
