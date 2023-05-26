use crate::{Lexer, Tree, TreeKind};

pub struct Parser {
    tokens: Lexer,
    events: Vec<Event>,
}

#[derive(Debug)]
pub enum Event {
    Open { kind: TreeKind },
    Close,
    Advance,
}
pub struct MarkOpened {
    index: usize,
}
impl Parser {
    pub fn new(tokens: Lexer) -> Self {
        Self {
            tokens,
            events: Vec::new(),
        }
    }

    pub fn build_tree(self) -> Tree {
        let mut tokens = self.tokens;
        let mut events = self.events;
        let mut stack = Vec::new();

        assert!(matches!(events.pop(), Some(Event::Close)));

        for event in events {
            match event {
                Event::Open { kind } => stack.push(Tree {
                    kind,
                    children: Vec::new(),
                }),
                Event::Close => {
                    let tree = stack.pop().unwrap();
                    stack
                        .last_mut()
                        .unwrap()
                        .children
                        .push(crate::tree::Child::Tree(tree));
                }
                Event::Advance => todo!("advance"),
            }
        }

        stack.pop().unwrap()
    }

    pub fn open(&mut self) -> MarkOpened {
        let mark = MarkOpened {
            index: self.events.len(),
        };
        self.events.push(Event::Open {
            kind: TreeKind::ErrorTree,
        });
        mark
    }

    pub fn close(&mut self, m: MarkOpened, kind: TreeKind) {
        self.events[m.index] = Event::Open { kind };
        self.events.push(Event::Close);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
