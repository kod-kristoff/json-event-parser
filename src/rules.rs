use crate::parser::Parser;
use crate::TokenKind::*;
use crate::TreeKind::*;

pub fn start(p: &mut Parser) {
    println!("rules.start");
    let m = p.open();
    dbg!(p.eof());
    while !p.eof() {
        dbg!(p.nth(0));
        if p.at(LCurly) {
            object(p);
        } else {
            p.advance_with_error("expected an object");
        }
    }
    p.close(m, Start);
}

fn object(p: &mut Parser) {
    assert!(p.at(LCurly));
    let m = p.open();
    p.expect(LCurly);
    p.expect(RCurly);
    p.close(m, Object);
    // todo!("parse object")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
