use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{digit1, space0};
use nom::multi::separated_list1;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_num<K>(i: &str) -> nom::IResult<&str, K>
where
    K: FromStr,
    <K as FromStr>::Err: Debug,
{
    let (i, (_, v)) = tuple((space0, digit1))(i)?;
    Ok((i, v.parse().unwrap()))
}

pub fn parse_float(i: &str) -> nom::IResult<&str, f32> {
    let (i, (_, v)) = tuple((space0, float))(i)?;
    Ok((i, v))
}

/// 用空格分割的两个数字
/// "1 2"
pub fn parse_list_num<K>(i: &str) -> nom::IResult<&str, Vec<K>>
where
    K: FromStr,
    <K as FromStr>::Err: Debug,
{
    let sep = " ";
    separated_list1(tag(sep), parse_num)(i)
}

pub fn parse_list_float(i: &str) -> nom::IResult<&str, Vec<f32>> {
    let sep = " ";
    separated_list1(tag(sep), parse_float)(i)
}

/// 用sep分割的字符串
pub fn parse_list_str<'a>(i: &'a str, sep: &str) -> IResult<&'a str, Vec<&'a str>> {
    separated_list1(tag(sep), is_not(sep))(i)
}

#[test]
fn t() {
    assert_eq!(parse_num("13"), Ok(("", 13)));
    // leading space
    assert_eq!(parse_num("  13"), Ok(("", 13)));
    assert_eq!(parse_list_num("0 5"), Ok(("", vec![0, 5])));
    // leading space
    assert_eq!(parse_list_num(" 0 5"), Ok(("", vec![0, 5])));
    assert_eq!(parse_list_str("0 5", " "), Ok(("", vec!["0", "5"])));
    assert_eq!(parse_list_str("LAS PHX", " "), Ok(("", vec!["LAS", "PHX"])));
    assert_eq!(
        parse_list_str("aaa/bbb/ccc", "/"),
        Ok(("", vec!["aaa", "bbb", "ccc"]))
    );
    assert_eq!(
        parse_list_str("aaa, (1991)/bbb, 111/ccc (C)", "/"),
        Ok(("", vec!["aaa, (1991)", "bbb, 111", "ccc (C)"]))
    );
    assert_eq!(parse_list_float("4 5 0.35"), Ok(("", vec![4.0, 5.0, 0.35])));
}
