use nom::{IResult, digit, multispace, space, line_ending, not_line_ending};
use nom::IResult::*;
use nom::Err::*;

use std::str;
use std::str::FromStr;

type EmailAddress<'a> = &'a str;
type ServerAddress<'a>  = &'a str;

#[derive(Debug,PartialEq)]
enum Command<'a> {
    Helo(ServerAddress<'a>), // contains address of connecting server
    Mail(EmailAddress<'a>),  // contains from address
    Rcpt(EmailAddress<'a>),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
}

named!(data, tag!("DATA"));

named!(helo <&[u8], Command>,
    chain!(
       cmd: tag!("HELO")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       line_ending          ,
       || { Command::Helo(rest) }
   )
);

named!(mail <&[u8], Command>,
    chain!(
       cmd: tag!("MAIL")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       line_ending          ,
       || { Command::Mail(rest) }
   )
);

named!(rcpt <&[u8], Command>,
    chain!(
       cmd: tag!("RCPT")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       line_ending          ,
       || { Command::Rcpt(rest) }
   )
);

#[test]
fn test_parser() {
    assert_eq!(mail(&b"MAIL gnu\n"[..]), IResult::Done(&b""[..], Command::Mail("gnu")));
    assert_eq!(helo(&b"HELO gnu\n"[..]), IResult::Done(&b""[..], Command::Helo("gnu")));
    assert_eq!(rcpt(&b"RCPT gnu\n"[..]), IResult::Done(&b""[..], Command::Rcpt("gnu")));
}
