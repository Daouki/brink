use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct NodeId(usize);

#[allow(dead_code)]
impl NodeId {
    pub fn from_usize(id: usize) -> Self {
        Self(id)
    }

    pub fn as_usize(self) -> usize {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", self.0)
    }
}

pub struct NodeIdGenerator {
    next_id: usize,
}

impl NodeIdGenerator {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        NodeId(id)
    }
}
