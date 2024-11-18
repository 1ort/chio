use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, rest},
    sequence::preceded,
    IResult,
};
use nom_locate::LocatedSpan;

use crate::ast::ChangeLog;
const LINE_BREAK: char = '\n';

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
enum FSMState {
    Begin,
    Header(String),
    EmptyLines(usize),
    VersionHeader(String),
    GroupHeader(String),
    Task(String),
    SubListEntry(String),
    End,
}

type ErrorType = String;

fn parse_header(input: Span) -> IResult<Span, FSMState> {
    let mut parser = preceded(tag("# "), rest);
    let (rem, res) = parser(input)?;
    IResult::Ok((rem, FSMState::Header(res.to_string())))
}

fn parse_empty_line(input: Span) -> IResult<Span, FSMState> {
    let parser = eof;
    let (rem, _) = parser(input)?;
    IResult::Ok((rem, FSMState::EmptyLines(1)))
}

fn parse_version_header(input: Span) -> IResult<Span, FSMState> {
    let mut parser = preceded(tag("## "), rest);
    let (rem, res) = parser(input)?;
    IResult::Ok((rem, FSMState::VersionHeader(res.to_string())))
}

fn parse_group_header(input: Span) -> IResult<Span, FSMState> {
    let mut parser = preceded(tag("### "), rest);
    let (rem, res) = parser(input)?;
    IResult::Ok((rem, FSMState::GroupHeader(res.to_string())))
}

fn parse_task(input: Span) -> IResult<Span, FSMState> {
    let mut parser = preceded(tag("- "), rest);
    let (rem, res) = parser(input)?;
    IResult::Ok((rem, FSMState::Task(res.to_string())))
}

fn parse_sub_list_entry(input: Span) -> IResult<Span, FSMState> {
    let mut parser = preceded(tag("  - "), rest);
    let (rem, res) = parser(input)?;
    IResult::Ok((rem, FSMState::SubListEntry(res.to_string())))
}

fn parse_line(line: &str) -> IResult<Span, FSMState> {
    let input = Span::from(line);
    let mut parser = alt((
        parse_header,
        parse_empty_line,
        parse_version_header,
        parse_group_header,
        parse_task,
        parse_sub_list_entry,
    ));
    parser(input)
}

impl FSMState {
    fn process_line(
        &self,
        line: &str,
    ) -> (Self, Vec<ErrorType>) {
        match self {
            Self::Begin => self.process_header(line),
            Self::Header(_) => self.process_header_empty_line(line),
            _ => (Self::End, vec![]),
        }
    }

    fn process_header(
        &self,
        line: &str,
    ) -> (Self, Vec<ErrorType>) {
        match parse_line(line) {
            IResult::Ok((_, next_state)) => {
                match next_state {
                    FSMState::Header(header) => (FSMState::Header(header), vec![]),
                    _ => {
                        (
                            next_state,
                            vec![String::from("Expected # Changelog header")],
                        )
                    },
                }
            },
            IResult::Err(_) => {
                println!("[Error] cant recognize line!, {}", line);
                (
                    Self::Begin,
                    vec![String::from("Expected # Changelog header")],
                )
            },
        }
    }

    fn process_header_empty_line(
        &self,
        line: &str,
    ) -> (Self, Vec<ErrorType>) {
        let lines_num = {
            match self {
                Self::EmptyLines(num) => num,
                _ => &0,
            }
        };
        match parse_line(line) {
            IResult::Ok((_, next_state)) => {
                match next_state {
                    FSMState::EmptyLines(n) => (FSMState::EmptyLines(lines_num + n), vec![]),
                    _ => {
                        (
                            next_state,
                            vec![String::from("Expected # Changelog header")],
                        )
                    },
                }
            },
            IResult::Err(_) => {
                println!("[Error] cant recognize line!, {}", line);
                (
                    self,
                    vec![String::from("Expected empty line before last version")],
                )
            },
        }
    }
}

pub fn parse_changelog(input: &str) -> ChangeLog {
    let splitted_str = input.split(LINE_BREAK);

    let mut current_state = FSMState::Begin;

    for line in splitted_str {
        let (next_state, errors) = current_state.process_line(line);
        match state {}
    }
}
