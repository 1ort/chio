use crate::ast::{ChangeLog, Task, TaskGroup, TaskId, VersionGroup};
use core::str;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_until1, take_while},
    character::complete::{char, u32},
    combinator::rest,
    error::{Error, ParseError},
    multi::{many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use nom_locate::{position, LocatedSpan};
type Span<'a> = LocatedSpan<&'a str>;

fn take_until_newline(input: &str) -> IResult<&str, &str> {
    terminated(take_until1("\n"), tag("\n"))(input)
}

fn take_until_newline_or_eof(input: &str) -> IResult<&str, &str> {
    alt((take_until_newline, rest))(input)
}

fn uppercase_char<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_task_id(input: &str) -> IResult<&str, TaskId> {
    let (remaining, (project, number)) = separated_pair(uppercase_char, char('-'), u32)(input)?;
    IResult::Ok((
        remaining,
        TaskId {
            project: String::from(project),
            number,
        },
    ))
}

fn parse_task_sub_list(input: &str) -> IResult<&str, Vec<String>> {
    let parse_sub_list_row = preceded(tag("  - "), take_until_newline_or_eof);

    let mut parse_sub_list = many1(parse_sub_list_row);

    let (remaining, sub_list) = parse_sub_list(input)?;
    let sub_list = sub_list.iter().map(|&s| s.into()).collect();
    IResult::Ok((remaining, sub_list))
}

fn parse_task_group_entry(input: &str) -> IResult<&str, Task> {
    let mut parse_task_link = delimited(char('['), parse_task_id, char(']'));
    let mut parse_task_link = preceded(tag("- "), parse_task_link);
    let mut parse_task_link = terminated(parse_task_link, char(' '));
    let (remaining, task_id) = parse_task_link(input)?;

    let (remaining, task_body) = take_until_newline_or_eof(remaining)?;

    let sub_group_parse_attempt_result = parse_task_sub_list(remaining);
    let (remaining, sub_list) = match sub_group_parse_attempt_result {
        IResult::Ok((rem, sub_group)) => (rem, Some(sub_group)),
        IResult::Err(_) => (remaining, None),
    };

    IResult::Ok((
        remaining,
        Task {
            id: task_id,
            description: String::from(task_body),
            sub_list: sub_list,
        },
    ))
}

fn parse_task_group(input: &str) -> IResult<&str, TaskGroup> {
    // ### header
    let mut parse_task_group_header = preceded(tag("### "), take_until_newline);
    let (remaining, header) = parse_task_group_header(input)?;
    // task group body until next ### tag
    let mut parse_task_group_body = terminated(take_until1("\n###"), tag("\n"));
    let mut parse_task_group_body = alt((parse_task_group_body, rest));
    let (remaining, task_group_body) = parse_task_group_body(remaining)?;
    let mut parse_task_entries = many1(parse_task_group_entry);
    let (task_group_remaining, task_group_entries) = parse_task_entries(task_group_body)?;
    IResult::Ok((
        remaining,
        TaskGroup {
            header: String::from(header),
            entries: task_group_entries,
        },
    ))
}

fn parse_version_group(input: &str) -> IResult<&str, VersionGroup> {
    // ## versiion
    let mut parse_version = preceded(tag("## "), take_until_newline);
    let (remaining, version_str) = parse_version(input)?;
    // version log body: until empty string
    let mut parse_version_group_body = terminated(take_until1("\n\n"), tag("\n"));
    let (remaining, version_group_body) = parse_version_group_body(remaining)?;
    let (body_remaining, task_groups) = many1(parse_task_group)(version_group_body)?;
    IResult::Ok((
        remaining,
        VersionGroup {
            version: String::from(version_str),
            task_groups: task_groups,
        },
    ))
}

pub fn parse_changelog(input: &str) -> IResult<&str, ChangeLog> {
    // # Header
    let mut parse_header = preceded(tag("# "), take_until_newline);
    let (remaining, header) = parse_header(input)?;

    // single empty line
    let (remaining, _) = tag("\n")(remaining)?;

    // list of versions separated by double empty lines
    let mut parse_versions = separated_list0(tag("\n\n"), parse_version_group);
    let (remaining, version_groups) = parse_versions(remaining)?;

    IResult::Ok((
        remaining,
        ChangeLog {
            header: String::from(header),
            versions: version_groups,
        },
    ))
}
