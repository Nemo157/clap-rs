extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod attrs;
mod field;
mod define_app;
mod from_arg_matches;
mod define_sub_commands;
mod sub_command_from_arg_matches;

#[proc_macro_derive(App, attributes(clap))]
pub fn app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = define_app::expand(&ast);
    expanded.parse().unwrap()
}

#[proc_macro_derive(FromArgMatches, attributes(clap))]
pub fn from_arg_matches(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let expanded = from_arg_matches::expand(&ast);
    expanded.parse().unwrap()
}

#[proc_macro_derive(SubCommands, attributes(clap))]
pub fn subcommands(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();
    let subcommands = define_sub_commands::expand(&ast);
    let from_arg_matches = sub_command_from_arg_matches::expand(&ast);
    quote!(#subcommands #from_arg_matches).to_string().parse().unwrap()
}
