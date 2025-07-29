use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use structmeta::StructMeta;
use syn::{
    parse::Parse, parse2, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Expr, Field,
    Fields, Member, Result, Variant,
};

use crate::bound::WhereClauseBuilder;

#[derive(StructMeta, Default)]
struct CellsAttr {
    dump: bool,
}

#[derive(StructMeta, Default)]
struct CellsAttrForField {
    header: Option<Expr>,
}

fn parse_attrs<T: Parse + Default>(name: &str, attrs: &[syn::Attribute]) -> Result<T> {
    for attr in attrs {
        if attr.path().is_ident(name) {
            return attr.parse_args();
        }
    }
    Ok(Default::default())
}

pub fn build(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    let attr = parse_attrs::<CellsAttr>("cells", &input.attrs)?;
    let (impl_g, self_g, _) = input.generics.split_for_impl();
    let mut wcb = WhereClauseBuilder::new(&input.generics);
    let code = match &input.data {
        Data::Struct(data) => build_from_struct(data, &mut wcb)?,
        Data::Enum(data) => build_from_enum(data)?,
        Data::Union(_) => bail!(input.span(), "`#[derive(Cells)] not supported for unions"),
    };
    let self_ident = &input.ident;
    let wheres = wcb.build(|ty| quote!(#ty : ::text_grid::Cells));
    let code = quote! {
        #[automatically_derived]
        impl #impl_g ::text_grid::Cells for #self_ident #self_g #wheres {
            #[allow(unused_variables)]
            fn fmt(f: &mut ::text_grid::CellsFormatter<Self>) {
                #code
            }
        }
    };
    if attr.dump {
        panic!("dump :\n{code}");
    }
    Ok(code)
}
fn build_from_struct(data: &DataStruct, wcb: &mut WhereClauseBuilder) -> Result<TokenStream> {
    let mut codes = Vec::new();
    for (index, field) in data.fields.iter().enumerate() {
        let attr = parse_attrs::<CellsAttrForField>("cells", &field.attrs)?;
        let header = if let Some(header) = &attr.header {
            Some(quote!(#header))
        } else if let Some(ident) = &field.ident {
            let ident_str = ident.to_string();
            Some(quote!(#ident_str))
        } else {
            None
        };
        let content = if let Some(ident) = &field.ident {
            quote!(|x| &x.#ident)
        } else {
            let m = Member::Unnamed(index.into());
            quote!(|x| &x.#m)
        };
        if let Some(header) = header {
            codes.push(quote!(::text_grid::CellsFormatter::column(f, #header, #content)));
        } else {
            codes.push(quote!(::text_grid::CellsFormatter::content(f, #content)));
        }
        wcb.push_bounds_for_field(field);
    }
    Ok(quote!(#(#codes;)*))
}
fn build_from_enum(data: &DataEnum) -> Result<TokenStream> {
    let mut key_to_offset = HashMap::new();

    let mut name_count = 0;
    let mut unnamed_count = 0;
    for variant in &data.variants {
        for (index, field) in variant.fields.iter().enumerate() {
            key_to_offset
                .entry(FieldKey::new(index, field))
                .or_insert_with_key(|key| {
                    let offset;
                    match key {
                        FieldKey::Unnamed(_) => {
                            offset = index;
                            unnamed_count += 1;
                        }
                        FieldKey::Named(_) => {
                            offset = name_count;
                            name_count += 1;
                        }
                    }
                    offset
                });
        }
    }
    for (key, offset) in &mut key_to_offset {
        if key.is_named() {
            *offset += unnamed_count;
        }
    }

    let mut name_arms = Vec::new();
    let mut columns = vec![Column::dummy(); key_to_offset.len()];
    for (key, offset) in &key_to_offset {
        columns[*offset] = Column::new(key);
    }
    for variant in &data.variants {
        let mut exprs = vec![quote!(None); columns.len()];
        for (index, field) in variant.fields.iter().enumerate() {
            let offset = key_to_offset[&FieldKey::new(index, field)];
            let var = to_var(index, field);
            exprs[offset] = quote!(Some(#var));
        }
        let pat = to_pat(variant);
        for (column, expr) in columns.iter_mut().zip(exprs) {
            column.arms.push(quote!(Self::#pat => #expr));
        }
        let ident_str = variant.ident.to_string();
        name_arms.push(quote!(Self::#pat => #ident_str));
    }
    Ok(quote! {
        ::text_grid::CellsFormatter::content(f, |x| match x {
            #(#name_arms,)*
        });
        #(#columns;)*
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FieldKey {
    Unnamed(usize),
    Named(String),
}
impl FieldKey {
    fn new(index: usize, field: &Field) -> Self {
        if let Some(ident) = &field.ident {
            FieldKey::Named(ident.to_string())
        } else {
            FieldKey::Unnamed(index)
        }
    }
    fn to_header(&self) -> String {
        match self {
            FieldKey::Unnamed(index) => index.to_string(),
            FieldKey::Named(name) => name.clone(),
        }
    }
    fn is_named(&self) -> bool {
        matches!(self, FieldKey::Named(_))
    }
}

#[derive(Clone)]
struct Column {
    key: FieldKey,
    arms: Vec<TokenStream>,
}
impl Column {
    fn dummy() -> Self {
        Self {
            key: FieldKey::Unnamed(0),
            arms: Vec::new(),
        }
    }
    fn new(key: &FieldKey) -> Self {
        Self {
            key: key.clone(),
            arms: Vec::new(),
        }
    }
}
impl ToTokens for Column {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arms = &self.arms;
        let header = self.key.to_header();
        tokens.extend(quote!(
            ::text_grid::CellsFormatter::column(f, #header, |x| match x {
                #(#arms,)*
            });
        ))
    }
}

fn to_pat(variant: &Variant) -> TokenStream {
    let ident = &variant.ident;
    let vars = variant
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| to_var(index, field));
    let ctor_args = match &variant.fields {
        Fields::Unit => quote!(),
        Fields::Named(_) => quote!({#(#vars,)*}),
        Fields::Unnamed(_) => quote!((#(#vars,)*)),
    };
    quote!(#ident #ctor_args)
}

fn to_var(index: usize, field: &Field) -> TokenStream {
    if let Some(ident) = &field.ident {
        quote!(#ident)
    } else {
        let mut ident = format_ident!("f{index}");
        ident.set_span(field.span());
        ident.to_token_stream()
    }
}
