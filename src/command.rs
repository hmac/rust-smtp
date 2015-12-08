pub type EmailAddress<'a> = &'a str;
pub type ServerAddress<'a>  = &'a str;

#[derive(Debug,PartialEq)]
pub enum Command<'a> {
    Helo(ServerAddress<'a>), // contains address of connecting server
    Mail(EmailAddress<'a>),  // contains from address
    Rcpt(EmailAddress<'a>),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
}
