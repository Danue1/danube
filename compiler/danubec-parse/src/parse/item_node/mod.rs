mod enum_item_node;
mod struct_item_node;

impl crate::Context {
    pub fn item_node(&mut self) -> crate::State {
        one_of!(self.struct_item_node(), self.enum_item_node())
    }
}
