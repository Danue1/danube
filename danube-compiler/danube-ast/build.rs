#[macro_use]
extern crate quote;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use std::process::Command;
use syn::{parse_file, Fields, FieldsNamed, FieldsUnnamed, File, Item};

const INPUT: &'static str = include_str!("./src/ast.rs");

fn main() -> std::io::Result<()> {
    let input = parse_file(INPUT).unwrap();

    std::fs::write(
        "./src/visit.rs",
        build_trait(input.clone(), Mode::Visit).to_string(),
    )
    .unwrap();
    std::fs::write(
        "./src/fold.rs",
        build_trait(input.clone(), Mode::Fold).to_string(),
    )
    .unwrap();

    Command::new("rustfmt")
        .arg("./src/visit.rs")
        .arg("./src/fold.rs")
        .spawn()?
        .wait()?;

    Ok(())
}

enum Mode {
    Visit,
    Fold,
}

fn build_trait(input: File, mode: Mode) -> TokenStream {
    let mut trait_fns = quote! {};
    let mut walks = quote! {};

    for item in input.items {
        match item {
            Item::Struct(struct_) => {
                let struct_name = &struct_.ident;
                let name = Ident::new(
                    snake_case(&struct_name.to_string()).as_str(),
                    Span::call_site(),
                );
                let visit_name = match mode {
                    Mode::Visit => {
                        Ident::new(format!("visit_{}", name).as_str(), Span::call_site())
                    }
                    Mode::Fold => Ident::new(format!("fold_{}", name).as_str(), Span::call_site()),
                };
                let walk_name = Ident::new(format!("walk_{}", name).as_str(), Span::call_site());

                match mode {
                    Mode::Visit => {
                        trait_fns.extend(quote! {
                            fn #visit_name(&mut self, context: &Self::Context, #name: &'ast #struct_name) {
                                #walk_name(self, context, #name);
                            }
                        });
                    }
                    Mode::Fold => {
                        trait_fns.extend(quote! {
                            fn #visit_name(&mut self, context: &Self::Context, #name: &'ast mut #struct_name) {
                                #walk_name(self, context, #name);
                            }
                        });
                    }
                }

                let mut fields = quote! {};
                match struct_.fields {
                    syn::Fields::Named(named_fields) => {
                        for argument in named_fields.named.iter() {
                            let field_name = &argument.ident;
                            let ty = &argument.ty;

                            match ty {
                                syn::Type::Path(node) => match node.path.segments.first() {
                                    Some(segment) => {
                                        let ident = segment.ident.to_string();
                                        match ident.as_str() {
                                            "Option" => {
                                                match segment.arguments {
                                                    syn::PathArguments::AngleBracketed(
                                                        ref node,
                                                    ) => {
                                                        for argument in node.args.iter() {
                                                            match argument {
                                                                syn::GenericArgument::Type(
                                                                    ref ty,
                                                                ) => match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str()
                                                                                )
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            if let Some(ref #field_name) = #name.#field_name {
                                                                                visitor.#visit_name(context, #field_name);
                                                                            }
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            if let Some(ref mut #field_name) = #name.#field_name {
                                                                                folder.#fold_name(context, #field_name);
                                                                            }
                                                                        });
                                                                    }
                                                                },
                                                                _ => {
                                                                    //
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        //
                                                    }
                                                }
                                            }
                                            "Vec" => match segment.arguments {
                                                syn::PathArguments::AngleBracketed(ref node) => {
                                                    for argument in node.args.iter() {
                                                        match argument {
                                                            syn::GenericArgument::Type(ref ty) => {
                                                                match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            for #field_name in #name.#field_name.iter() {
                                                                                visitor.#visit_name(context, #field_name);
                                                                            }
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            for #field_name in #name.#field_name.iter_mut() {
                                                                                folder.#fold_name(context, #field_name);
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                //
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    //
                                                }
                                            },
                                            "Box" => match segment.arguments {
                                                syn::PathArguments::AngleBracketed(ref node) => {
                                                    for argument in node.args.iter() {
                                                        match argument {
                                                            syn::GenericArgument::Type(ref ty) => {
                                                                match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            visitor.#visit_name(context, &#name.#field_name);
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        fields.extend(quote! {
                                                                            folder.#fold_name(context, &mut #name.#field_name);
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                //
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    //
                                                }
                                            },
                                            "NodeId" | "Symbol" | "AttributeId" | "LiteralKind" => {
                                                //
                                            }
                                            _ => {
                                                match ident.as_str() {
                                                    "bool" => {
                                                        //
                                                    }
                                                    _ => match mode {
                                                        Mode::Visit => {
                                                            let visit_name = Ident::new(
                                                                format!(
                                                                    "visit_{}",
                                                                    snake_case(ident.as_str()),
                                                                )
                                                                .as_str(),
                                                                Span::call_site(),
                                                            );
                                                            fields.extend(quote! {
                                                                visitor.#visit_name(context, &#name.#field_name);
                                                            });
                                                        }
                                                        Mode::Fold => {
                                                            let fold_name = Ident::new(
                                                                format!(
                                                                    "fold_{}",
                                                                    snake_case(ident.as_str()),
                                                                )
                                                                .as_str(),
                                                                Span::call_site(),
                                                            );
                                                            fields.extend(quote! {
                                                                folder.#fold_name(context, &mut #name.#field_name);
                                                            });
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        //
                                    }
                                },
                                _ => {
                                    //
                                }
                            }
                        }
                    }
                    _ => {
                        //
                    }
                }
                match mode {
                    Mode::Visit => {
                        walks.extend(quote! {
                            #[allow(unused_variables)]
                            pub fn #walk_name<'ast, V: Visit<'ast>>(visitor: &mut V, context: &V::Context, #name: &'ast #struct_name) {
                                #fields
                            }
                        });
                    }
                    Mode::Fold => {
                        walks.extend(quote! {
                            #[allow(unused_variables)]
                            pub fn #walk_name<'ast, F: Fold<'ast>>(folder: &mut F, context: &F::Context, #name: &'ast mut #struct_name) {
                                #fields
                            }
                        });
                    }
                }
            }
            Item::Enum(enum_) => {
                let enum_name = enum_.ident;
                let name = Ident::new(
                    snake_case(&enum_name.to_string()).as_str(),
                    Span::call_site(),
                );
                let visit_name = match mode {
                    Mode::Visit => {
                        Ident::new(format!("visit_{}", name).as_str(), Span::call_site())
                    }
                    Mode::Fold => Ident::new(format!("fold_{}", name).as_str(), Span::call_site()),
                };
                let walk_name = Ident::new(format!("walk_{}", name).as_str(), Span::call_site());

                match mode {
                    Mode::Visit => {
                        trait_fns.extend(quote! {
                            fn #visit_name(&mut self, context: &Self::Context, #name: &'ast #enum_name) {
                                #walk_name(self, context, #name);
                            }
                        });
                    }
                    Mode::Fold => {
                        trait_fns.extend(quote! {
                            fn #visit_name(&mut self, context: &Self::Context, #name: &'ast mut #enum_name) {
                                #walk_name(self, context, #name);
                            }
                        });
                    }
                }

                let mut variants = quote! {};
                for variant in enum_.variants {
                    let variant_name = variant.ident;

                    match variant.fields {
                        Fields::Unit => {
                            variants.extend(quote! {
                                #enum_name::#variant_name => {
                                    //
                                },
                            });
                        }
                        Fields::Named(FieldsNamed { named: _named, .. }) => {
                            //
                        }
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                            let field = unnamed.first().unwrap();
                            match &field.ty {
                                syn::Type::Path(node) => match node.path.segments.first() {
                                    Some(segment) => {
                                        let ident = segment.ident.to_string();
                                        match ident.as_str() {
                                            "Option" => {
                                                match segment.arguments {
                                                    syn::PathArguments::AngleBracketed(
                                                        ref node,
                                                    ) => {
                                                        for argument in node.args.iter() {
                                                            match argument {
                                                                syn::GenericArgument::Type(
                                                                    ref ty,
                                                                ) => match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(node) => {
                                                                                if let Some(node) = node {
                                                                                    visitor.#visit_name(context, node);
                                                                                }
                                                                            },
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(node) => {
                                                                                if let Some(node) = node {
                                                                                    folder.#fold_name(context, node);
                                                                                }
                                                                            },
                                                                        });
                                                                    }
                                                                },
                                                                _ => {
                                                                    //
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        //
                                                    }
                                                }
                                            }
                                            "Vec" => match segment.arguments {
                                                syn::PathArguments::AngleBracketed(ref node) => {
                                                    for argument in node.args.iter() {
                                                        match argument {
                                                            syn::GenericArgument::Type(ref ty) => {
                                                                match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(nodes) => {
                                                                                for node in nodes.iter() {
                                                                                    visitor.#visit_name(context, node);
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(nodes) => {
                                                                                for node in nodes.iter_mut() {
                                                                                    folder.#fold_name(context, node);
                                                                                }
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                //
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    //
                                                }
                                            },
                                            "Box" => match segment.arguments {
                                                syn::PathArguments::AngleBracketed(ref node) => {
                                                    for argument in node.args.iter() {
                                                        match argument {
                                                            syn::GenericArgument::Type(ref ty) => {
                                                                match mode {
                                                                    Mode::Visit => {
                                                                        let visit_name = Ident::new(
                                                                            format!(
                                                                                "visit_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(node) => {
                                                                                visitor.#visit_name(context, node);
                                                                            }
                                                                        });
                                                                    }
                                                                    Mode::Fold => {
                                                                        let fold_name = Ident::new(
                                                                            format!(
                                                                                "fold_{}",
                                                                                snake_case(
                                                                                    quote! { #ty }
                                                                                        .to_string()
                                                                                        .as_str(),
                                                                                ),
                                                                            )
                                                                            .as_str(),
                                                                            Span::call_site(),
                                                                        );
                                                                        variants.extend(quote! {
                                                                            #enum_name::#variant_name(node) => {
                                                                                folder.#fold_name(context, node);
                                                                            }
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                //
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    //
                                                }
                                            },
                                            _ => match mode {
                                                Mode::Visit => {
                                                    let visit_name = Ident::new(
                                                        format!(
                                                            "visit_{}",
                                                            snake_case(ident.as_str()),
                                                        )
                                                        .as_str(),
                                                        Span::call_site(),
                                                    );
                                                    variants.extend(quote! {
                                                        #enum_name::#variant_name(node) => {
                                                            visitor.#visit_name(context, node);
                                                        }
                                                    });
                                                }
                                                Mode::Fold => {
                                                    let fold_name = Ident::new(
                                                        format!(
                                                            "fold_{}",
                                                            snake_case(ident.as_str()),
                                                        )
                                                        .as_str(),
                                                        Span::call_site(),
                                                    );
                                                    variants.extend(quote! {
                                                        #enum_name::#variant_name(node) => {
                                                            folder.#fold_name(context, node);
                                                        }
                                                    });
                                                }
                                            },
                                        }
                                    }
                                    _ => {
                                        //
                                    }
                                },
                                _ => {
                                    //
                                }
                            }
                        }
                    }
                }

                match mode {
                    Mode::Visit => {
                        walks.extend(quote! {
                            #[allow(unused_variables)]
                            fn #walk_name<'ast, V: Visit<'ast>>(visitor: &mut V, context: &V::Context, #name: &'ast #enum_name) {
                                match #name {
                                    #variants
                                }
                            }
                        });
                    }
                    Mode::Fold => {
                        walks.extend(quote! {
                            #[allow(unused_variables)]
                            fn #walk_name<'ast, F: Fold<'ast>>(folder: &mut F, context: &F::Context, #name: &'ast mut #enum_name) {
                                match #name {
                                    #variants
                                }
                            }
                        });
                    }
                }
            }
            _ => {
                //
            }
        }
    }

    match mode {
        Mode::Visit => {
            quote! {
                // ! AUTOGENERATED
                // ! DO NOT EDIT

                use crate::ast::*;

                #[allow(unused_variables)]
                pub trait Visit<'ast>: Sized {
                    type Context;

                    #trait_fns
                }

                #walks
            }
        }
        Mode::Fold => {
            quote! {
                // ! AUTOGENERATED
                // ! DO NOT EDIT

                use crate::ast::*;

                #[allow(unused_variables)]
                pub trait Fold<'ast>: Sized {
                    type Context;

                    #trait_fns
                }

                #walks
            }
        }
    }
}

fn snake_case(ident: &str) -> String {
    ident.from_case(Case::Pascal).to_case(Case::Snake)
}
