#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
#[macro_use] extern crate quote;
use quote::ToTokens;

struct Visit;

impl Visit {
    fn emit_diagnostic(&self, span: proc_macro::Span, name: &'static str) {
        proc_macro::Diagnostic::spanned(
            span, proc_macro::Level::Error, format!(
                "A `{}` expression is not allowed within the body of a `#[parallel]` loop",
                name)
        ).emit();
    }
}

impl<'ast> syn::visit::Visit<'ast> for Visit {
    fn visit_expr_break(&mut self, expr: &'ast syn::ExprBreak) {
        self.emit_diagnostic(expr.break_token.span.unwrap(), "break");
    }

    fn visit_expr_continue(&mut self, expr: &'ast syn::ExprContinue) {
        self.emit_diagnostic(expr.continue_token.span.unwrap(), "continue");
    }

    fn visit_expr_return(&mut self, expr: &'ast syn::ExprReturn) {
        self.emit_diagnostic(expr.return_token.span.unwrap(), "return");
    }

    fn visit_expr_closure(&mut self, _: &'ast syn::ExprClosure) {
        // Do not inspect.
    }
}

/// Converts a for-loop to rayon parallel for_each.
///
/// Only valid on when applied to a for statement, with #![feature(stmt_expr_attributes)] present.
#[proc_macro_attribute]
pub fn parallel(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Return the input unchanged if it failed to parse. The compiler will show
    // the right diagnostics.
    let input: syn::ExprForLoop = syn::parse(input.clone())
        .expect("parallel attribute may only be applied to for loops.");

    // TODO support input.label
    // TODO support input.attrs
    let expr = input.expr.clone().into_token_stream();
    let body = input.body.clone().into_token_stream();
    let pat = input.pat.clone().into_token_stream();

    let mut visitor = Visit;
    syn::visit::visit_expr_for_loop(&mut visitor, &input);

    // TODO check for early returns in the for loop, they would be errors in the parallel version.
    let parallelized = quote!((#expr).into_par_iter().for_each(|#pat| #body));

    parallelized.to_string().parse().unwrap()
}
