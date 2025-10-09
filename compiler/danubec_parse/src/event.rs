use danubec_syntax::SyntaxKind;

pub struct EventStream {
    events: Vec<Event>,
}

#[derive(Debug)]
pub enum Event {
    Placeholder,
    Start {
        kind: SyntaxKind,
        forward_parent: Option<usize>,
    },
    Token,
    End,
    Expire {
        forward_parent: Option<usize>,
    },
}

pub struct Marker {
    index: usize,
    bomb: drop_bomb::DropBomb,
}

pub struct CompleteMarker {
    index: usize,
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

    pub fn complete(&mut self, m: Marker, kind: SyntaxKind) -> CompleteMarker {
        self.events[m.index] = Event::Start {
            kind,
            forward_parent: None,
        };
        self.events.push(Event::End);

        m.complete()
    }

    pub fn expire(&mut self) -> CompleteMarker {
        let m = self.reserve();
        self.events[m.index] = Event::Expire {
            forward_parent: None,
        };

        m.complete()
    }

    pub fn precede(&mut self, cm: CompleteMarker) -> Marker {
        let m = self.reserve();

        match &mut self.events[cm.index] {
            Event::Start { forward_parent, .. } => {
                *forward_parent = Some(m.index - cm.index);
            }
            Event::Expire { forward_parent } => {
                *forward_parent = Some(m.index - cm.index);
            }
            _ => {
                //
            }
        }

        m
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
    fn complete(mut self) -> CompleteMarker {
        self.bomb.defuse();

        CompleteMarker { index: self.index }
    }

    #[inline]
    pub fn terminate(mut self) {
        self.bomb.defuse();
    }
}
