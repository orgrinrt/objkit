//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

// This crate is not `no_std` and cannot be: a procedural macro runs inside the compiler on
// the host, and syn, quote and proc-macro2 all use std. It used to carry
// `#![cfg_attr(feature = "no_std", no_std)]`, which took `format!`, `Vec` and `ToString`
// away from its own source, so `objkit` built with `no_std` did not compile at all.
//
// What the feature actually selects is which `Box` the *generated* code names, which is
// `box_path` below and has nothing to do with this crate's own prelude.

use include_proc_macro::macros;
use quote::quote;
use syn::__private::TokenStream2;
use syn::parse_quote;

mod downcast;
mod obj_send_sync;
mod obj_visitor;
mod proxy;

fn auxiliary_trait_name(trait_name: Option<&syn::Ident>, postfix: &str) -> syn::Ident {
    let ident_string = match trait_name {
        Some(ident) => format!("{}{}", ident, postfix),
        None => postfix.to_string(),
    };
    syn::Ident::new(
        &ident_string,
        trait_name
            .map(|ident| ident.span())
            .unwrap_or_else(proc_macro2::Span::call_site),
    )
}

#[inline]
fn box_path() -> TokenStream2 {
    #[cfg(all(not(feature = "std_box"), feature = "alloc_box"))]
    quote! {
        ::alloc::boxed::Box
    }
    #[cfg(feature = "std_box")]
    quote! {
        ::std::boxed::Box
    }
    #[cfg(all(feature = "std_box", feature = "alloc_box"))]
    compile_error!(
        "objkit's `std` and `no_std` features are exclusive, and both are on. `std` is the \
         default, so selecting `no_std` also needs `default-features = false`. (Reported \
         here as `std_box` and `alloc_box`, which is what objkit forwards them to.)"
    );
    #[cfg(all(not(feature = "std_box"), not(feature = "alloc_box")))]
    compile_error!(
        "objkit needs one of its two features and has neither. `std` is the default; with \
         `default-features = false`, add `features = [\"no_std\"]`. (Reported here as \
         `std_box` and `alloc_box`, which is what objkit forwards them to.)"
    );
}

fn merged_generics(
    input_generics: &syn::Generics,
    internal_generics: &syn::Generics,
) -> syn::Generics {
    fn param_names_match(a: &syn::GenericParam, b: &syn::GenericParam) -> bool {
        match (a, b) {
            (syn::GenericParam::Type(a), syn::GenericParam::Type(b)) => a.ident == b.ident,
            (syn::GenericParam::Lifetime(a), syn::GenericParam::Lifetime(b)) => {
                a.lifetime.ident == b.lifetime.ident
            },
            (syn::GenericParam::Const(a), syn::GenericParam::Const(b)) => a.ident == b.ident,
            _ => false,
        }
    }

    let mut merged_generics = input_generics.clone();

    for param in &internal_generics.params {
        if !input_generics
            .params
            .iter()
            .any(|p| param_names_match(p, param))
        {
            merged_generics.params.push(param.clone());
        }
    }

    merged_generics
}

