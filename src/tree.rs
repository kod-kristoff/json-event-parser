use std::fmt;

#[derive(Debug)]
#[rustfmt::skip]
pub enum TreeKind {
    ErrorTree,
}

pub struct Tree {
    pub kind: TreeKind,
}

use crate::format_to;
impl Tree {
    fn print(&self, buf: &mut String, level: usize) -> Result<(), fmt::Error> {
        let indent = " ".repeat(level);
        format_to!(buf, "{indent}{:?}\n", self.kind);
        Ok(())
    }
}
impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        self.print(&mut buf, 0)?;
        write!(f, "{}", buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
