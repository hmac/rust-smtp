use std::fmt;

#[derive(Debug,PartialEq,Clone)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.local, self.domain)
    }
}

pub type ServerAddress  = String;

#[derive(Debug,PartialEq)]
pub enum Command {
    Helo(ServerAddress), // contains address of connecting server
    Mail(EmailAddress),  // contains from address
    Rcpt(EmailAddress),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
}

