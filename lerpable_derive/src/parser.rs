use darling::{ast, FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Debug)]
pub(crate) struct ParsedFieldIdent {
    pub(crate) name: syn::Ident,
}

// trait and helpers needed to parse a variety of objects
pub(crate) trait GenFinal
where
    Self: Sized,
{
    fn from_newtype_struct(_idents: StructIdents, parent_ident: syn::Ident) -> Self;
    fn from_newtype_recurse_struct_vec(_idents: StructIdents) -> Self;
    fn from_unnamed_enum(idents: EnumIdents) -> Self;
    fn from_unit_enum(idents: EnumIdents) -> Self;
    fn from_noop_struct(idents: StructIdents) -> Self;
    fn from_type_struct(idents: StructIdents) -> Self;
    fn from_recurse_struct_vec(idents: StructIdents) -> Self;

    fn from_ast(ast_receiver: LivecodeReceiver) -> TokenStream2 {
        match ast_receiver.data {
            ast::Data::Enum(_) => Self::make_enum(&ast_receiver),
            ast::Data::Struct(ast::Fields {
                style: ast::Style::Tuple,
                ..
            }) => Self::make_newtype(&ast_receiver),
            ast::Data::Struct(_) => Self::make_struct(&ast_receiver),
        }
    }

    fn make_enum_final(idents: ParsedFieldIdent, variants: Vec<Self>) -> TokenStream2;
    fn make_struct_final(idents: ParsedFieldIdent, variants: Vec<Self>) -> TokenStream2;
    fn make_newtype_struct_final(idents: ParsedFieldIdent, variants: Vec<Self>) -> TokenStream2;

    fn make_struct(s: &LivecodeReceiver) -> TokenStream2 {
        let name = s.ident.clone();

        #[cfg(feature = "debug_logging")]
        log::info!("{}::make_struct {}", Self::classname(), name.to_string());

        // shouldn't be calling this with something that's not a struct..
        let fields = s.data.clone().take_struct().unwrap();

        let livecodable_fields = fields
            .iter()
            .map(|field| {
                let idents = StructIdents {
                    data: field.clone(),
                };

                match field.how_to_control_this() {
                    // leave this field alone (useful for String and HashMaps)
                    HowToControlThis::Skip => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_noop_struct");
                        Self::from_noop_struct(idents)
                    }
                    // creating with a set type
                    HowToControlThis::LerpifyType => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_type_struct");
                        Self::from_type_struct(idents)
                    }
                    // creating a Vec<Something>
                    HowToControlThis::Vec => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_recurse_struct_vec");
                        Self::from_recurse_struct_vec(idents)
                    }
                }
            })
            .collect::<Vec<_>>();

        let idents = ParsedFieldIdent { name: name.clone() };

        Self::make_struct_final(idents, livecodable_fields)
    }

    fn make_enum(e: &LivecodeReceiver) -> TokenStream2 {
        let name = e.ident.clone();

        #[cfg(feature = "debug_logging")]
        log::info!("{}::make_enum {}", Self::classname(), name.to_string());

        let variants = e.data.clone().take_enum().unwrap();

        // just go through and find ones that wrap around a type, and make sure those types are
        let variants = variants
            .iter()
            .map(|variant| {
                let ident = EnumIdents {
                    enum_name: name.clone(),
                    data: variant.clone(),
                };

                match variant.fields.style {
                    ast::Style::Tuple => Self::from_unnamed_enum(ident),
                    ast::Style::Struct => panic!("enum named fields not supported yet"),
                    ast::Style::Unit => Self::from_unit_enum(ident),
                }
            })
            .collect::<Vec<_>>();

        let idents = ParsedFieldIdent { name: name.clone() };

        Self::make_enum_final(idents, variants)
    }

    fn make_newtype(s: &LivecodeReceiver) -> TokenStream2 {
        let name = s.ident.clone();

        #[cfg(feature = "debug_logging")]
        log::info!("{}::make_newtype {}", Self::classname(), name.to_string());

        // shouldn't be calling this with something that's not a struct..
        let fields = s.data.clone().take_struct().unwrap();

        let livecodable_fields = fields
            .iter()
            .map(|field| {
                let idents = StructIdents {
                    data: field.clone(),
                };

                match field.how_to_control_this() {
                    HowToControlThis::LerpifyType => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_newtype_struct");
                        Self::from_newtype_struct(idents, name.clone())
                    }
                    // creating a Vec<Something>
                    HowToControlThis::Vec => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_newtype_recurse_struct_vec");
                        Self::from_newtype_recurse_struct_vec(idents)
                    }
                    HowToControlThis::Skip => {
                        #[cfg(feature = "debug_logging")]
                        log::info!("-> from_newtype_recurse_struct_vec");
                        Self::from_noop_struct(idents)
                    }
                }
            })
            .collect::<Vec<_>>();

        let idents = ParsedFieldIdent { name: name.clone() };

        Self::make_newtype_struct_final(idents, livecodable_fields)
    }
}

