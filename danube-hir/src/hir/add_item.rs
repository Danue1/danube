use crate::*;

impl HirContext {
    fn add_item(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
        kind: ItemKind,
    ) {
        let id = self.next_id();
        let item = Item {
            id,
            visibility,
            ident,
            attribute_list,
            kind,
        };
        self.items.insert(id, item);
    }

    pub(super) fn add_use(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
        path: Path,
    ) {
        self.add_item(visibility, ident, attribute_list, ItemKind::Use(path));
    }

    pub(super) fn add_module(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_named_struct(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
        fields: Vec<StructField>,
    ) {
        self.add_item(
            visibility,
            ident,
            attribute_list,
            ItemKind::Struct(VariantKind::Named(fields)),
        )
    }

    pub(super) fn add_unnamed_struct(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
        fields: Vec<StructField>,
    ) {
        let id = self.next_id();
        self.add_item(
            visibility,
            ident,
            attribute_list,
            ItemKind::Struct(VariantKind::Unnamed(fields, id)),
        )
    }

    pub(super) fn add_enum(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_function(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_type_alias(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_trait(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_constant(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_static(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }

    pub(super) fn add_implement(
        &mut self,
        visibility: VisibilityKind,
        ident: Ident,
        attribute_list: Vec<Attribute>,
    ) {
        //
    }
}
