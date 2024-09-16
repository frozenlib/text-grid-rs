use proc_macro2::TokenStream;
use quote::quote;
use structmeta::StructMeta;
use syn::{
    parse2, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Member, Result,
};

use crate::bound::WhereClauseBuilder;

#[derive(StructMeta)]
struct CellsAttr {
    dump: bool,
}

pub fn build(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveInput = parse2(input)?;
    let mut dump = false;
    for attr in &input.attrs {
        if attr.meta.path().is_ident("cells") {
            let attr: CellsAttr = attr.parse_args()?;
            dump |= attr.dump;
        }
    }
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
        impl #impl_g ::text_grid::Cells for #self_ident #self_g #wheres {
            fn fmt(f: &mut ::text_grid::CellsFormatter<Self>) {
                #code
            }
        }
    };
    if dump {
        panic!("dump :\n{code}");
    }
    Ok(code)
}
fn build_from_struct(data: &DataStruct, wcb: &mut WhereClauseBuilder) -> Result<TokenStream> {
    let mut codes = Vec::new();
    for (index, field) in data.fields.iter().enumerate() {
        if let Some(ident) = &field.ident {
            let ident_str = ident.to_string();
            codes.push(quote!(::text_grid::CellsFormatter::column(f, #ident_str, |x| &x.#ident)));
        } else {
            let m = Member::Unnamed(index.into());
            codes.push(quote!(::text_grid::CellsFormatter::content(f, |x| &x.#m)));
        };
        wcb.push_bounds_for_field(field);
    }
    Ok(quote!(#(#codes;)*))
}
fn build_from_enum(data: &DataEnum) -> Result<TokenStream> {
    let mut vars = Vec::new();
    for variant in &data.variants {
        let ident = &variant.ident;
        let ident_str = ident.to_string();
        if let Fields::Unit = &variant.fields {
            vars.push(quote!(Self::#ident => #ident_str))
        } else {
            bail!(variant.span(), "only unit variants supported");
        }
        let mut fields = Vec::new();
        for field in &variant.fields {
            let ty = &field.ty;
            fields.push(quote!(#ty));
        }
    }
    Ok(quote! {
        ::text_grid::CellsFormatter::content(f, |x| ::text_grid::cell(match x {
            #(#vars,)*
        }));
    })
}
