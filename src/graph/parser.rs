use nom::character::complete::digit1;
use nom::multi::separated_list1;
use std::fmt::Debug;
use std::str::FromStr;

fn parse_num<K>(i: &str) -> nom::IResult<&str, K>
where
    K: FromStr,
    <K as FromStr>::Err: Debug,
{
    let (i, v) = digit1(i)?;
    Ok((i, v.parse().unwrap()))
}

pub(crate) fn parse_vertices(i: &str) -> nom::IResult<&str, usize> {
    parse_num(i)
}

pub(crate) fn parse_edges(i: &str) -> nom::IResult<&str, usize> {
    parse_num(i)
}

pub(crate) fn parse_link(i: &str) -> nom::IResult<&str, Vec<usize>> {
    separated_list1(nom::character::complete::char(' '), parse_num)(i)
}

#[test]
fn t() {
    assert_eq!(parse_vertices("13"), Ok(("", 13)));
    assert_eq!(parse_edges("13"), Ok(("", 13)));
    assert_eq!(parse_link("0 5"), Ok(("", vec![0, 5])));
}
