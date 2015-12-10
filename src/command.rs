pub type EmailAddress = String;
pub type ServerAddress  = String;

#[derive(Debug,PartialEq)]
pub enum Command {
    Helo(ServerAddress), // contains address of connecting server
    Mail(EmailAddress),  // contains from address
    Rcpt(EmailAddress),  // contains to address
    Data,                // mail message to follow
    Terminate            // client has disconnected
}
