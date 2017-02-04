use syn;

use attrs::FieldAttributes;

pub enum Field<'a> {
    Arg(Arg<'a>),
    Subcommand(Subcommand<'a>),
}

pub struct Arg<'a> {
    pub ident: &'a syn::Ident,
    pub name: &'a str,
    pub ty: &'a syn::Ty,
    pub short: Option<char>,
    pub long: Option<&'a str>,
    pub value_name: Option<&'a str>,
    pub index: Option<u64>,
    pub docs: String,
    pub takes_value: bool,
    pub is_counter: bool,
    pub multiple: bool,
    pub is_optional: bool,
    pub required: bool,
    pub default_value: Option<&'a str>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
}

pub struct Subcommand<'a> {
    pub ident: &'a syn::Ident,
    pub ty: &'a syn::Ty,
    pub is_optional: bool,
}

impl<'a> Field<'a> {
    pub fn arg(&self) -> Option<&Arg> {
        if let Field::Arg(ref arg) = *self {
            Some(arg)
        } else {
            None
        }
    }

    pub fn subcommand(&self) -> Option<&Subcommand> {
        if let Field::Subcommand(ref subcommand) = *self {
            Some(subcommand)
        } else {
            None
        }
    }
}

impl<'a> From<&'a syn::Field> for Field<'a> {
    fn from(field: &'a syn::Field) -> Field<'a> {
        let attrs: FieldAttributes = field.attrs.as_slice().into();
        if attrs.subcommand {
            Field::Subcommand(Subcommand::from(field))
        } else {
            Field::Arg(Arg::from((field, attrs)))
        }
    }
}

impl<'a> From<(&'a syn::Field, FieldAttributes<'a>)> for Arg<'a> {
    fn from((field, attrs): (&'a syn::Field, FieldAttributes<'a>)) -> Arg<'a> {
        let name = attrs.name.unwrap_or_else(|| field.ident.as_ref().unwrap().as_ref());

        // Unlike clap we default to a flag option unless there's a attribute given
        // telling us to not do so
        let is_flag = !attrs.index.is_some() && !attrs.arg;

        let long = attrs.long.or_else(|| if is_flag { Some(name) } else { None });

        let (is_bool, is_optional, is_vec, ty);
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                is_bool = path.segments[0].ident == "bool";
                is_optional = path.segments[0].ident == "Option";
                is_vec = path.segments[0].ident == "Vec";
                if is_optional || is_vec {
                    if let syn::PathParameters::AngleBracketed(ref params) =
                        path.segments[0].parameters {
                        ty = &params.types[0];
                    } else {
                        panic!();
                    }
                } else {
                    ty = &field.ty;
                }
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        Arg {
            ident: field.ident.as_ref().unwrap(),
            ty: ty,
            name: name,
            short: attrs.short,
            long: long,
            index: attrs.index,
            value_name: attrs.value_name,
            docs: attrs.docs.iter().map(|s| s.trim()).collect::<Vec<_>>().join("\n"),
            is_counter: attrs.counted,
            multiple: attrs.counted || is_vec,
            takes_value: !attrs.counted && !is_bool,
            is_optional: is_optional,
            required: !is_bool && !is_optional,
            default_value: attrs.default_value,
            min_values: attrs.min_values,
            max_values: attrs.max_values,
        }
    }
}

impl<'a> From<&'a syn::Field> for Subcommand<'a> {
    fn from(field: &'a syn::Field) -> Subcommand<'a> {
        let (is_optional, ty);
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                is_optional = path.segments[0].ident == "Option";
                if is_optional {
                    if let syn::PathParameters::AngleBracketed(ref params) =
                        path.segments[0].parameters {
                        ty = &params.types[0];
                    } else {
                        panic!();
                    }
                } else {
                    ty = &field.ty;
                }
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        Subcommand {
            ident: field.ident.as_ref().unwrap(),
            ty: ty,
            is_optional: is_optional,
        }
    }
}
