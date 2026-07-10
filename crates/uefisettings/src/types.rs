// Backend interface types for uefisettings.
//
// Previously generated from thrift/uefisettings_backend.thrift via the Thrift
// compiler (fbthrift). Now plain Rust, merged into the main crate.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Question {
    pub name: String,
    pub answer: String,
    pub options: Vec<String>,
    pub help: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SetResponse {
    pub selector: String,
    pub backend: Backend,
    pub question: Question,
    pub modified: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SetResponseList {
    pub responses: Vec<SetResponse>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetResponse {
    pub selector: String,
    pub backend: Backend,
    pub question: Question,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GetResponseList {
    pub responses: Vec<GetResponse>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HiiShowIfrResponse {
    pub readable_representation: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HiiDatabase {
    pub db: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HiiStringsPackage {
    pub package_list: String,
    pub string_package: BTreeMap<i32, String>,
    pub language: String,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Backend {
    #[default]
    Unknown,
    Hii,
}
