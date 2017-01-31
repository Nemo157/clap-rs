use syn;
use quote;

use attrs::StructAttributes;
use field::{ Arg, Field, Subcommand };

fn expand_arg(arg: &Arg) -> quote::Tokens {
    let name = arg.name;
    let ty = arg.ty;
    let short = arg.short.as_ref().map(|s| quote! { .short(#s) });
    let long = arg.long.map(|s| quote! { .long(#s) });
    let value_name = arg.value_name.map(|s| quote! { .value_name(#s) });
    let takes_value = arg.takes_value;
    let index = arg.index.map(|i| quote! { .index(#i) });
    let docs = (arg.summary.clone() + "\n\n" + &arg.docs).trim().to_string();
    let multiple = arg.multiple;
    let default_value = arg.default_value.map(|d| quote! { .default_value(#d) });
    let min_values = arg.min_values.map(|m| quote! { .min_values(#m) });
    let max_values = arg.max_values.map(|m| quote! { .max_values(#m) });
    let required = arg.required;
    let validator = if arg.takes_value {
        Some(quote! {
            .validator(|s| {
                <#ty as ::std::str::FromStr>::from_str(&s)
                    .map(|_| ())
                    .map_err(|e| format!("failed to parse value {:?} for argument '{}': {}", s, #name, e))
            })
        })
    } else {
        None
    };

    quote! {
        ::clap::Arg::with_name(#name)
            #short
            #long
            #value_name
            #index
            .help(#docs)
            .takes_value(#takes_value)
            .multiple(#multiple)
            #default_value
            #min_values
            #max_values
            .required(#required)
            #validator
    }
}

fn expand_args<'a, 'b: 'a, I>(args: I) -> quote::Tokens where I: Iterator<Item=&'a Arg<'b>> {
    let args = args.map(expand_arg);
    quote! { .args(&[#(#args),*]) }
}

fn expand_subcommand(subcommand: &Subcommand) -> quote::Tokens {
    let ty = subcommand.ty;
    let required = if subcommand.is_optional {
        None
    } else {
        Some(quote! { .setting(::clap::AppSettings::SubcommandRequiredElseHelp) })
    };

    quote! {
        .subcommands(<#ty as ::clap::code_gen::SubCommands>::subcommands())
        #required
    }
}

fn expand_app(ast: &syn::MacroInput, fields: &[Field]) -> quote::Tokens {
    let attrs = StructAttributes::from(ast.attrs.as_slice());
    let name = attrs.name.map(syn::Lit::from).unwrap_or_else(|| syn::Lit::from(ast.ident.as_ref().to_lowercase()));

    let version = if attrs.crate_version {
        Some(quote! { .version(crate_version!()) })
    } else {
        attrs.version.map(|a| quote! { .version(#a) })
    };

    let author = if attrs.crate_authors {
        Some(quote! { .author(crate_authors!()) })
    } else {
        attrs.author.map(|a| quote! { .author(#a) })
    };

    let args = expand_args(fields.iter().filter_map(|field| field.arg()));
    let subcommand = fields.iter()
        .filter_map(|field| field.subcommand())
        .find(|_| true)
        .map(expand_subcommand);

    let summary = attrs.summary;
    let docs = attrs.docs;
    let alias = attrs.alias.map(|a| quote! { .alias(#a) });
    let settings = attrs.global_settings.iter().cloned().map(syn::Ident::from);
    let global_settings = quote! {
        .global_settings(&[#(::clap::AppSettings::#settings),*])
    };

    quote! {
        ::clap::App::new(#name)
            #version
            #author
            #args
            #subcommand
            .about(#summary)
            .after_help(#docs)
            #alias
            #global_settings
    }
}

pub fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Unit) => {
            Vec::new()
        }
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields.iter().map(Field::from).collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("#[derive(DefineApp)] is not supported on tuple structs")
        }
        syn::Body::Enum(_) => {
            panic!("#[derive(DefineApp)] is not supported on enums")
        }
    };

    let ident = &ast.ident;
    let app = expand_app(ast, &fields);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics ::clap::code_gen::App for #ident #ty_generics #where_clause {
            fn app() -> ::clap::App<'static, 'static> {
                #app
            }
        }
    }
}