#[derive(Debug, FromField, Clone)]
#[darling(attributes(lerpable))]
pub(crate) struct LivecodeFieldReceiver {
    pub(crate) ident: Option<syn::Ident>,
    pub(crate) ty: syn::Type,
    pub(crate) method: Option<String>, // from this point on, start using String instead of the function we started with
}
impl LivecodeFieldReceiver {
    fn is_skip(&self) -> bool {
        self.method.as_deref().map_or(false, |x| x == "skip")
    }

    fn how_to_control_this(&self) -> HowToControlThis {
        if self.is_skip() {
            HowToControlThis::Skip
        } else {
            let type_idents = ident_from_type(&self.ty);
            HowToControlThis::from_type_str(type_idents.to_string().as_ref())
        }
    }
}

// for enums
#[derive(Debug, FromVariant, Clone)]
#[darling(attributes(lerpable))]
pub(crate) struct LivecodeVariantReceiver {
    pub(crate) ident: syn::Ident,
    pub(crate) fields: ast::Fields<LivecodeFieldReceiver>,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(lerpable), supports(any))]
pub(crate) struct LivecodeReceiver {
    ident: syn::Ident,
    data: ast::Data<LivecodeVariantReceiver, LivecodeFieldReceiver>,
}
impl LivecodeReceiver {}

// represents an enum
pub(crate) struct EnumIdents {
    pub(crate) enum_name: syn::Ident,
    pub(crate) data: LivecodeVariantReceiver,
}

impl EnumIdents {
    pub(crate) fn variant_ident(&self) -> syn::Ident {
        self.data.ident.clone()
    }

    pub(crate) fn enum_ident(&self) -> syn::Ident {
        self.enum_name.clone()
    }
}

#[derive(Clone, Debug)]
pub struct StructIdents {
    pub(crate) data: LivecodeFieldReceiver,
}
impl StructIdents {
    pub(crate) fn name(&self) -> syn::Ident {
        self.data.ident.clone().unwrap()
    }

    pub(crate) fn to_method_override(&self) -> TokenStream2 {
        if self.data.is_skip() {
            quote! {
                let method = pct;
            }
        } else {
            if let Some(method_str) = &self.data.method {
                let method: syn::Path =
                    syn::parse_str(method_str).expect("Custom method is invalid path!");
                quote! {
                    let method = &#method().with_lerp_pct(pct.lerp_pct());
                }
            } else {
                quote! {
                    let method = pct;
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum HowToControlThis {
    Skip,        // return the old object
    Vec,         // combine_vec
    LerpifyType, // structs, enums, f32, bool, etc, just going to call their lerpify functions
}
impl HowToControlThis {
    pub(crate) fn from_type_str(value: &str) -> HowToControlThis {
        match value {
            "Vec" => HowToControlThis::Vec,
            "String" => HowToControlThis::Skip,
            _ => HowToControlThis::LerpifyType,
        }
    }
}

pub fn recursive_ident_from_path(t: &syn::Type, acc: &mut Vec<syn::Ident>) {
    match t {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            let s = path.segments.last().unwrap();
            let main_type = s.ident.clone();

            acc.push(main_type);

            if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                args,
                ..
            }) = s.arguments.clone()
            {
                if let syn::GenericArgument::Type(other_ty) = args.first().unwrap() {
                    recursive_ident_from_path(other_ty, acc);
                } else {
                    panic!("recursive ident not implemented yet {:?}", args);
                }
            }
        }
        x => panic!("no name for type {:?}", x),
    }
}

pub(crate) fn ident_from_type(t: &syn::Type) -> syn::Ident {
    let mut acc = vec![];
    recursive_ident_from_path(t, &mut acc);

    // will always have at least one item
    acc[0].clone()
}
