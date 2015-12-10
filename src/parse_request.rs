use nom::{IResult, space, not_line_ending, crlf};
use nom::IResult::*;

use std::str;
use command::{Command};

named!(data <&[u8], Command>,
       chain!(
           tag!("DATA") ~
           space?       ~
           crlf         ,
           || { Command::Data }));

named!(helo <&[u8], Command>,
    chain!(
       tag!("HELO")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       crlf                 ,
       || { Command::Helo(rest.to_string()) }
   )
);

named!(mail <&[u8], Command>,
    chain!(
       tag!("MAIL")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       crlf                 ,
       || { Command::Mail(rest.to_string()) }
   )
);

named!(rcpt <&[u8], Command>,
    chain!(
       tag!("RCPT")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       crlf                 ,
       || { Command::Rcpt(rest.to_string()) }
   )
);

named!(quit <&[u8], Command>,
    chain!(
        tag!("QUIT") ~
        space?       ~
        crlf         ,
        || { Command::Terminate }));

named!(command <&[u8], Command>,
    alt!(
        helo |
        mail |
        rcpt |
        data |
        quit
    )
);

pub fn parse(req: &str) -> Result<Command, u8> {
    match command(req.as_bytes()) {
        IResult::Error(err) => {
           println!("{:?}", err);
           Err(1)
        },
        IResult::Incomplete(err) => {
           println!("{:?}", err);
           Err(1)
        },
        IResult::Done(_, command) => Ok(command)
    }
}

#[test]
fn test_parser() {
    assert_eq!(mail(&b"MAIL gnu\r\n"[..]), IResult::Done(&b""[..], Command::Mail("gnu")));
    assert_eq!(helo(&b"HELO gnu\r\n"[..]), IResult::Done(&b""[..], Command::Helo("gnu")));
    assert_eq!(rcpt(&b"RCPT gnu\r\n"[..]), IResult::Done(&b""[..], Command::Rcpt("gnu")));
    assert_eq!(
        command(&b"HELO smtp.gnu.org\r\n"[..]),
        IResult::Done(&b""[..], Command::Helo("smtp.gnu.org"))
    );
    assert_eq!(
        command(&b"MAIL FROM:<bill@gnu.org>\r\n"[..]),
        IResult::Done(&b""[..], Command::Mail("FROM:<bill@gnu.org>"))
    );
}
