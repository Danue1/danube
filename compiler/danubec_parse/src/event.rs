use danubec_syntax::SyntaxKind;

pub struct EventStream {
    events: Vec<Event>,
}

#[derive(Debug)]
pub enum Event {
    Placeholder,
    Start(SyntaxKind),
    Token,
    End,
}

pub struct Marker {
    index: usize,
    bomb: drop_bomb::DropBomb,
}

impl EventStream {
    pub const fn new() -> Self {
        Self { events: vec![] }
    }

    pub fn reserve(&mut self) -> Marker {
        let index = self.events.len();
        self.events.push(Event::Placeholder);

        Marker::new(index)
    }

    pub fn token(&mut self) {
        self.events.push(Event::Token);
    }

    pub fn complete(&mut self, m: Marker, kind: SyntaxKind) {
        self.events[m.index] = Event::Start(kind);
        self.events.push(Event::End);
        m.complete();
    }

    #[inline]
    pub fn finalize(self) -> Vec<Event> {
        self.events
    }
}

impl Marker {
    fn new(index: usize) -> Self {
        Self {
            index,
            bomb: drop_bomb::DropBomb::new("Marker not completed"),
        }
    }

    #[inline]
    fn complete(mut self) {
        self.bomb.defuse();
    }

    #[inline]
    pub fn expire(mut self) {
        self.bomb.defuse();
    }
}
