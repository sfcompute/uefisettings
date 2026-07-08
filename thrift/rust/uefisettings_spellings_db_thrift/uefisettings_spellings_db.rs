// Spellings database type and constant definitions for uefisettings.
//
// Previously generated from thrift/uefisettings_spellings_db.thrift via the
// Thrift compiler (fbthrift). Now hand-maintained plain Rust so the crate
// carries no fbthrift dependency.

pub use self::consts::*;
pub use self::types::*;

pub mod types {
    use std::collections::BTreeMap;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct QuestionMapping {
        pub hii_question: Option<crate::types::HiiQuestion>,
        pub ilo_question: Option<crate::types::IloQuestion>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct HiiQuestion {
        pub question_variations: Vec<String>,
        pub answer_replacements: Option<BTreeMap<String, Vec<String>>>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct IloQuestion {
        pub question: String,
        pub answer_replacements: Option<BTreeMap<String, String>>,
    }
}

pub mod consts {
    use std::collections::BTreeMap;

    use once_cell::sync::Lazy;

    use crate::types::HiiQuestion;
    use crate::types::IloQuestion;
    use crate::types::QuestionMapping;

    pub const CQ_TPM_STATE: &str = "TPM State";
    pub const CQ_SECURITY_DEVICE_SUPPORT: &str = "Security Device Support";
    pub const CQ_HYPER_THREADING: &str = "Hyper Threading";
    pub const CQ_TXT_SUPPORT: &str = "TXT Support";
    pub const CQ_VT_D: &str = "VTd";

    fn enable_disable_replacements() -> BTreeMap<String, Vec<String>> {
        BTreeMap::from([
            (
                "Enabled".to_owned(),
                vec!["Enabled".to_owned(), "Enable".to_owned()],
            ),
            (
                "Disabled".to_owned(),
                vec!["Disabled".to_owned(), "Disable".to_owned()],
            ),
        ])
    }

    #[allow(non_upper_case_globals)]
    pub static translation_db: Lazy<BTreeMap<String, QuestionMapping>> = Lazy::new(|| {
        BTreeMap::from([
            (
                CQ_TPM_STATE.to_owned(),
                QuestionMapping {
                    hii_question: Some(HiiQuestion {
                        question_variations: vec!["TPM State".to_owned()],
                        answer_replacements: Some(enable_disable_replacements()),
                    }),
                    ilo_question: Some(IloQuestion {
                        question: "TpmState".to_owned(),
                        answer_replacements: Some(BTreeMap::from([(
                            "Enabled".to_owned(),
                            "PresentEnabled".to_owned(),
                        )])),
                    }),
                },
            ),
            (
                CQ_SECURITY_DEVICE_SUPPORT.to_owned(),
                QuestionMapping {
                    hii_question: Some(HiiQuestion {
                        question_variations: vec!["Security Device Support".to_owned()],
                        answer_replacements: Some(enable_disable_replacements()),
                    }),
                    ilo_question: None,
                },
            ),
            (
                CQ_TXT_SUPPORT.to_owned(),
                QuestionMapping {
                    hii_question: Some(HiiQuestion {
                        question_variations: vec![
                            "TXT Support".to_owned(),
                            "Enable Intel(R) TXT".to_owned(),
                        ],
                        answer_replacements: Some(enable_disable_replacements()),
                    }),
                    ilo_question: Some(IloQuestion {
                        question: "IntelTxt".to_owned(),
                        answer_replacements: None,
                    }),
                },
            ),
            (
                CQ_HYPER_THREADING.to_owned(),
                QuestionMapping {
                    hii_question: Some(HiiQuestion {
                        question_variations: vec![
                            "Hyper-Threading".to_owned(),
                            "Enable LP".to_owned(),
                            "Hyper-Threading [ALL]".to_owned(),
                        ],
                        answer_replacements: Some(enable_disable_replacements()),
                    }),
                    ilo_question: None,
                },
            ),
            (
                CQ_VT_D.to_owned(),
                QuestionMapping {
                    hii_question: Some(HiiQuestion {
                        question_variations: vec![
                            "(VT-d)".to_owned(),
                            "VT for Directed I/O".to_owned(),
                            "Intel® VT for Directed I/O (VT-d)".to_owned(),
                        ],
                        answer_replacements: Some(enable_disable_replacements()),
                    }),
                    ilo_question: None,
                },
            ),
        ])
    });
}
