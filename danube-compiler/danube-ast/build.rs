#[macro_use]
extern crate quote;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use std::process::Command;
use syn::{parse_file, Fields, FieldsNamed, FieldsUnnamed, File, Item};

const INPUT: &'static str = include_str!("./src/ast.rs");

fn main() -> std::io::Result<()> {
    let input = parse_file(INPUT).unwrap();

    std::fs::write("./src/visit.rs", build_visit(input.clone()).to_string()).unwrap();

    Command::new("rustfmt")
        .arg("./src/visit.rs")
        .spawn()?
        .wait()?;

    Ok(())
}

fn build_visit(input: File) -> TokenStream {
    let mut visits = quote! {};
    let mut walks = quote! {};

    for item in input.items {
        match item {
            Item::Struct(struct_) => {
                let struct_name = &struct_.ident;
                let name = Ident::new(
                    snake_case(&struct_name.to_string()).as_str(),
                    Span::call_site(),
                );
                let visit_name = Ident::new(format!("visit_{}", name).as_str(), Span::call_site());
                let walk_name = Ident::new(format!("walk_{}", name).as_str(), Span::call_site());

                visits.extend(quote! {
                    fn #visit_name(&mut self, context: &Self::Context, #name: &'ast #struct_name) {
                        #walk_name(self, context, #name);
                    }
                });

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
                                                                ) => {
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
                                                                    for #field_name in #name.#field_name.iter() {
                                                                        visitor.#visit_name(context, #field_name);
                                                                    }
                                                                });
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
                                                                    visitor.#visit_name(context, &#name.#field_name);
                                                                });
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
                                                    _ => {
                                                        let visit_name = Ident::new(
                                                            format!(
                                                                "visit_{}",
                                                                snake_case(ident.as_str())
                                                            )
                                                            .as_str(),
                                                            Span::call_site(),
                                                        );
                                                        fields.extend(quote! {
                                                            visitor.#visit_name(context, &#name.#field_name);
                                                        });
                                                    }
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
                walks.extend(quote! {
                    #[allow(unused_variables)]
                    pub fn #walk_name<'ast, V: Visit<'ast>>(visitor: &mut V, context: &V::Context, #name: &'ast #struct_name)
                    where V::Context: VisitContext, {
                        #fields
                    }
                });
            }
            Item::Enum(enum_) => {
                let enum_name = enum_.ident;
                let name = Ident::new(
                    snake_case(&enum_name.to_string()).as_str(),
                    Span::call_site(),
                );
                let visit_name = Ident::new(format!("visit_{}", name).as_str(), Span::call_site());
                let walk_name = Ident::new(format!("walk_{}", name).as_str(), Span::call_site());

                visits.extend(quote! {
                    fn #visit_name(&mut self, context: &Self::Context, #name: &'ast #enum_name) {
                        #walk_name(self, context, #name);
                    }
                });

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
                                                                ) => {
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
                                                                    variants.extend(quote! {
                                                                        #enum_name::#variant_name(node) => {
                                                                            if let Some(node) = node {
                                                                                visitor.#visit_name(context, node);
                                                                            }
                                                                        },
                                                                    });
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
                                                }
                                            }
                                            "Vec" => match segment.arguments {
                                                syn::PathArguments::AngleBracketed(ref node) => {
                                                    for argument in node.args.iter() {
                                                        match argument {
                                                            syn::GenericArgument::Type(ref ty) => {
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
                                                                variants.extend(quote! {
                                                                    #enum_name::#variant_name(nodes) => {
                                                                        for node in nodes {
                                                                            visitor.#visit_name(context, node);
                                                                        }
                                                                    }
                                                                });
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
                                                                variants.extend(quote! {
                                                                    #enum_name::#variant_name(node) => {
                                                                        visitor.#visit_name(context, node);
                                                                    }
                                                                });
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
                                            _ => {
                                                let visit_name = Ident::new(
                                                    format!("visit_{}", snake_case(ident.as_str()))
                                                        .as_str(),
                                                    Span::call_site(),
                                                );
                                                variants.extend(quote! {
                                                    #enum_name::#variant_name(node) => {
                                                        visitor.#visit_name(context, node);
                                                    }
                                                })
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
                }

                walks.extend(quote! {
                    #[allow(unused_variables)]
                    fn #walk_name<'ast, V: Visit<'ast>>(visitor: &mut V, context: &V::Context, #name: &'ast #enum_name)
                    where V::Context: VisitContext,
                    {
                        match #name {
                            #variants
                        }
                    }
                });
            }
            _ => {
                //
            }
        }
    }

    quote! {
        // ! AUTOGENERATED
        // ! DO NOT EDIT

        use crate::ast::*;
        use danube_diagnostics::Message;

        pub trait VisitContext {
            fn report(&self, message: Message);
        }

        #[allow(unused_variables)]
        pub trait Visit<'ast>: Sized
        where Self::Context: VisitContext,
        {
            type Context;

            #visits
        }

        #walks
    }
}

fn snake_case(ident: &str) -> String {
    ident.from_case(Case::Pascal).to_case(Case::Snake)
}
