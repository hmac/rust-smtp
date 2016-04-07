use nom::{IResult, space, not_line_ending};

use std::str;
use command::{Command, EmailAddress};

named!(rn, alt!(tag!("\r\n") | tag!("\n")));

named!(data <&[u8], Command>,
       chain!(
           tag!("DATA") ~
           space?       ~
           rn           ,
           || { Command::Data }));

named!(helo <&[u8], Command>,
    chain!(
       tag!("HELO")    ~
       space                ~
       rest: map_res!(
           not_line_ending,
           str::from_utf8
       )                    ~
       rn                   ,
       || { Command::Helo(rest.to_string()) }
   )
);

named!(mail <&[u8], Command>,
    chain!(
       tag!("MAIL")         ~
       space                ~
       tag!("FROM:")        ~
       address: email_address  ~
       rn                   ,
       || { Command::Mail(address) }
   )
);

named!(rcpt <&[u8], Command>,
    chain!(
       tag!("RCPT")    ~
       space                ~
       address: email_address ~
       rn                   ,
       || { Command::Rcpt(address) }
   )
);

named!(quit <&[u8], Command>,
    chain!(
        tag!("QUIT") ~
        space?       ~
        rn           ,
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

named!(email_address <&[u8], EmailAddress>,
   chain!(
       tag!("<") ~
       local: map_res!(is_not!("@"), str::from_utf8) ~
       tag!("@") ~
       domain: map_res!(take_until_either!("\r\n"), str::from_utf8) ,
       || { EmailAddress { local: local.to_string(), domain: domain.to_string() } }
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
    assert_eq!(mail(&b"MAIL gnu\r\n"[..]), IResult::Done(&b""[..], Command::Mail("gnu".to_string())));
    assert_eq!(helo(&b"HELO gnu\r\n"[..]), IResult::Done(&b""[..], Command::Helo("gnu".to_string())));
    assert_eq!(rcpt(&b"RCPT gnu\r\n"[..]), IResult::Done(&b""[..], Command::Rcpt("gnu".to_string())));
    assert_eq!(
        command(&b"HELO smtp.gnu.org\r\n"[..]),
        IResult::Done(&b""[..], Command::Helo("smtp.gnu.org".to_string()))
    );
    assert_eq!(
        command(&b"MAIL FROM:<bill@gnu.org>\r\n"[..]),
        IResult::Done(&b""[..], Command::Mail("FROM:<bill@gnu.org>".to_string()))
    );
}
