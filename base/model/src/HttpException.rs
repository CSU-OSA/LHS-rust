use std::fmt::{Debug, Display, Formatter, write};
use std::sync::Arc;

use String;

#[derive(Debug)]
pub struct HttpException {
    msg: Arc<String>,
}

impl From<&str> for HttpException {
    fn from(msg: &str) -> Self {
        Self {
            msg: Arc::new(String::from(msg))
        }
    }
}

impl Display for HttpException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}