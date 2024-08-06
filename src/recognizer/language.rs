use std::fmt;
use std::fmt::{Display, Formatter};

/// Language that the recognizer should recognize.
///
/// The language is used to specify the language that the recognizer should recognize.
/// If the language is not mapped, you can use the `Custom` variant to specify the language.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Language {
    AfZa,
    AmEt,
    ArAe,
    ArBh,
    ArDz,
    ArEg,
    ArIq,
    ArJo,
    ArKw,
    ArLy,
    ArMa,
    ArQa,
    ArSa,
    ArSy,
    ArTn,
    ArYe,
    BgBg,
    BnBd,
    BnIn,
    CaEs,
    CsCz,
    CyGb,
    DaDk,
    DeAt,
    DeCh,
    DeDe,
    ElGr,
    EnAu,
    EnCa,
    #[default]
    EnGb,
    EnHk,
    EnIe,
    EnIn,
    EnKe,
    EnNg,
    EnNz,
    EnPh,
    EnSg,
    EnTz,
    EnUs,
    EnZa,
    EsAr,
    EsBo,
    EsCl,
    EsCo,
    EsCr,
    EsCu,
    EsDo,
    EsEc,
    EsEs,
    EsGq,
    EsGt,
    EsHn,
    EsMx,
    EsNi,
    EsPa,
    EsPe,
    EsPr,
    EsPy,
    EsSv,
    EsUs,
    EsUy,
    EsVe,
    EtEe,
    FaIr,
    FiFi,
    FilPh,
    FrBe,
    FrCa,
    FrCh,
    FrFr,
    GaIe,
    GlEs,
    GuIn,
    HeIl,
    HiIn,
    HrHr,
    HuHu,
    IdId,
    IsIs,
    ItIt,
    JaJp,
    JvId,
    KkKz,
    KmKh,
    KnIn,
    KoKr,
    LoLa,
    LtLt,
    LvLv,
    MkMk,
    MlIn,
    MrIn,
    MsMy,
    MtMt,
    MyMm,
    NbNo,
    NlBe,
    NlNl,
    PlPl,
    PsAf,
    PtBr,
    PtPt,
    RoRo,
    RuRu,
    SiLk,
    SkSk,
    SlSi,
    SoSo,
    SrRs,
    SuId,
    SvSe,
    SwKe,
    SwTz,
    TaIn,
    TaLk,
    TaSg,
    TeIn,
    ThTh,
    TrTr,
    UkUa,
    UrIn,
    UrPk,
    UzUz,
    ViVn,
    ZhCn,
    ZhHk,
    ZhTw,
    ZuZa,
    Custom(String),
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        
        let value = match self {
            Language::Custom(s) => s.as_str(),
            Language::AfZa => "af-ZA",
            Language::AmEt => "am-ET",
            Language::ArAe => "ar-AE",
            Language::ArBh => "ar-BH",
            Language::ArDz => "ar-DZ",
            Language::ArEg => "ar-EG",
            Language::ArIq => "ar-IQ",
            Language::ArJo => "ar-JO",
            Language::ArKw => "ar-KW",
            Language::ArLy => "ar-LY",
            Language::ArMa => "ar-MA",
            Language::ArQa => "ar-QA",
            Language::ArSa => "ar-SA",
            Language::ArSy => "ar-SY",
            Language::ArTn => "ar-TN",
            Language::ArYe => "ar-YE",
            Language::BgBg => "bg-BG",
            Language::BnBd => "bn-BD",
            Language::BnIn => "bn-IN",
            Language::CaEs => "ca-ES",
            Language::CsCz => "cs-CZ",
            Language::CyGb => "cy-GB",
            Language::DaDk => "da-DK",
            Language::DeAt => "de-AT",
            Language::DeCh => "de-CH",
            Language::DeDe => "de-DE",
            Language::ElGr => "el-GR",
            Language::EnAu => "en-AU",
            Language::EnCa => "en-CA",
            Language::EnGb => "en-GB",
            Language::EnHk => "en-HK",
            Language::EnIe => "en-IE",
            Language::EnIn => "en-IN",
            Language::EnKe => "en-KE",
            Language::EnNg => "en-NG",
            Language::EnNz => "en-NZ",
            Language::EnPh => "en-PH",
            Language::EnSg => "en-SG",
            Language::EnTz => "en-TZ",
            Language::EnUs => "en-US",
            Language::EnZa => "en-ZA",
            Language::EsAr => "es-AR",
            Language::EsBo => "es-BO",
            Language::EsCl => "es-CL",
            Language::EsCo => "es-CO",
            Language::EsCr => "es-CR",
            Language::EsCu => "es-CU",
            Language::EsDo => "es-DO",
            Language::EsEc => "es-EC",
            Language::EsEs => "es-ES",
            Language::EsGq => "es-GQ",
            Language::EsGt => "es-GT",
            Language::EsHn => "es-HN",
            Language::EsMx => "es-MX",
            Language::EsNi => "es-NI",
            Language::EsPa => "es-PA",
            Language::EsPe => "es-PE",
            Language::EsPr => "es-PR",
            Language::EsPy => "es-PY",
            Language::EsSv => "es-SV",
            Language::EsUs => "es-US",
            Language::EsUy => "es-UY",
            Language::EsVe => "es-VE",
            Language::EtEe => "et-EE",
            Language::FaIr => "fa-IR",
            Language::FiFi => "fi-FI",
            Language::FilPh => "fil-PH",
            Language::FrBe => "fr-BE",
            Language::FrCa => "fr-CA",
            Language::FrCh => "fr-CH",
            Language::FrFr => "fr-FR",
            Language::GaIe => "ga-IE",
            Language::GlEs => "gl-ES",
            Language::GuIn => "gu-IN",
            Language::HeIl => "he-IL",
            Language::HiIn => "hi-IN",
            Language::HrHr => "hr-HR",
            Language::HuHu => "hu-HU",
            Language::IdId => "id-ID",
            Language::IsIs => "is-IS",
            Language::ItIt => "it-IT",
            Language::JaJp => "ja-JP",
            Language::JvId => "jv-ID",
            Language::KkKz => "kk-KZ",
            Language::KmKh => "km-KH",
            Language::KnIn => "kn-IN",
            Language::KoKr => "ko-KR",
            Language::LoLa => "lo-LA",
            Language::LtLt => "lt-LT",
            Language::LvLv => "lv-LV",
            Language::MkMk => "mk-MK",
            Language::MlIn => "ml-IN",
            Language::MrIn => "mr-IN",
            Language::MsMy => "ms-MY",
            Language::MtMt => "mt-MT",
            Language::MyMm => "my-MM",
            Language::NbNo => "nb-NO",
            Language::NlBe => "nl-BE",
            Language::NlNl => "nl-NL",
            Language::PlPl => "pl-PL",
            Language::PsAf => "ps-AF",
            Language::PtBr => "pt-BR",
            Language::PtPt => "pt-PT",
            Language::RoRo => "ro-RO",
            Language::RuRu => "ru-RU",
            Language::SiLk => "si-LK",
            Language::SkSk => "sk-SK",
            Language::SlSi => "sl-SI",
            Language::SoSo => "so-SO",
            Language::SrRs => "sr-RS",
            Language::SuId => "su-ID",
            Language::SvSe => "sv-SE",
            Language::SwKe => "sw-KE",
            Language::SwTz => "sw-TZ",
            Language::TaIn => "ta-IN",
            Language::TaLk => "ta-LK",
            Language::TaSg => "ta-SG",
            Language::TeIn => "te-IN",
            Language::ThTh => "th-TH",
            Language::TrTr => "tr-TR",
            Language::UkUa => "uk-UA",
            Language::UrIn => "ur-IN",
            Language::UrPk => "ur-PK",
            Language::UzUz => "uz-UZ",
            Language::ViVn => "vi-VN",
            Language::ZhCn => "zh-CN",
            Language::ZhHk => "zh-HK",
            Language::ZhTw => "zh-TW",
            Language::ZuZa => "zu-ZA",
        };
        
        write!(f, "{}", value)
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "af-ZA" => Language::AfZa,
            "am-ET" => Language::AmEt,
            "ar-AE" => Language::ArAe,
            "ar-BH" => Language::ArBh,
            "ar-DZ" => Language::ArDz,
            "ar-EG" => Language::ArEg,
            "ar-IQ" => Language::ArIq,
            "ar-JO" => Language::ArJo,
            "ar-KW" => Language::ArKw,
            "ar-LY" => Language::ArLy,
            "ar-MA" => Language::ArMa,
            "ar-QA" => Language::ArQa,
            "ar-SA" => Language::ArSa,
            "ar-SY" => Language::ArSy,
            "ar-TN" => Language::ArTn,
            "ar-YE" => Language::ArYe,
            "bg-BG" => Language::BgBg,
            "bn-BD" => Language::BnBd,
            "bn-IN" => Language::BnIn,
            "ca-ES" => Language::CaEs,
            "cs-CZ" => Language::CsCz,
            "cy-GB" => Language::CyGb,
            "da-DK" => Language::DaDk,
            "de-AT" => Language::DeAt,
            "de-CH" => Language::DeCh,
            "de-DE" => Language::DeDe,
            "el-GR" => Language::ElGr,
            "en-AU" => Language::EnAu,
            "en-CA" => Language::EnCa,
            "en-GB" => Language::EnGb,
            "en-HK" => Language::EnHk,
            "en-IE" => Language::EnIe,
            "en-IN" => Language::EnIn,
            "en-KE" => Language::EnKe,
            "en-NG" => Language::EnNg,
            "en-NZ" => Language::EnNz,
            "en-PH" => Language::EnPh,
            "en-SG" => Language::EnSg,
            "en-TZ" => Language::EnTz,
            "en-US" => Language::EnUs,
            "en-ZA" => Language::EnZa,
            "es-AR" => Language::EsAr,
            "es-BO" => Language::EsBo,
            "es-CL" => Language::EsCl,
            "es-CO" => Language::EsCo,
            "es-CR" => Language::EsCr,
            "es-CU" => Language::EsCu,
            "es-DO" => Language::EsDo,
            "es-EC" => Language::EsEc,
            "es-ES" => Language::EsEs,
            "es-GQ" => Language::EsGq,
            "es-GT" => Language::EsGt,
            "es-HN" => Language::EsHn,
            "es-MX" => Language::EsMx,
            "es-NI" => Language::EsNi,
            "es-PA" => Language::EsPa,
            "es-PE" => Language::EsPe,
            "es-PR" => Language::EsPr,
            "es-PY" => Language::EsPy,
            "es-SV" => Language::EsSv,
            "es-US" => Language::EsUs,
            "es-UY" => Language::EsUy,
            "es-VE" => Language::EsVe,
            "et-EE" => Language::EtEe,
            "fa-IR" => Language::FaIr,
            "fi-FI" => Language::FiFi,
            "fil-PH" => Language::FilPh,
            "fr-BE" => Language::FrBe,
            "fr-CA" => Language::FrCa,
            "fr-CH" => Language::FrCh,
            "fr-FR" => Language::FrFr,
            "ga-IE" => Language::GaIe,
            "gl-ES" => Language::GlEs,
            "gu-IN" => Language::GuIn,
            "he-IL" => Language::HeIl,
            "hi-IN" => Language::HiIn,
            "hr-HR" => Language::HrHr,
            "hu-HU" => Language::HuHu,
            "id-ID" => Language::IdId,
            "is-IS" => Language::IsIs,
            "it-IT" => Language::ItIt,
            "ja-JP" => Language::JaJp,
            "jv-ID" => Language::JvId,
            "kk-KZ" => Language::KkKz,
            "km-KH" => Language::KmKh,
            "kn-IN" => Language::KnIn,
            "ko-KR" => Language::KoKr,
            "lo-LA" => Language::LoLa,
            "lt-LT" => Language::LtLt,
            "lv-LV" => Language::LvLv,
            "mk-MK" => Language::MkMk,
            "ml-IN" => Language::MlIn,
            "mr-IN" => Language::MrIn,
            "ms-MY" => Language::MsMy,
            "mt-MT" => Language::MtMt,
            "my-MM" => Language::MyMm,
            "nb-NO" => Language::NbNo,
            "nl-BE" => Language::NlBe,
            "nl-NL" => Language::NlNl,
            "pl-PL" => Language::PlPl,
            "ps-AF" => Language::PsAf,
            "pt-BR" => Language::PtBr,
            "pt-PT" => Language::PtPt,
            "ro-RO" => Language::RoRo,
            "ru-RU" => Language::RuRu,
            "si-LK" => Language::SiLk,
            "sk-SK" => Language::SkSk,
            "sl-SI" => Language::SlSi,
            "so-SO" => Language::SoSo,
            "sr-RS" => Language::SrRs,
            "su-ID" => Language::SuId,
            "sv-SE" => Language::SvSe,
            "sw-KE" => Language::SwKe,
            "sw-TZ" => Language::SwTz,
            "ta-IN" => Language::TaIn,
            "ta-LK" => Language::TaLk,
            "ta-SG" => Language::TaSg,
            "te-IN" => Language::TeIn,
            "th-TH" => Language::ThTh,
            "tr-TR" => Language::TrTr,
            "uk-UA" => Language::UkUa,
            "ur-IN" => Language::UrIn,
            "ur-PK" => Language::UrPk,
            "uz-UZ" => Language::UzUz,
            "vi-VN" => Language::ViVn,
            "zh-CN" => Language::ZhCn,
            "zh-HK" => Language::ZhHk,
            "zh-TW" => Language::ZhTw,
            "zu-ZA" => Language::ZuZa,
            _ => Language::Custom(value),
        }
    }
}
