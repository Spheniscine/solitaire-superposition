use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parse, parse::ParseStream, Expr, Lit, Token};

struct MathInput {
    latex: String,
    display_mode: bool,
}

impl Parse for MathInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the LaTeX string
        let expr: Expr = input.parse()?;
        
        let latex = match &expr {
            Expr::Lit(syn::ExprLit { lit: Lit::Str(s), .. }) => s.value(),
            _ => return Err(syn::Error::new_spanned(expr, "expected string literal")),
        };
        
        // Optional: parse display_mode flag
        let display_mode = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let mode: syn::Expr = input.parse()?;
            match &mode {
                Expr::Lit(syn::ExprLit { lit: Lit::Bool(b), .. }) => b.value,
                _ => false,
            }
        } else {
            false
        };
        
        Ok(MathInput { latex, display_mode })
    }
}

#[proc_macro]
pub fn math(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as MathInput);
    
    let opts = katex::Opts::builder()
        .display_mode(parsed.display_mode)
        .output_type(katex::OutputType::Html)
        .throw_on_error(false)
        .build()
        .unwrap();
    
    let html = match katex::render_with_opts(&parsed.latex, &opts) {
        Ok(h) => h,
        Err(e) => {
            let msg = format!("KaTeX render error: {}", e);
            return syn::Error::new(Span::call_site(), msg).to_compile_error().into();
        }
    };
    
    let expanded = quote! {
        #html
    };
    
    TokenStream::from(expanded)
}

// For display mode (centered/block)
#[proc_macro]
pub fn math_display(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as MathInput);
    
    let opts = katex::Opts::builder()
        .display_mode(true)
        .output_type(katex::OutputType::Html)
        .throw_on_error(false)
        .build()
        .unwrap();
    
    let html = match katex::render_with_opts(&parsed.latex, &opts) {
        Ok(h) => h,
        Err(e) => {
            let msg = format!("KaTeX render error: {}", e);
            return syn::Error::new(Span::call_site(), msg).to_compile_error().into();
        }
    };
    
    let expanded = quote! {
        #html
    };
    
    TokenStream::from(expanded)
}