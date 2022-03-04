danube_index::newtype_index! {
    pub struct NodeId(usize);
    pub struct AttributeId(usize);
}

pub const DUMMY_NODE_ID: NodeId = NodeId(0);
pub const DUMMY_ATTRIBUTE_ID: AttributeId = AttributeId(0);
