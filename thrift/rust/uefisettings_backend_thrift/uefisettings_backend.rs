// Backend interface type definitions for uefisettings.
//
// Previously generated from thrift/uefisettings_backend.thrift via the Thrift
// compiler (fbthrift). Now hand-maintained plain Rust so the crate carries no
// fbthrift dependency.

pub use self::types::*;

pub mod types {
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct MachineInfo {
        pub backend: std::collections::BTreeSet<crate::types::Backend>,
        pub bios_vendor: String,
        pub bios_version: String,
        pub bios_release: String,
        pub bios_date: String,
        pub product_name: String,
        pub product_family: String,
        pub product_version: String,
    }

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
        pub backend: crate::types::Backend,
        pub question: crate::types::Question,
        pub modified: bool,
        pub is_translated: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct SetResponseList {
        pub responses: Vec<crate::types::SetResponse>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct GetResponse {
        pub selector: String,
        pub backend: crate::types::Backend,
        pub question: crate::types::Question,
        pub is_translated: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct GetResponseList {
        pub responses: Vec<crate::types::GetResponse>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Error {
        pub error_message: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct IloAttributes {
        pub selector: String,
        pub attributes: std::collections::BTreeMap<String, String>,
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
        pub string_package: std::collections::BTreeMap<i32, String>,
        pub language: String,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
    pub enum Backend {
        #[default]
        Unknown,
        Hii,
        Ilo,
    }
}
