macro_rules! attrs {
    ($name:ident($scope:ident) { $($arg_name:ident: $arg_type:tt,)* }) => {
        attrs!(fields $name [] $($arg_name $arg_type,)*);
        attrs!(into foo $name($scope) [] [] $($arg_name $arg_type,)*);
    };

    (fields $name:ident [$($p:tt)*] $field:ident bool, $($r:tt)*) => {
        attrs!(fields $name [
            $($p)*
            pub $field: bool,
        ] $($r)*);
    };

    (fields $name:ident [$($p:tt)*] $field:ident str, $($r:tt)*) => {
        attrs!(fields $name [
            $($p)*
            pub $field: Option<&'a str>,
        ] $($r)*);
    };

    (fields $name:ident [$($p:tt)*] $field:ident [str], $($r:tt)*) => {
        attrs!(fields $name [
            $($p)*
            pub $field: Vec<&'a str>,
        ] $($r)*);
    };

    (fields $name:ident [$($p:tt)*] $field:ident $ty:ty, $($r:tt)*) => {
        attrs!(fields $name [
            $($p)*
            pub $field: Option<$ty>,
        ] $($r)*);
    };

    (fields $name:ident [$($p:tt)*]) => {
        pub struct $name<'a> {
            $($p)*
            pub docs: String,
            pub summary: String,
        }
    };

    (into $name:ident $st:ident($scope:ident) [$($i:tt)*] [$($m:tt)*] $field:ident bool, $($r:tt)*) => {
        attrs!(into $name $st($scope) [
            $($i)*
            $field: false,
        ] [
            $($m)*
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == stringify!($field) => {
                    $name.$field = match *value {
                        ::syn::Lit::Bool(value) => value,
                        ::syn::Lit::Str(ref value, _) => {
                            value.parse().unwrap_or_else(|err| {
                                panic!(
                                    "Parsing attribute value {:?} for {}({}) failed: {}",
                                    value, stringify!($scope), ident.as_ref(), err)
                            })
                        }
                        _ => {
                            panic!(
                                "Unexpected attribute literal value {:?} for {}({}), expected {}",
                                value, stringify!($scope), ident.as_ref(), "bool")
                        }
                    }
                }
            ::syn::MetaItem::Word(ref ident)
                if ident.as_ref() == stringify!($field) => {
                    $name.$field = true;
                }
        ]
        $($r)*);
    };

    (into $name:ident $st:ident($scope:ident) [$($i:tt)*] [$($m:tt)*] $field:ident str, $($r:tt)*) => {
        attrs!(into $name $st($scope) [
            $($i)*
            $field: None,
        ] [
            $($m)*
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == stringify!($field) => {
                    $name.$field = None;
                }
        ]
        $($r)*);
    };

    (into $name:ident $st:ident($scope:ident) [$($i:tt)*] [$($m:tt)*] $field:ident [str], $($r:tt)*) => {
        attrs!(into $name $st($scope) [
            $($i)*
            $field: Vec::new(),
        ] [
            $($m)*
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == stringify!($field) => {
                }
        ]
        $($r)*);
    };

    (into $name:ident $st:ident($scope:ident) [$($i:tt)*] [$($m:tt)*] $field:ident $ty:ty, $($r:tt)*) => {
        attrs!(into $name $st($scope) [
            $($i)*
            $field: None,
        ] [
            $($m)*
            ::syn::MetaItem::NameValue(ref ident, ref value)
                if ident.as_ref() == stringify!($field) => {
                    $name.$field = None;
                }
        ]
        $($r)*);
    };

    (into $name:ident $st:ident($scope:ident) [$($i:tt)*] [$($m:tt)*]) => {
        impl<'a> From<&'a [::syn::Attribute]> for $st<'a> {
            fn from(attrs: &[::syn::Attribute]) -> $st {
                let docs = attrs.iter()
                    .filter(|a| a.is_sugared_doc)
                    .map(|a| match a.value {
                        ::syn::MetaItem::NameValue(_, ::syn::Lit::Str(ref doc, _)) => doc,
                        _ => unreachable!(),
                    })
                    .fold(String::new(), |docs, line| docs + line.trim_left_matches('/').trim() + "\n");

                let index = docs.find("\n\n");
                let (summary, docs) = if let Some(index) = index {
                    let (summary, docs) = docs.split_at(index);
                    let (_, docs) = docs.split_at(2);
                    (summary.into(), docs.into())
                } else {
                    (docs, "".into())
                };

                let mut $name = $st {
                    $($i)*
                    docs: docs,
                    summary: summary,
                };
                for attr in attrs {
                    if let ::syn::MetaItem::List(ref ident, ref values) = attr.value {
                        if ident == stringify!($scope) {
                            for value in values {
                                if let ::syn::NestedMetaItem::MetaItem(ref item) = *value {
                                    match *item {
                                        $($m)*
                                        ref item => {
                                            panic!("Unexpected attribute {:?}", item);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                $name
            }
        }
    };
}

attrs! {
    StructAttributes(clap) {
        name: str,
        crate_version: bool,
        version: str,
        crate_authors: bool,
        author: str,
        alias: str,
        global_settings: [str],
    }
}

attrs! {
    FieldAttributes(clap) {
        name: str,
        index: u64,
        arg: bool,
        long: str,
        short: char,
        counted: bool,
        default_value: str,
        min_values: u64,
        max_values: u64,
        value_name: str,
        subcommand: bool,
    }
}

