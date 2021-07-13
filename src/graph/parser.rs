use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::IResult;
use std::fmt::Debug;
use std::str::FromStr;

pub(crate) fn parse_num<K>(i: &str) -> nom::IResult<&str, K>
where
    K: FromStr,
    <K as FromStr>::Err: Debug,
{
    let (i, v) = digit1(i)?;
    Ok((i, v.parse().unwrap()))
}

/// 用空格分割的两个数字
/// "1 2"
pub(crate) fn parse_list_num<K>(i: &str) -> nom::IResult<&str, Vec<K>>
where
    K: FromStr,
    <K as FromStr>::Err: Debug,
{
    let sep = " ";
    separated_list1(tag(sep), parse_num)(i)
}

/// 用sep分割的字符串
pub(crate) fn parse_list_str<'a>(i: &'a str, sep: &str) -> IResult<&'a str, Vec<&'a str>> {
    separated_list1(tag(sep), is_not(sep))(i)
}

#[test]
fn t() {
    assert_eq!(parse_num("13"), Ok(("", 13)));
    assert_eq!(parse_list_num("0 5"), Ok(("", vec![0, 5])));
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
}
