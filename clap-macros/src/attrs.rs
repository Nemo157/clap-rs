#[derive(PromAttire)]
#[attire(scope = "clap", docs = "docs")]
pub struct StructAttributes<'a> {
    pub name: Option<&'a str>,
    pub crate_version: bool,
    pub version: Option<&'a str>,
    pub crate_authors: bool,
    pub author: Option<&'a str>,
    pub alias: Option<&'a str>,
    pub global_settings: Vec<&'a str>,
    pub docs: Vec<&'a str>,
}

#[derive(PromAttire)]
#[attire(scope = "clap", docs = "docs")]
pub struct FieldAttributes<'a> {
    pub name: Option<&'a str>,
    pub index: Option<u64>,
    pub arg: bool,
    pub long: Option<&'a str>,
    pub short: Option<char>,
    pub counted: bool,
    pub default_value: Option<&'a str>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
    pub value_name: Option<&'a str>,
    pub subcommand: bool,
    pub docs: Vec<&'a str>,
}
