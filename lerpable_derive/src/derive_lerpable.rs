use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::parser::*;

pub(crate) struct FieldTokensLerpable {
    pub(crate) for_lerpable: TokenStream2,
}
impl GenFinal for FieldTokensLerpable {
    // Something(f32)
    fn make_newtype_struct_final(
        idents: ParsedFieldIdent,
        variants: Vec<FieldTokensLerpable>,
    ) -> TokenStream2 {
        let name = idents.name;

        let for_lerpable = variants.iter().map(|x| x.for_lerpable.clone());

        quote! {
            impl lerpable::Lerpable for #name {
                fn lerpify<T: lerpable::IsLerpingMethod>(&self, other: &Self, pct: &T) -> Self {
                    #name(#(#for_lerpable,)*)
                }
            }
        }
    }

    fn make_struct_final(
        idents: ParsedFieldIdent,
        variants: Vec<FieldTokensLerpable>,
    ) -> TokenStream2 {
        let name = idents.name;

        let for_lerpable = variants.iter().map(|a| a.for_lerpable.clone());

        quote! {
            impl lerpable::Lerpable for #name {
                fn lerpify<T: lerpable::IsLerpingMethod>(&self, other: &Self, pct: &T) -> Self {
                    #name {
                        #(#for_lerpable,)*
                    }
                }
            }
        }
    }

    fn make_enum_final(
        idents: ParsedFieldIdent,
        variants: Vec<FieldTokensLerpable>,
    ) -> TokenStream2 {
        let name = idents.name;

        let for_lerpable = variants.iter().map(|a| a.for_lerpable.clone());

        quote! {
            impl lerpable::Lerpable for #name {
                fn lerpify<T: lerpable::IsLerpingMethod>(&self, other: &Self, pct: &T) -> Self {
                    match (self, other) {
                        #(#for_lerpable,)*
                        _ => lerpable::step(self, other, pct)
                    }
                }
            }
        }
    }

    fn from_newtype_struct(idents: StructIdents, _parent_ident: syn::Ident) -> FieldTokensLerpable {
        let method_def = idents.to_method_override();

        let for_lerpable = quote! {
            {
                #method_def
                self.0.lerpify(&other.0, method)
            }
        };

        FieldTokensLerpable { for_lerpable }
    }

    // e.g. TileAxisLocs::V(TileAxisVs)
    fn from_unnamed_enum(idents: EnumIdents) -> FieldTokensLerpable {
        let variant_ident = idents.variant_ident();
        let name = idents.enum_ident();

        // if they're the same, lerp the struct inside. otherwise, will default to the step!
        let for_lerpable = quote! {
            (#name::#variant_ident(self_s), #name::#variant_ident(other_s)) => #name::#variant_ident(self_s.lerpify(&other_s, pct))
        };

        FieldTokensLerpable { for_lerpable }
    }

    // e.g. TileAxis::Diag
    fn from_unit_enum(idents: EnumIdents) -> FieldTokensLerpable {
        let variant_ident = idents.variant_ident();
        let name = idents.enum_ident();

        // hmm, not really needed, since it can fall back on th global step
        let for_lerpable: TokenStream2 = {
            quote! { (#name::#variant_ident, #name::#variant_ident) => lerpable::step(self, other, pct) }
        };

        FieldTokensLerpable { for_lerpable }
    }

    // s: String
    fn from_noop_struct(idents: StructIdents) -> FieldTokensLerpable {
        let name = idents.name();

        let method_def = idents.to_method_override();

        let for_lerpable: TokenStream2 = quote! {
            #name: {
                #method_def;
                lerpable::step(&self.#name, &other.#name, method)
            }
        };

        FieldTokensLerpable { for_lerpable }
    }

    // f32, Vec2, etc
    fn from_type_struct(idents: StructIdents) -> FieldTokensLerpable {
        let name = idents.name();

        let method_def = idents.to_method_override();

        // we'll just use the trait! (unless it's none, then we bail)
        let for_lerpable = {
            quote! { #name: {
                #method_def
                self.#name.lerpify(&other.#name, method)
            }}
        };

        FieldTokensLerpable { for_lerpable }
    }
}