fn appended_generics(generics: &syn::Generics, new_param: &syn::Ident) -> syn::Generics {
    merged_generics(generics, &parse_quote!(<#new_param>))
}

/// Refuses a trait carrying an associated type, for the macros that must form `dyn Trait`.
///
/// `dyn T` is not a type when `T` has an unspecified associated type, and a macro cannot
/// know which concrete type the caller meant. Left alone, the generated code fails with
/// `E0191` spanned inside an expansion the caller cannot read. `as_any` is unaffected,
/// because the only trait object it forms is `dyn Any`.
fn refuse_associated_types(
    original_trait: &syn::ItemTrait,
    macro_name: &str,
) -> Option<TokenStream2> {
    let associated: Vec<&syn::TraitItemType> = original_trait
        .items
        .iter()
        .filter_map(|item| match item {
            syn::TraitItem::Type(ty) => Some(ty),
            _ => None,
        })
        .collect();

    let first = associated.first()?;
    let name = &first.ident;
    let trait_name = &original_trait.ident;
    let message = format!(
        "`#[{macro_name}]` needs to name `dyn {trait_name}`, and a trait with an \
         associated type has no such type until the associated type is given a value. \
         Take `{name}` off `{trait_name}` and make it a generic parameter, which \
         `#[{macro_name}]` does support, or use `#[as_any]`, which forms only `dyn Any`."
    );
    Some(quote! { ::core::compile_error!(#message); })
}

/// The annotated trait's own where clause, with one more predicate on the end.
///
/// The generated impls each need a bound of their own, and every one of them used to write
/// that bound as the whole `where` clause, which dropped whatever the annotated trait had
/// declared. A trait written `trait T<A> where A: Clone` then produced impls that did not
/// know `A: Clone`.
fn aux_where_clause(
    generics: &syn::Generics,
    extra: syn::WherePredicate,
) -> syn::WhereClause {
    let mut clause = generics
        .where_clause
        .clone()
        .unwrap_or_else(|| parse_quote!(where));
    clause.predicates.push(extra);
    clause
}

macros! {
    attribute -> clone_box::clone_box,
    attribute -> as_any::as_any,
    attribute -> as_super::as_super,
    attribute -> obj_eq::obj_eq,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> syn::ItemTrait {
        syn::parse_str(source).expect("the test's own trait parses")
    }

    #[test]
    fn a_trait_without_an_associated_type_is_not_refused() {
        for source in [
            "trait T {}",
            "trait T { fn f(&self); }",
            "trait T<A> { fn f(&self) -> A; }",
            "trait T<A: Clone> where A: Send { const N: usize; fn f(&self) -> A; }",
        ] {
            assert!(
                refuse_associated_types(&parse(source), "clone_box").is_none(),
                "{source} carries no associated type"
            );
        }
    }

    #[test]
    fn a_trait_with_an_associated_type_is_refused() {
        let refusal = refuse_associated_types(&parse("trait T { type A; }"), "clone_box")
            .expect("an associated type is refused");
        // A token stream renders with spaces around punctuation, so the path is compared
        // with the whitespace taken out rather than as it prints.
        let rendered: String = refusal.to_string().split_whitespace().collect();
        assert!(
            rendered.starts_with("::core::compile_error!"),
            "the refusal is a compile_error, not something the caller has to notice: {rendered}"
        );
    }

    #[test]
    fn the_refusal_names_the_macro_the_trait_and_the_associated_type() {
        let refusal =
            refuse_associated_types(&parse("trait Carrier { type Item; }"), "as_super")
                .expect("an associated type is refused")
                .to_string();

        for expected in ["as_super", "Carrier", "Item", "as_any"] {
            assert!(
                refusal.contains(expected),
                "the diagnostic names {expected}, so the caller can act on it: {refusal}"
            );
        }
    }

    #[test]
    fn a_const_or_a_method_is_not_mistaken_for_an_associated_type() {
        // `TraitItem` has four kinds and only one of them is the refused one.
        let source = "trait T { const N: usize; fn f(&self); type A; }";
        let refusal = refuse_associated_types(&parse(source), "obj_eq")
            .expect("the associated type is found among the other items");
        assert!(refusal.to_string().contains('A'));
    }

    #[test]
    fn the_appended_generic_does_not_duplicate_one_the_trait_already_has() {
        let generics: syn::Generics = syn::parse_str("<A, B>").unwrap();
        let appended = appended_generics(&generics, &quote::format_ident!("A"));
        assert_eq!(
            appended.params.len(),
            2,
            "an `A` already present is not added a second time"
        );

        let appended = appended_generics(&generics, &quote::format_ident!("__U__"));
        assert_eq!(appended.params.len(), 3);
    }

    #[test]
    fn the_aux_where_clause_keeps_what_the_trait_declared() {
        let generics: syn::Generics = syn::parse_str("<A>").unwrap();
        let mut with_where = generics.clone();
        with_where.where_clause = Some(syn::parse_quote!(where A: Clone));

        let clause = aux_where_clause(&with_where, syn::parse_quote!(__U__: Send));
        let rendered = quote! { #clause }.to_string();
        assert!(rendered.contains("A : Clone"), "{rendered}");
        assert!(rendered.contains("__U__ : Send"), "{rendered}");

        // And a trait with no where clause of its own still gets one.
        let clause = aux_where_clause(&generics, syn::parse_quote!(__U__: Send));
        let rendered = quote! { #clause }.to_string();
        assert!(rendered.contains("__U__ : Send"), "{rendered}");
        assert!(!rendered.contains("Clone"), "{rendered}");
    }
}
