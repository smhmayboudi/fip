#[proc_macro_derive(Sub)]
pub fn derive_sub(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    // let input = proc_macro2::TokenStream::from(input);
    let ident = input.ident;
    let expanded = quote::quote! {
        impl Sub for #ident {}
    };
    proc_macro::TokenStream::from(expanded)
}

// #[proc_macro_attribute]
// pub fn attribute_show_streams(
//     attr: proc_macro::TokenStream,
//     item: proc_macro::TokenStream,
// ) -> proc_macro::TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

// #[proc_macro_derive(MyMacro)]
// pub fn derive_my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = syn::parse_macro_input!(input as syn::DeriveInput);

//     println!(" ==> input ==> {:#?}", input);

//     // Build the output, possibly using quasi-quotation
//     let expanded = quote::quote! {
//         // ...
//     };

//     println!(" ==> expanded ==> {:#?}", expanded);

//     // let input = proc_macro2::TokenStream::from(input);
//     // let expanded: proc_macro2::TokenStream = {
//     //     /* transform input */
//     // };

//     // Hand the output tokens back to the compiler
//     proc_macro::TokenStream::from(expanded)
// }
