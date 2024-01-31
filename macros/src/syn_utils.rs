use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::{
    ext::IdentExt,
    visit::{visit_path, Visit},
    GenericParam, Generics, Ident, Result, Type,
};

macro_rules! bail {
    (_, $($arg:tt)*) => {
        bail!(::proc_macro2::Span::call_site(), $($arg)*)
    };
    ($span:expr, $fmt:literal $(,)?) => {
        return ::std::result::Result::Err(::syn::Error::new($span, ::std::format!($fmt)))
    };
    ($span:expr, $fmt:literal, $($arg:tt)*) => {
        return ::std::result::Result::Err(::syn::Error::new($span, ::std::format!($fmt, $($arg)*)))
    };
}

pub fn into_macro_output(input: Result<TokenStream>) -> proc_macro::TokenStream {
    match input {
        Ok(s) => s,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

pub struct GenericParamSet {
    idents: HashSet<Ident>,
}

impl GenericParamSet {
    pub fn new(generics: &Generics) -> Self {
        let mut idents = HashSet::new();
        for p in &generics.params {
            match p {
                GenericParam::Type(t) => {
                    idents.insert(t.ident.unraw());
                }
                GenericParam::Const(t) => {
                    idents.insert(t.ident.unraw());
                }
                _ => {}
            }
        }
        Self { idents }
    }
    fn contains(&self, ident: &Ident) -> bool {
        self.idents.contains(&ident.unraw())
    }

    pub fn contains_in_type(&self, ty: &Type) -> bool {
        struct Visitor<'a> {
            generics: &'a GenericParamSet,
            result: bool,
        }
        impl<'a, 'ast> Visit<'ast> for Visitor<'a> {
            fn visit_path(&mut self, i: &'ast syn::Path) {
                if i.leading_colon.is_none() {
                    if let Some(s) = i.segments.iter().next() {
                        if self.generics.contains(&s.ident) {
                            self.result = true;
                        }
                    }
                }
                visit_path(self, i);
            }
        }
        let mut visitor = Visitor {
            generics: self,
            result: false,
        };
        visitor.visit_type(ty);
        visitor.result
    }
}
