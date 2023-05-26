use crate::parser::Parser;
use crate::TreeKind::*;

pub fn start(p: &mut Parser) {
    let m = p.open();
    p.close(m, Start);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
