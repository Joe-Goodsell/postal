use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum HttpVerb {
    GET,
    POST,
}

impl fmt::Display for HttpVerb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpVerb::GET => write!(f, "GET"),
            HttpVerb::POST => write!(f, "POST"),
        }
    }
}
