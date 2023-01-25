use phf::{phf_map, Map};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;

#[cfg(test)]
mod tests {
    #[test]
    fn test_from_code() {
        let l = crate::from_code("EUR");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_numeric() {
        let l = crate::from_numeric(360);
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_from_numeric_str() {
        let l = crate::from_numeric_str("643");
        print!("test_from_code result {:?}", l)
    }

    #[test]
    fn test_all() {
        println!("{:?}", crate::ALL);
        println!("{:?}", crate::ALL_MAP);
    }
}
//https://unicode.org/iso15924/iso15924-codes.html
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode {
    //English Name
    name: &'static str,
    //code
    code: &'static str,
    //ISO number
    numeric: i32,
    //Minor unit
    unit: i32,
    //type
    code_type: &'static str,
    //countries
    countries: &'static [&'static str],
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode {
    //English Name
    pub name: &'static str,
    //code
    pub code: &'static str,
    //ISO number
    pub numeric: i32,
    //Minor unit
    pub unit: i32,
    //type
    pub code_type: &'static str,
    pub countries: &'static [&'static str],
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CurrencyCode {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

    #[wasm_bindgen(getter)]
    pub fn numeric(&self) -> i32 {
        self.numeric
    }

    #[wasm_bindgen(getter)]
    pub fn unit(&self) -> i32 {
        self.unit
    }
    #[wasm_bindgen(getter)]
    pub fn code_type(&self) -> String {
        self.code_type.into()
    }
    #[wasm_bindgen(getter)]
    pub fn countries(&self) -> Array {
        let mut vector: Vec<&'static str> = Vec::new();
        for i in 0..self.countries.len() {
            vector.push(self.countries[i])
        }
        vector.into_iter().map(JsValue::from).collect()
    }
}

/// Returns the CurrencyCode with the given Alpha4 code, if exists.
/// #Sample
/// ```
/// let currency = rust_iso4217::from_code("COU");
/// assert_eq!(970, currency.unwrap().numeric);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_code(alpha3: &str) -> Option<CurrencyCode> {
    let up = alpha3.to_uppercase();
    ALL_MAP.get(&up).cloned()
}
/// Returns the CurrencyCode with the given numeric , if exists.
// #Sample
/// ```
/// let currency = rust_iso4217::from_numeric(116);
/// assert_eq!("KHR", currency.unwrap().code);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric(numeric: i32) -> Option<CurrencyCode> {
    let k = format!("{:03}", numeric);
    NUMERIC_MAP.get(&k).cloned()
}

/// Returns the CurrencyCode with the given numeric 3 length str, if exists.
// #Sample
/// ```
/// let currency = rust_iso4217::from_numeric_str("840");
/// assert_eq!("USD", currency.unwrap().code);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric_str(numeric: &str) -> Option<CurrencyCode> {
    NUMERIC_MAP.get(numeric).cloned()
}

/// Returns the CurrencyCode with the given numeric 3 length str, if exists.
// #Sample
/// ```
/// let currencies = rust_iso4217::from_country("CHN");
/// assert_eq!(currencies.len() > 0,true)
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn from_country(country: &str) -> Vec<CurrencyCode> {
    let mut vector: Vec<CurrencyCode> = Vec::new();
    let currencies = COUNTRY_MAP.get(country);
    match currencies {
        Some(cs) => {
            for i in 0..cs.len() {
                vector.push(cs[i].clone())
            }
        }
        None => todo!(),
    }
    vector
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn from_country(country: &str) -> Array {
    let mut vector: Vec<CurrencyCode> = Vec::new();
    let currencies = COUNTRY_MAP.get(country);
    match currencies {
        Some(cs) => {
            for i in 0..cs.len() {
                vector.push(cs[i].clone())
            }
        }
        None => todo!(),
    }
    vector.into_iter().map(JsValue::from).collect()
}

pub const AFN: CurrencyCode = CurrencyCode {
    name: "Afghani",
    code: "AFN",
    numeric: 971,
    unit: 2,
    code_type: "currency",
    countries: &["AFG"],
};

pub const EUR: CurrencyCode = CurrencyCode {
    name: "Euro",
    code: "EUR",
    numeric: 978,
    unit: 2,
    code_type: "currency",
    countries: &[
        "AND", "AUT", "BEL", "CYP", "EST", "FIN", "FRA", "GUF", "ATF", "DEU", "GRC", "GLP", "VAT",
        "IRL", "ITA", "LVA", "LTU", "LUX", "MLT", "MTQ", "MYT", "MCO", "MNE", "NLD", "PRT", "REU",
        "BLM", "MAF", "SPM", "SMR", "SVK", "SVN", "ESP", "ALA",
    ],
};

pub const ALL: CurrencyCode = CurrencyCode {
    name: "Lek",
    code: "ALL",
    numeric: 008,
    unit: 2,
    code_type: "currency",
    countries: &["ALB"],
};

pub const DZD: CurrencyCode = CurrencyCode {
    name: "Algerian Dinar",
    code: "DZD",
    numeric: 012,
    unit: 2,
    code_type: "currency",
    countries: &["DZA"],
};

pub const USD: CurrencyCode = CurrencyCode {
    name: "US Dollar",
    code: "USD",
    numeric: 840,
    unit: 2,
    code_type: "currency",
    countries: &[
        "ASM", "BES", "IOT", "VGB", "ECU", "GUM", "MHL", "FSM", "MNP", "PLW", "PRI", "TLS", "TCA",
        "UMI", "VIR", "USA",
    ],
};

pub const AOA: CurrencyCode = CurrencyCode {
    name: "Kwanza",
    code: "AOA",
    numeric: 973,
    unit: 2,
    code_type: "currency",
    countries: &["AGO"],
};

pub const XCD: CurrencyCode = CurrencyCode {
    name: "East Caribbean Dollar",
    code: "XCD",
    numeric: 951,
    unit: 2,
    code_type: "currency",
    countries: &["AIA", "ATG", "DMA", "GRD", "MSR", "KNA", "LCA", "VCT"],
};

pub const ARS: CurrencyCode = CurrencyCode {
    name: "Argentine Peso",
    code: "ARS",
    numeric: 032,
    unit: 2,
    code_type: "currency",
    countries: &["ARG"],
};

pub const AMD: CurrencyCode = CurrencyCode {
    name: "Armenian Dram",
    code: "AMD",
    numeric: 051,
    unit: 2,
    code_type: "currency",
    countries: &["ARM"],
};

pub const AWG: CurrencyCode = CurrencyCode {
    name: "Aruban Florin",
    code: "AWG",
    numeric: 533,
    unit: 2,
    code_type: "currency",
    countries: &["ABW"],
};

pub const AUD: CurrencyCode = CurrencyCode {
    name: "Australian Dollar",
    code: "AUD",
    numeric: 036,
    unit: 2,
    code_type: "currency",
    countries: &["AUS", "CXR", "CCK", "HMD", "KIR", "NRU", "NFK", "TUV"],
};

pub const AZN: CurrencyCode = CurrencyCode {
    name: "Azerbaijan Manat",
    code: "AZN",
    numeric: 944,
    unit: 2,
    code_type: "currency",
    countries: &["AZE"],
};

pub const BSD: CurrencyCode = CurrencyCode {
    name: "Bahamian Dollar",
    code: "BSD",
    numeric: 044,
    unit: 2,
    code_type: "currency",
    countries: &["BHS"],
};

pub const BHD: CurrencyCode = CurrencyCode {
    name: "Bahraini Dinar",
    code: "BHD",
    numeric: 048,
    unit: 3,
    code_type: "currency",
    countries: &["BHR"],
};

pub const BDT: CurrencyCode = CurrencyCode {
    name: "Taka",
    code: "BDT",
    numeric: 050,
    unit: 2,
    code_type: "currency",
    countries: &["BGD"],
};

pub const BBD: CurrencyCode = CurrencyCode {
    name: "Barbados Dollar",
    code: "BBD",
    numeric: 052,
    unit: 2,
    code_type: "currency",
    countries: &["BRB"],
};

pub const BYN: CurrencyCode = CurrencyCode {
    name: "Belarusian Ruble",
    code: "BYN",
    numeric: 933,
    unit: 2,
    code_type: "currency",
    countries: &["BLR"],
};

pub const BZD: CurrencyCode = CurrencyCode {
    name: "Belize Dollar",
    code: "BZD",
    numeric: 084,
    unit: 2,
    code_type: "currency",
    countries: &["BLZ"],
};

pub const XOF: CurrencyCode = CurrencyCode {
    name: "CFA Franc BCEAO",
    code: "XOF",
    numeric: 952,
    unit: 0,
    code_type: "currency",
    countries: &["BEN", "BFA", "CIV", "GNB", "MLI", "NER", "SEN", "TGO"],
};

pub const BMD: CurrencyCode = CurrencyCode {
    name: "Bermudian Dollar",
    code: "BMD",
    numeric: 060,
    unit: 2,
    code_type: "currency",
    countries: &["BMU"],
};

pub const INR: CurrencyCode = CurrencyCode {
    name: "Indian Rupee",
    code: "INR",
    numeric: 356,
    unit: 2,
    code_type: "currency",
    countries: &["IND"],
};

pub const BTN: CurrencyCode = CurrencyCode {
    name: "Ngultrum",
    code: "BTN",
    numeric: 064,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const BOB: CurrencyCode = CurrencyCode {
    name: "Boliviano",
    code: "BOB",
    numeric: 068,
    unit: 2,
    code_type: "currency",
    countries: &["BOL"],
};

pub const BOV: CurrencyCode = CurrencyCode {
    name: "Mvdol",
    code: "BOV",
    numeric: 984,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const BAM: CurrencyCode = CurrencyCode {
    name: "Convertible Mark",
    code: "BAM",
    numeric: 977,
    unit: 2,
    code_type: "currency",
    countries: &["BIH"],
};

pub const BWP: CurrencyCode = CurrencyCode {
    name: "Pula",
    code: "BWP",
    numeric: 072,
    unit: 2,
    code_type: "currency",
    countries: &["BWA"],
};

pub const NOK: CurrencyCode = CurrencyCode {
    name: "Norwegian Krone",
    code: "NOK",
    numeric: 578,
    unit: 2,
    code_type: "currency",
    countries: &["BVT", "NOR", "SJM"],
};

pub const BRL: CurrencyCode = CurrencyCode {
    name: "Brazilian Real",
    code: "BRL",
    numeric: 986,
    unit: 2,
    code_type: "currency",
    countries: &["BRA"],
};

pub const BND: CurrencyCode = CurrencyCode {
    name: "Brunei Dollar",
    code: "BND",
    numeric: 096,
    unit: 2,
    code_type: "currency",
    countries: &["BRN"],
};

pub const BGN: CurrencyCode = CurrencyCode {
    name: "Bulgarian Lev",
    code: "BGN",
    numeric: 975,
    unit: 2,
    code_type: "currency",
    countries: &["BGR"],
};

pub const BIF: CurrencyCode = CurrencyCode {
    name: "Burundi Franc",
    code: "BIF",
    numeric: 108,
    unit: 0,
    code_type: "currency",
    countries: &["BDI"],
};

pub const CVE: CurrencyCode = CurrencyCode {
    name: "Cabo Verde Escudo",
    code: "CVE",
    numeric: 132,
    unit: 2,
    code_type: "currency",
    countries: &["CPV"],
};

pub const KHR: CurrencyCode = CurrencyCode {
    name: "Riel",
    code: "KHR",
    numeric: 116,
    unit: 2,
    code_type: "currency",
    countries: &["KHM"],
};

pub const XAF: CurrencyCode = CurrencyCode {
    name: "CFA Franc BEAC",
    code: "XAF",
    numeric: 950,
    unit: 0,
    code_type: "currency",
    countries: &["CMR", "CAF", "TCD", "COG", "GNQ", "GAB"],
};

pub const CAD: CurrencyCode = CurrencyCode {
    name: "Canadian Dollar",
    code: "CAD",
    numeric: 124,
    unit: 2,
    code_type: "currency",
    countries: &["CAN"],
};

pub const KYD: CurrencyCode = CurrencyCode {
    name: "Cayman Islands Dollar",
    code: "KYD",
    numeric: 136,
    unit: 2,
    code_type: "currency",
    countries: &["CYM"],
};

pub const CLP: CurrencyCode = CurrencyCode {
    name: "Chilean Peso",
    code: "CLP",
    numeric: 152,
    unit: 0,
    code_type: "currency",
    countries: &["CHL"],
};

pub const CLF: CurrencyCode = CurrencyCode {
    name: "Unidad de Fomento",
    code: "CLF",
    numeric: 990,
    unit: 4,
    code_type: "currency",
    countries: &[],
};

pub const CNY: CurrencyCode = CurrencyCode {
    name: "Yuan Renminbi",
    code: "CNY",
    numeric: 156,
    unit: 2,
    code_type: "currency",
    countries: &["CHN"],
};

pub const COP: CurrencyCode = CurrencyCode {
    name: "Colombian Peso",
    code: "COP",
    numeric: 170,
    unit: 2,
    code_type: "currency",
    countries: &["COL"],
};

pub const COU: CurrencyCode = CurrencyCode {
    name: "Unidad de Valor Real",
    code: "COU",
    numeric: 970,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const KMF: CurrencyCode = CurrencyCode {
    name: "Comorian Franc ",
    code: "KMF",
    numeric: 174,
    unit: 0,
    code_type: "currency",
    countries: &["COM"],
};

pub const CDF: CurrencyCode = CurrencyCode {
    name: "Congolese Franc",
    code: "CDF",
    numeric: 976,
    unit: 2,
    code_type: "currency",
    countries: &["COD"],
};

pub const NZD: CurrencyCode = CurrencyCode {
    name: "New Zealand Dollar",
    code: "NZD",
    numeric: 554,
    unit: 2,
    code_type: "currency",
    countries: &["COK", "NZL", "NIU", "PCN", "TKL"],
};

pub const CRC: CurrencyCode = CurrencyCode {
    name: "Costa Rican Colon",
    code: "CRC",
    numeric: 188,
    unit: 2,
    code_type: "currency",
    countries: &["CRI"],
};

pub const CUP: CurrencyCode = CurrencyCode {
    name: "Cuban Peso",
    code: "CUP",
    numeric: 192,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const CUC: CurrencyCode = CurrencyCode {
    name: "Peso Convertible",
    code: "CUC",
    numeric: 931,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const ANG: CurrencyCode = CurrencyCode {
    name: "Netherlands Antillean Guilder",
    code: "ANG",
    numeric: 532,
    unit: 2,
    code_type: "currency",
    countries: &["CUW", "SXM"],
};

pub const CZK: CurrencyCode = CurrencyCode {
    name: "Czech Koruna",
    code: "CZK",
    numeric: 203,
    unit: 2,
    code_type: "currency",
    countries: &["CZE"],
};

pub const DKK: CurrencyCode = CurrencyCode {
    name: "Danish Krone",
    code: "DKK",
    numeric: 208,
    unit: 2,
    code_type: "currency",
    countries: &["DNK", "FRO", "GRL"],
};

pub const DJF: CurrencyCode = CurrencyCode {
    name: "Djibouti Franc",
    code: "DJF",
    numeric: 262,
    unit: 0,
    code_type: "currency",
    countries: &["DJI"],
};

pub const DOP: CurrencyCode = CurrencyCode {
    name: "Dominican Peso",
    code: "DOP",
    numeric: 214,
    unit: 2,
    code_type: "currency",
    countries: &["DOM"],
};

pub const EGP: CurrencyCode = CurrencyCode {
    name: "Egyptian Pound",
    code: "EGP",
    numeric: 818,
    unit: 2,
    code_type: "currency",
    countries: &["EGY"],
};

pub const SVC: CurrencyCode = CurrencyCode {
    name: "El Salvador Colon",
    code: "SVC",
    numeric: 222,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const ERN: CurrencyCode = CurrencyCode {
    name: "Nakfa",
    code: "ERN",
    numeric: 232,
    unit: 2,
    code_type: "currency",
    countries: &["ERI"],
};

pub const SZL: CurrencyCode = CurrencyCode {
    name: "Lilangeni",
    code: "SZL",
    numeric: 748,
    unit: 2,
    code_type: "currency",
    countries: &["SWZ"],
};

pub const ETB: CurrencyCode = CurrencyCode {
    name: "Ethiopian Birr",
    code: "ETB",
    numeric: 230,
    unit: 2,
    code_type: "currency",
    countries: &["ETH"],
};

pub const FKP: CurrencyCode = CurrencyCode {
    name: "Falkland Islands Pound",
    code: "FKP",
    numeric: 238,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const FJD: CurrencyCode = CurrencyCode {
    name: "Fiji Dollar",
    code: "FJD",
    numeric: 242,
    unit: 2,
    code_type: "currency",
    countries: &["FJI"],
};

pub const XPF: CurrencyCode = CurrencyCode {
    name: "CFP Franc",
    code: "XPF",
    numeric: 953,
    unit: 0,
    code_type: "currency",
    countries: &["PYF", "NCL", "WLF"],
};

pub const GMD: CurrencyCode = CurrencyCode {
    name: "Dalasi",
    code: "GMD",
    numeric: 270,
    unit: 2,
    code_type: "currency",
    countries: &["GMB"],
};

pub const GEL: CurrencyCode = CurrencyCode {
    name: "Lari",
    code: "GEL",
    numeric: 981,
    unit: 2,
    code_type: "currency",
    countries: &["GEO"],
};

pub const GHS: CurrencyCode = CurrencyCode {
    name: "Ghana Cedi",
    code: "GHS",
    numeric: 936,
    unit: 2,
    code_type: "currency",
    countries: &["GHA"],
};

pub const GIP: CurrencyCode = CurrencyCode {
    name: "Gibraltar Pound",
    code: "GIP",
    numeric: 292,
    unit: 2,
    code_type: "currency",
    countries: &["GIB"],
};

pub const GTQ: CurrencyCode = CurrencyCode {
    name: "Quetzal",
    code: "GTQ",
    numeric: 320,
    unit: 2,
    code_type: "currency",
    countries: &["GTM"],
};

pub const GBP: CurrencyCode = CurrencyCode {
    name: "Pound Sterling",
    code: "GBP",
    numeric: 826,
    unit: 2,
    code_type: "currency",
    countries: &["GGY", "IMN", "JEY", "GBR"],
};

pub const GNF: CurrencyCode = CurrencyCode {
    name: "Guinean Franc",
    code: "GNF",
    numeric: 324,
    unit: 0,
    code_type: "currency",
    countries: &["GIN"],
};

pub const GYD: CurrencyCode = CurrencyCode {
    name: "Guyana Dollar",
    code: "GYD",
    numeric: 328,
    unit: 2,
    code_type: "currency",
    countries: &["GUY"],
};

pub const HTG: CurrencyCode = CurrencyCode {
    name: "Gourde",
    code: "HTG",
    numeric: 332,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const HNL: CurrencyCode = CurrencyCode {
    name: "Lempira",
    code: "HNL",
    numeric: 340,
    unit: 2,
    code_type: "currency",
    countries: &["HND"],
};

pub const HKD: CurrencyCode = CurrencyCode {
    name: "Hong Kong Dollar",
    code: "HKD",
    numeric: 344,
    unit: 2,
    code_type: "currency",
    countries: &["HKG"],
};

pub const HUF: CurrencyCode = CurrencyCode {
    name: "Forint",
    code: "HUF",
    numeric: 348,
    unit: 2,
    code_type: "currency",
    countries: &["HUN"],
};

pub const ISK: CurrencyCode = CurrencyCode {
    name: "Iceland Krona",
    code: "ISK",
    numeric: 352,
    unit: 0,
    code_type: "currency",
    countries: &["ISL"],
};

pub const IDR: CurrencyCode = CurrencyCode {
    name: "Rupiah",
    code: "IDR",
    numeric: 360,
    unit: 2,
    code_type: "currency",
    countries: &["IDN"],
};

pub const XDR: CurrencyCode = CurrencyCode {
    name: "SDR (Special Drawing Right)",
    code: "XDR",
    numeric: 960,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const IRR: CurrencyCode = CurrencyCode {
    name: "Iranian Rial",
    code: "IRR",
    numeric: 364,
    unit: 2,
    code_type: "currency",
    countries: &["IRN"],
};

pub const IQD: CurrencyCode = CurrencyCode {
    name: "Iraqi Dinar",
    code: "IQD",
    numeric: 368,
    unit: 3,
    code_type: "currency",
    countries: &["IRQ"],
};

pub const ILS: CurrencyCode = CurrencyCode {
    name: "New Israeli Sheqel",
    code: "ILS",
    numeric: 376,
    unit: 2,
    code_type: "currency",
    countries: &["ISR"],
};

pub const JMD: CurrencyCode = CurrencyCode {
    name: "Jamaican Dollar",
    code: "JMD",
    numeric: 388,
    unit: 2,
    code_type: "currency",
    countries: &["JAM"],
};

pub const JPY: CurrencyCode = CurrencyCode {
    name: "Yen",
    code: "JPY",
    numeric: 392,
    unit: 0,
    code_type: "currency",
    countries: &["JPN"],
};

pub const JOD: CurrencyCode = CurrencyCode {
    name: "Jordanian Dinar",
    code: "JOD",
    numeric: 400,
    unit: 3,
    code_type: "currency",
    countries: &["JOR"],
};

pub const KZT: CurrencyCode = CurrencyCode {
    name: "Tenge",
    code: "KZT",
    numeric: 398,
    unit: 2,
    code_type: "currency",
    countries: &["KAZ"],
};

pub const KES: CurrencyCode = CurrencyCode {
    name: "Kenyan Shilling",
    code: "KES",
    numeric: 404,
    unit: 2,
    code_type: "currency",
    countries: &["KEN"],
};

pub const KPW: CurrencyCode = CurrencyCode {
    name: "North Korean Won",
    code: "KPW",
    numeric: 408,
    unit: 2,
    code_type: "currency",
    countries: &["PRK"],
};

pub const KRW: CurrencyCode = CurrencyCode {
    name: "Won",
    code: "KRW",
    numeric: 410,
    unit: 0,
    code_type: "currency",
    countries: &["KOR"],
};

pub const KWD: CurrencyCode = CurrencyCode {
    name: "Kuwaiti Dinar",
    code: "KWD",
    numeric: 414,
    unit: 3,
    code_type: "currency",
    countries: &["KWT"],
};

pub const KGS: CurrencyCode = CurrencyCode {
    name: "Som",
    code: "KGS",
    numeric: 417,
    unit: 2,
    code_type: "currency",
    countries: &["KGZ"],
};

pub const LAK: CurrencyCode = CurrencyCode {
    name: "Lao Kip",
    code: "LAK",
    numeric: 418,
    unit: 2,
    code_type: "currency",
    countries: &["LAO"],
};

pub const LBP: CurrencyCode = CurrencyCode {
    name: "Lebanese Pound",
    code: "LBP",
    numeric: 422,
    unit: 2,
    code_type: "currency",
    countries: &["LBN"],
};

pub const LSL: CurrencyCode = CurrencyCode {
    name: "Loti",
    code: "LSL",
    numeric: 426,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const ZAR: CurrencyCode = CurrencyCode {
    name: "Rand",
    code: "ZAR",
    numeric: 710,
    unit: 2,
    code_type: "currency",
    countries: &["ZAF"],
};

pub const LRD: CurrencyCode = CurrencyCode {
    name: "Liberian Dollar",
    code: "LRD",
    numeric: 430,
    unit: 2,
    code_type: "currency",
    countries: &["LBR"],
};

pub const LYD: CurrencyCode = CurrencyCode {
    name: "Libyan Dinar",
    code: "LYD",
    numeric: 434,
    unit: 3,
    code_type: "currency",
    countries: &["LBY"],
};

pub const CHF: CurrencyCode = CurrencyCode {
    name: "Swiss Franc",
    code: "CHF",
    numeric: 756,
    unit: 2,
    code_type: "currency",
    countries: &["LIE", "CHE"],
};

pub const MOP: CurrencyCode = CurrencyCode {
    name: "Pataca",
    code: "MOP",
    numeric: 446,
    unit: 2,
    code_type: "currency",
    countries: &["MAC"],
};

pub const MKD: CurrencyCode = CurrencyCode {
    name: "Denar",
    code: "MKD",
    numeric: 807,
    unit: 2,
    code_type: "currency",
    countries: &["MKD"],
};

pub const MGA: CurrencyCode = CurrencyCode {
    name: "Malagasy Ariary",
    code: "MGA",
    numeric: 969,
    unit: 2,
    code_type: "currency",
    countries: &["MDG"],
};

pub const MWK: CurrencyCode = CurrencyCode {
    name: "Malawi Kwacha",
    code: "MWK",
    numeric: 454,
    unit: 2,
    code_type: "currency",
    countries: &["MWI"],
};

pub const MYR: CurrencyCode = CurrencyCode {
    name: "Malaysian Ringgit",
    code: "MYR",
    numeric: 458,
    unit: 2,
    code_type: "currency",
    countries: &["MYS"],
};

pub const MVR: CurrencyCode = CurrencyCode {
    name: "Rufiyaa",
    code: "MVR",
    numeric: 462,
    unit: 2,
    code_type: "currency",
    countries: &["MDV"],
};

pub const MRU: CurrencyCode = CurrencyCode {
    name: "Ouguiya",
    code: "MRU",
    numeric: 929,
    unit: 2,
    code_type: "currency",
    countries: &["MRT"],
};

pub const MUR: CurrencyCode = CurrencyCode {
    name: "Mauritius Rupee",
    code: "MUR",
    numeric: 480,
    unit: 2,
    code_type: "currency",
    countries: &["MUS"],
};

pub const XUA: CurrencyCode = CurrencyCode {
    name: "ADB Unit of Account",
    code: "XUA",
    numeric: 965,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const MXN: CurrencyCode = CurrencyCode {
    name: "Mexican Peso",
    code: "MXN",
    numeric: 484,
    unit: 2,
    code_type: "currency",
    countries: &["MEX"],
};

pub const MXV: CurrencyCode = CurrencyCode {
    name: "Mexican Unidad de Inversion (UDI)",
    code: "MXV",
    numeric: 979,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const MDL: CurrencyCode = CurrencyCode {
    name: "Moldovan Leu",
    code: "MDL",
    numeric: 498,
    unit: 2,
    code_type: "currency",
    countries: &["MDA"],
};

pub const MNT: CurrencyCode = CurrencyCode {
    name: "Tugrik",
    code: "MNT",
    numeric: 496,
    unit: 2,
    code_type: "currency",
    countries: &["MNG"],
};

pub const MAD: CurrencyCode = CurrencyCode {
    name: "Moroccan Dirham",
    code: "MAD",
    numeric: 504,
    unit: 2,
    code_type: "currency",
    countries: &["MAR", "ESH"],
};

pub const MZN: CurrencyCode = CurrencyCode {
    name: "Mozambique Metical",
    code: "MZN",
    numeric: 943,
    unit: 2,
    code_type: "currency",
    countries: &["MOZ"],
};

pub const MMK: CurrencyCode = CurrencyCode {
    name: "Kyat",
    code: "MMK",
    numeric: 104,
    unit: 2,
    code_type: "currency",
    countries: &["MMR"],
};

pub const NAD: CurrencyCode = CurrencyCode {
    name: "Namibia Dollar",
    code: "NAD",
    numeric: 516,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const NPR: CurrencyCode = CurrencyCode {
    name: "Nepalese Rupee",
    code: "NPR",
    numeric: 524,
    unit: 2,
    code_type: "currency",
    countries: &["NPL"],
};

pub const NIO: CurrencyCode = CurrencyCode {
    name: "Cordoba Oro",
    code: "NIO",
    numeric: 558,
    unit: 2,
    code_type: "currency",
    countries: &["NIC"],
};

pub const NGN: CurrencyCode = CurrencyCode {
    name: "Naira",
    code: "NGN",
    numeric: 566,
    unit: 2,
    code_type: "currency",
    countries: &["NGA"],
};

pub const OMR: CurrencyCode = CurrencyCode {
    name: "Rial Omani",
    code: "OMR",
    numeric: 512,
    unit: 3,
    code_type: "currency",
    countries: &["OMN"],
};

pub const PKR: CurrencyCode = CurrencyCode {
    name: "Pakistan Rupee",
    code: "PKR",
    numeric: 586,
    unit: 2,
    code_type: "currency",
    countries: &["PAK"],
};

pub const PAB: CurrencyCode = CurrencyCode {
    name: "Balboa",
    code: "PAB",
    numeric: 590,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const PGK: CurrencyCode = CurrencyCode {
    name: "Kina",
    code: "PGK",
    numeric: 598,
    unit: 2,
    code_type: "currency",
    countries: &["PNG"],
};

pub const PYG: CurrencyCode = CurrencyCode {
    name: "Guarani",
    code: "PYG",
    numeric: 600,
    unit: 0,
    code_type: "currency",
    countries: &["PRY"],
};

pub const PEN: CurrencyCode = CurrencyCode {
    name: "Sol",
    code: "PEN",
    numeric: 604,
    unit: 2,
    code_type: "currency",
    countries: &["PER"],
};

pub const PHP: CurrencyCode = CurrencyCode {
    name: "Philippine Peso",
    code: "PHP",
    numeric: 608,
    unit: 2,
    code_type: "currency",
    countries: &["PHL"],
};

pub const PLN: CurrencyCode = CurrencyCode {
    name: "Zloty",
    code: "PLN",
    numeric: 985,
    unit: 2,
    code_type: "currency",
    countries: &["POL"],
};

pub const QAR: CurrencyCode = CurrencyCode {
    name: "Qatari Rial",
    code: "QAR",
    numeric: 634,
    unit: 2,
    code_type: "currency",
    countries: &["QAT"],
};

pub const RON: CurrencyCode = CurrencyCode {
    name: "Romanian Leu",
    code: "RON",
    numeric: 946,
    unit: 2,
    code_type: "currency",
    countries: &["ROU"],
};

pub const RUB: CurrencyCode = CurrencyCode {
    name: "Russian Ruble",
    code: "RUB",
    numeric: 643,
    unit: 2,
    code_type: "currency",
    countries: &["RUS"],
};

pub const RWF: CurrencyCode = CurrencyCode {
    name: "Rwanda Franc",
    code: "RWF",
    numeric: 646,
    unit: 0,
    code_type: "currency",
    countries: &["RWA"],
};

pub const SHP: CurrencyCode = CurrencyCode {
    name: "Saint Helena Pound",
    code: "SHP",
    numeric: 654,
    unit: 2,
    code_type: "currency",
    countries: &["SHN"],
};

pub const WST: CurrencyCode = CurrencyCode {
    name: "Tala",
    code: "WST",
    numeric: 882,
    unit: 2,
    code_type: "currency",
    countries: &["WSM"],
};

pub const STN: CurrencyCode = CurrencyCode {
    name: "Dobra",
    code: "STN",
    numeric: 930,
    unit: 2,
    code_type: "currency",
    countries: &["STP"],
};

pub const SAR: CurrencyCode = CurrencyCode {
    name: "Saudi Riyal",
    code: "SAR",
    numeric: 682,
    unit: 2,
    code_type: "currency",
    countries: &["SAU"],
};

pub const RSD: CurrencyCode = CurrencyCode {
    name: "Serbian Dinar",
    code: "RSD",
    numeric: 941,
    unit: 2,
    code_type: "currency",
    countries: &["SRB"],
};

pub const SCR: CurrencyCode = CurrencyCode {
    name: "Seychelles Rupee",
    code: "SCR",
    numeric: 690,
    unit: 2,
    code_type: "currency",
    countries: &["SYC"],
};

pub const SLL: CurrencyCode = CurrencyCode {
    name: "Leone",
    code: "SLL",
    numeric: 694,
    unit: 2,
    code_type: "currency",
    countries: &["SLE"],
};

pub const SLE: CurrencyCode = CurrencyCode {
    name: "Leone",
    code: "SLE",
    numeric: 925,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const SGD: CurrencyCode = CurrencyCode {
    name: "Singapore Dollar",
    code: "SGD",
    numeric: 702,
    unit: 2,
    code_type: "currency",
    countries: &["SGP"],
};

pub const XSU: CurrencyCode = CurrencyCode {
    name: "Sucre",
    code: "XSU",
    numeric: 994,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const SBD: CurrencyCode = CurrencyCode {
    name: "Solomon Islands Dollar",
    code: "SBD",
    numeric: 090,
    unit: 2,
    code_type: "currency",
    countries: &["SLB"],
};

pub const SOS: CurrencyCode = CurrencyCode {
    name: "Somali Shilling",
    code: "SOS",
    numeric: 706,
    unit: 2,
    code_type: "currency",
    countries: &["SOM"],
};

pub const SSP: CurrencyCode = CurrencyCode {
    name: "South Sudanese Pound",
    code: "SSP",
    numeric: 728,
    unit: 2,
    code_type: "currency",
    countries: &["SSD"],
};

pub const LKR: CurrencyCode = CurrencyCode {
    name: "Sri Lanka Rupee",
    code: "LKR",
    numeric: 144,
    unit: 2,
    code_type: "currency",
    countries: &["LKA"],
};

pub const SDG: CurrencyCode = CurrencyCode {
    name: "Sudanese Pound",
    code: "SDG",
    numeric: 938,
    unit: 2,
    code_type: "currency",
    countries: &["SDN"],
};

pub const SRD: CurrencyCode = CurrencyCode {
    name: "Surinam Dollar",
    code: "SRD",
    numeric: 968,
    unit: 2,
    code_type: "currency",
    countries: &["SUR"],
};

pub const SEK: CurrencyCode = CurrencyCode {
    name: "Swedish Krona",
    code: "SEK",
    numeric: 752,
    unit: 2,
    code_type: "currency",
    countries: &["SWE"],
};

pub const CHE: CurrencyCode = CurrencyCode {
    name: "WIR Euro",
    code: "CHE",
    numeric: 947,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const CHW: CurrencyCode = CurrencyCode {
    name: "WIR Franc",
    code: "CHW",
    numeric: 948,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const SYP: CurrencyCode = CurrencyCode {
    name: "Syrian Pound",
    code: "SYP",
    numeric: 760,
    unit: 2,
    code_type: "currency",
    countries: &["SYR"],
};

pub const TWD: CurrencyCode = CurrencyCode {
    name: "New Taiwan Dollar",
    code: "TWD",
    numeric: 901,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const TJS: CurrencyCode = CurrencyCode {
    name: "Somoni",
    code: "TJS",
    numeric: 972,
    unit: 2,
    code_type: "currency",
    countries: &["TJK"],
};

pub const TZS: CurrencyCode = CurrencyCode {
    name: "Tanzanian Shilling",
    code: "TZS",
    numeric: 834,
    unit: 2,
    code_type: "currency",
    countries: &["TZA"],
};

pub const THB: CurrencyCode = CurrencyCode {
    name: "Baht",
    code: "THB",
    numeric: 764,
    unit: 2,
    code_type: "currency",
    countries: &["THA"],
};

pub const TOP: CurrencyCode = CurrencyCode {
    name: "Pa’anga",
    code: "TOP",
    numeric: 776,
    unit: 2,
    code_type: "currency",
    countries: &["TON"],
};

pub const TTD: CurrencyCode = CurrencyCode {
    name: "Trinidad and Tobago Dollar",
    code: "TTD",
    numeric: 780,
    unit: 2,
    code_type: "currency",
    countries: &["TTO"],
};

pub const TND: CurrencyCode = CurrencyCode {
    name: "Tunisian Dinar",
    code: "TND",
    numeric: 788,
    unit: 3,
    code_type: "currency",
    countries: &["TUN"],
};

pub const TRY: CurrencyCode = CurrencyCode {
    name: "Turkish Lira",
    code: "TRY",
    numeric: 949,
    unit: 2,
    code_type: "currency",
    countries: &["TUR"],
};

pub const TMT: CurrencyCode = CurrencyCode {
    name: "Turkmenistan New Manat",
    code: "TMT",
    numeric: 934,
    unit: 2,
    code_type: "currency",
    countries: &["TKM"],
};

pub const UGX: CurrencyCode = CurrencyCode {
    name: "Uganda Shilling",
    code: "UGX",
    numeric: 800,
    unit: 0,
    code_type: "currency",
    countries: &["UGA"],
};

pub const UAH: CurrencyCode = CurrencyCode {
    name: "Hryvnia",
    code: "UAH",
    numeric: 980,
    unit: 2,
    code_type: "currency",
    countries: &["UKR"],
};

pub const AED: CurrencyCode = CurrencyCode {
    name: "UAE Dirham",
    code: "AED",
    numeric: 784,
    unit: 2,
    code_type: "currency",
    countries: &["ARE"],
};

pub const USN: CurrencyCode = CurrencyCode {
    name: "US Dollar (Next day)",
    code: "USN",
    numeric: 997,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const UYU: CurrencyCode = CurrencyCode {
    name: "Peso Uruguayo",
    code: "UYU",
    numeric: 858,
    unit: 2,
    code_type: "currency",
    countries: &["URY"],
};

pub const UYI: CurrencyCode = CurrencyCode {
    name: "Uruguay Peso en Unidades Indexadas (UI)",
    code: "UYI",
    numeric: 940,
    unit: 0,
    code_type: "currency",
    countries: &[],
};

pub const UYW: CurrencyCode = CurrencyCode {
    name: "Unidad Previsional",
    code: "UYW",
    numeric: 927,
    unit: 4,
    code_type: "currency",
    countries: &[],
};

pub const UZS: CurrencyCode = CurrencyCode {
    name: "Uzbekistan Sum",
    code: "UZS",
    numeric: 860,
    unit: 2,
    code_type: "currency",
    countries: &["UZB"],
};

pub const VUV: CurrencyCode = CurrencyCode {
    name: "Vatu",
    code: "VUV",
    numeric: 548,
    unit: 0,
    code_type: "currency",
    countries: &["VUT"],
};

pub const VES: CurrencyCode = CurrencyCode {
    name: "Bolívar Soberano",
    code: "VES",
    numeric: 928,
    unit: 2,
    code_type: "currency",
    countries: &["VEN"],
};

pub const VED: CurrencyCode = CurrencyCode {
    name: "Bolívar Soberano",
    code: "VED",
    numeric: 926,
    unit: 2,
    code_type: "currency",
    countries: &[],
};

pub const VND: CurrencyCode = CurrencyCode {
    name: "Dong",
    code: "VND",
    numeric: 704,
    unit: 0,
    code_type: "currency",
    countries: &["VNM"],
};

pub const YER: CurrencyCode = CurrencyCode {
    name: "Yemeni Rial",
    code: "YER",
    numeric: 886,
    unit: 2,
    code_type: "currency",
    countries: &["YEM"],
};

pub const ZMW: CurrencyCode = CurrencyCode {
    name: "Zambian Kwacha",
    code: "ZMW",
    numeric: 967,
    unit: 2,
    code_type: "currency",
    countries: &["ZMB"],
};

pub const ZWL: CurrencyCode = CurrencyCode {
    name: "Zimbabwe Dollar",
    code: "ZWL",
    numeric: 932,
    unit: 2,
    code_type: "currency",
    countries: &["ZWE"],
};

pub const XBA: CurrencyCode = CurrencyCode {
    name: "Bond Markets Unit European Composite Unit (EURCO)",
    code: "XBA",
    numeric: 955,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XBB: CurrencyCode = CurrencyCode {
    name: "Bond Markets Unit European Monetary Unit (E.M.U.-6)",
    code: "XBB",
    numeric: 956,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XBC: CurrencyCode = CurrencyCode {
    name: "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)",
    code: "XBC",
    numeric: 957,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XBD: CurrencyCode = CurrencyCode {
    name: "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)",
    code: "XBD",
    numeric: 958,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XTS: CurrencyCode = CurrencyCode {
    name: "Codes specifically reserved for testing purposes",
    code: "XTS",
    numeric: 963,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XXX: CurrencyCode = CurrencyCode {
    name: "The codes assigned for transactions where no currency is involved",
    code: "XXX",
    numeric: 999,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XAU: CurrencyCode = CurrencyCode {
    name: "Gold",
    code: "XAU",
    numeric: 959,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XPD: CurrencyCode = CurrencyCode {
    name: "Palladium",
    code: "XPD",
    numeric: 964,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XPT: CurrencyCode = CurrencyCode {
    name: "Platinum",
    code: "XPT",
    numeric: 962,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const XAG: CurrencyCode = CurrencyCode {
    name: "Silver",
    code: "XAG",
    numeric: 961,
    unit: -1,
    code_type: "currency",
    countries: &[],
};

pub const AFA: CurrencyCode = CurrencyCode {
    name: "Afghani",
    code: "AFA",
    numeric: 004,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const FIM: CurrencyCode = CurrencyCode {
    name: "Markka",
    code: "FIM",
    numeric: 246,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ALK: CurrencyCode = CurrencyCode {
    name: "Old Lek",
    code: "ALK",
    numeric: 008,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ADP: CurrencyCode = CurrencyCode {
    name: "Andorran Peseta",
    code: "ADP",
    numeric: 020,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ESP: CurrencyCode = CurrencyCode {
    name: "Spanish Peseta",
    code: "ESP",
    numeric: 724,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const FRF: CurrencyCode = CurrencyCode {
    name: "French Franc",
    code: "FRF",
    numeric: 250,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const AOK: CurrencyCode = CurrencyCode {
    name: "Kwanza",
    code: "AOK",
    numeric: 024,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const AON: CurrencyCode = CurrencyCode {
    name: "New Kwanza",
    code: "AON",
    numeric: 024,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const AOR: CurrencyCode = CurrencyCode {
    name: "Kwanza Reajustado",
    code: "AOR",
    numeric: 982,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ARA: CurrencyCode = CurrencyCode {
    name: "Austral",
    code: "ARA",
    numeric: 032,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ARP: CurrencyCode = CurrencyCode {
    name: "Peso Argentino",
    code: "ARP",
    numeric: 032,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ARY: CurrencyCode = CurrencyCode {
    name: "Peso",
    code: "ARY",
    numeric: 032,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const RUR: CurrencyCode = CurrencyCode {
    name: "Russian Ruble",
    code: "RUR",
    numeric: 810,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ATS: CurrencyCode = CurrencyCode {
    name: "Schilling",
    code: "ATS",
    numeric: 040,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const AYM: CurrencyCode = CurrencyCode {
    name: "Azerbaijan Manat",
    code: "AYM",
    numeric: 945,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const AZM: CurrencyCode = CurrencyCode {
    name: "Azerbaijanian Manat",
    code: "AZM",
    numeric: 031,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BYB: CurrencyCode = CurrencyCode {
    name: "Belarusian Ruble",
    code: "BYB",
    numeric: 112,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BYR: CurrencyCode = CurrencyCode {
    name: "Belarusian Ruble",
    code: "BYR",
    numeric: 974,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BEC: CurrencyCode = CurrencyCode {
    name: "Convertible Franc",
    code: "BEC",
    numeric: 993,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BEF: CurrencyCode = CurrencyCode {
    name: "Belgian Franc",
    code: "BEF",
    numeric: 056,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BEL: CurrencyCode = CurrencyCode {
    name: "Financial Franc",
    code: "BEL",
    numeric: 992,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BOP: CurrencyCode = CurrencyCode {
    name: "Peso boliviano",
    code: "BOP",
    numeric: 068,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BAD: CurrencyCode = CurrencyCode {
    name: "Dinar",
    code: "BAD",
    numeric: 070,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BRB: CurrencyCode = CurrencyCode {
    name: "Cruzeiro",
    code: "BRB",
    numeric: 076,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BRC: CurrencyCode = CurrencyCode {
    name: "Cruzado",
    code: "BRC",
    numeric: 076,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BRE: CurrencyCode = CurrencyCode {
    name: "Cruzeiro",
    code: "BRE",
    numeric: 076,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BRN: CurrencyCode = CurrencyCode {
    name: "New Cruzado",
    code: "BRN",
    numeric: 076,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BRR: CurrencyCode = CurrencyCode {
    name: "Cruzeiro Real",
    code: "BRR",
    numeric: 987,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BGJ: CurrencyCode = CurrencyCode {
    name: "Lev A/52",
    code: "BGJ",
    numeric: 100,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BGK: CurrencyCode = CurrencyCode {
    name: "Lev A/62",
    code: "BGK",
    numeric: 100,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BGL: CurrencyCode = CurrencyCode {
    name: "Lev",
    code: "BGL",
    numeric: 100,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const BUK: CurrencyCode = CurrencyCode {
    name: "Kyat",
    code: "BUK",
    numeric: 104,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const HRD: CurrencyCode = CurrencyCode {
    name: "Croatian Dinar",
    code: "HRD",
    numeric: 191,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const HRK: CurrencyCode = CurrencyCode {
    name: "Croatian Kuna",
    code: "HRK",
    numeric: 191,
    unit: -1,
    code_type: "historic",
    countries: &["HRV"],
};

pub const CYP: CurrencyCode = CurrencyCode {
    name: "Cyprus Pound",
    code: "CYP",
    numeric: 196,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const CSJ: CurrencyCode = CurrencyCode {
    name: "Krona A/53",
    code: "CSJ",
    numeric: 203,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const CSK: CurrencyCode = CurrencyCode {
    name: "Koruna",
    code: "CSK",
    numeric: 200,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ECS: CurrencyCode = CurrencyCode {
    name: "Sucre",
    code: "ECS",
    numeric: 218,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ECV: CurrencyCode = CurrencyCode {
    name: "Unidad de Valor Constante (UVC)",
    code: "ECV",
    numeric: 983,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GQE: CurrencyCode = CurrencyCode {
    name: "Ekwele",
    code: "GQE",
    numeric: 226,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const EEK: CurrencyCode = CurrencyCode {
    name: "Kroon",
    code: "EEK",
    numeric: 233,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const XEU: CurrencyCode = CurrencyCode {
    name: "European Currency Unit (E.C.U)",
    code: "XEU",
    numeric: 954,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GEK: CurrencyCode = CurrencyCode {
    name: "Georgian Coupon",
    code: "GEK",
    numeric: 268,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const DDM: CurrencyCode = CurrencyCode {
    name: "Mark der DDR",
    code: "DDM",
    numeric: 278,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const DEM: CurrencyCode = CurrencyCode {
    name: "Deutsche Mark",
    code: "DEM",
    numeric: 276,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GHC: CurrencyCode = CurrencyCode {
    name: "Cedi",
    code: "GHC",
    numeric: 288,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GHP: CurrencyCode = CurrencyCode {
    name: "Ghana Cedi",
    code: "GHP",
    numeric: 939,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GRD: CurrencyCode = CurrencyCode {
    name: "Drachma",
    code: "GRD",
    numeric: 300,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GNE: CurrencyCode = CurrencyCode {
    name: "Syli",
    code: "GNE",
    numeric: 324,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GNS: CurrencyCode = CurrencyCode {
    name: "Syli",
    code: "GNS",
    numeric: 324,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GWE: CurrencyCode = CurrencyCode {
    name: "Guinea Escudo",
    code: "GWE",
    numeric: 624,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const GWP: CurrencyCode = CurrencyCode {
    name: "Guinea-Bissau Peso",
    code: "GWP",
    numeric: 624,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ITL: CurrencyCode = CurrencyCode {
    name: "Italian Lira",
    code: "ITL",
    numeric: 380,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ISJ: CurrencyCode = CurrencyCode {
    name: "Old Krona",
    code: "ISJ",
    numeric: 352,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const IEP: CurrencyCode = CurrencyCode {
    name: "Irish Pound",
    code: "IEP",
    numeric: 372,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ILP: CurrencyCode = CurrencyCode {
    name: "Pound",
    code: "ILP",
    numeric: 376,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ILR: CurrencyCode = CurrencyCode {
    name: "Old Shekel",
    code: "ILR",
    numeric: 376,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LAJ: CurrencyCode = CurrencyCode {
    name: "Pathet Lao Kip",
    code: "LAJ",
    numeric: 418,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LVL: CurrencyCode = CurrencyCode {
    name: "Latvian Lats",
    code: "LVL",
    numeric: 428,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LVR: CurrencyCode = CurrencyCode {
    name: "Latvian Ruble",
    code: "LVR",
    numeric: 428,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LSM: CurrencyCode = CurrencyCode {
    name: "Loti",
    code: "LSM",
    numeric: 426,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZAL: CurrencyCode = CurrencyCode {
    name: "Financial Rand",
    code: "ZAL",
    numeric: 991,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LTL: CurrencyCode = CurrencyCode {
    name: "Lithuanian Litas",
    code: "LTL",
    numeric: 440,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LTT: CurrencyCode = CurrencyCode {
    name: "Talonas",
    code: "LTT",
    numeric: 440,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LUC: CurrencyCode = CurrencyCode {
    name: "Luxembourg Convertible Franc",
    code: "LUC",
    numeric: 989,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LUF: CurrencyCode = CurrencyCode {
    name: "Luxembourg Franc",
    code: "LUF",
    numeric: 442,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const LUL: CurrencyCode = CurrencyCode {
    name: "Luxembourg Financial Franc",
    code: "LUL",
    numeric: 988,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MGF: CurrencyCode = CurrencyCode {
    name: "Malagasy Franc",
    code: "MGF",
    numeric: 450,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MVQ: CurrencyCode = CurrencyCode {
    name: "Maldive Rupee",
    code: "MVQ",
    numeric: 462,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MLF: CurrencyCode = CurrencyCode {
    name: "Mali Franc",
    code: "MLF",
    numeric: 466,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MTL: CurrencyCode = CurrencyCode {
    name: "Maltese Lira",
    code: "MTL",
    numeric: 470,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MTP: CurrencyCode = CurrencyCode {
    name: "Maltese Pound",
    code: "MTP",
    numeric: 470,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MRO: CurrencyCode = CurrencyCode {
    name: "Ouguiya",
    code: "MRO",
    numeric: 478,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MXP: CurrencyCode = CurrencyCode {
    name: "Mexican Peso",
    code: "MXP",
    numeric: 484,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MZE: CurrencyCode = CurrencyCode {
    name: "Mozambique Escudo",
    code: "MZE",
    numeric: 508,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const MZM: CurrencyCode = CurrencyCode {
    name: "Mozambique Metical",
    code: "MZM",
    numeric: 508,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const NLG: CurrencyCode = CurrencyCode {
    name: "Netherlands Guilder",
    code: "NLG",
    numeric: 528,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const NIC: CurrencyCode = CurrencyCode {
    name: "Cordoba",
    code: "NIC",
    numeric: 558,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const PEH: CurrencyCode = CurrencyCode {
    name: "Sol",
    code: "PEH",
    numeric: 604,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const PEI: CurrencyCode = CurrencyCode {
    name: "Inti",
    code: "PEI",
    numeric: 604,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const PES: CurrencyCode = CurrencyCode {
    name: "Sol",
    code: "PES",
    numeric: 604,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const PLZ: CurrencyCode = CurrencyCode {
    name: "Zloty",
    code: "PLZ",
    numeric: 616,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const PTE: CurrencyCode = CurrencyCode {
    name: "Portuguese Escudo",
    code: "PTE",
    numeric: 620,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ROK: CurrencyCode = CurrencyCode {
    name: "Leu A/52",
    code: "ROK",
    numeric: 642,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ROL: CurrencyCode = CurrencyCode {
    name: "Old Leu",
    code: "ROL",
    numeric: 642,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const STD: CurrencyCode = CurrencyCode {
    name: "Dobra",
    code: "STD",
    numeric: 678,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const CSD: CurrencyCode = CurrencyCode {
    name: "Serbian Dinar",
    code: "CSD",
    numeric: 891,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SKK: CurrencyCode = CurrencyCode {
    name: "Slovak Koruna",
    code: "SKK",
    numeric: 703,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SIT: CurrencyCode = CurrencyCode {
    name: "Tolar",
    code: "SIT",
    numeric: 705,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const RHD: CurrencyCode = CurrencyCode {
    name: "Rhodesian Dollar",
    code: "RHD",
    numeric: 716,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ESA: CurrencyCode = CurrencyCode {
    name: "Spanish Peseta",
    code: "ESA",
    numeric: 996,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ESB: CurrencyCode = CurrencyCode {
    name: "\"A\" Account (convertible Peseta Account)",
    code: "ESB",
    numeric: 995,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SDD: CurrencyCode = CurrencyCode {
    name: "Sudanese Dinar",
    code: "SDD",
    numeric: 736,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SDP: CurrencyCode = CurrencyCode {
    name: "Sudanese Pound",
    code: "SDP",
    numeric: 736,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SRG: CurrencyCode = CurrencyCode {
    name: "Surinam Guilder",
    code: "SRG",
    numeric: 740,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const CHC: CurrencyCode = CurrencyCode {
    name: "WIR Franc (for electronic)",
    code: "CHC",
    numeric: 948,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const TJR: CurrencyCode = CurrencyCode {
    name: "Tajik Ruble",
    code: "TJR",
    numeric: 762,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const TPE: CurrencyCode = CurrencyCode {
    name: "Timor Escudo",
    code: "TPE",
    numeric: 626,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const TRL: CurrencyCode = CurrencyCode {
    name: "Old Turkish Lira",
    code: "TRL",
    numeric: 792,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const TMM: CurrencyCode = CurrencyCode {
    name: "Turkmenistan Manat",
    code: "TMM",
    numeric: 795,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const UGS: CurrencyCode = CurrencyCode {
    name: "Uganda Shilling",
    code: "UGS",
    numeric: 800,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const UGW: CurrencyCode = CurrencyCode {
    name: "Old Shilling",
    code: "UGW",
    numeric: 800,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const UAK: CurrencyCode = CurrencyCode {
    name: "Karbovanet",
    code: "UAK",
    numeric: 804,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const SUR: CurrencyCode = CurrencyCode {
    name: "Rouble",
    code: "SUR",
    numeric: 810,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const USS: CurrencyCode = CurrencyCode {
    name: "US Dollar (Same day)",
    code: "USS",
    numeric: 998,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const UYN: CurrencyCode = CurrencyCode {
    name: "Old Uruguay Peso",
    code: "UYN",
    numeric: 858,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const UYP: CurrencyCode = CurrencyCode {
    name: "Uruguayan Peso",
    code: "UYP",
    numeric: 858,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const VEB: CurrencyCode = CurrencyCode {
    name: "Bolivar",
    code: "VEB",
    numeric: 862,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const VEF: CurrencyCode = CurrencyCode {
    name: "Bolivar Fuerte",
    code: "VEF",
    numeric: 937,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const VNC: CurrencyCode = CurrencyCode {
    name: "Old Dong",
    code: "VNC",
    numeric: 704,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const YDD: CurrencyCode = CurrencyCode {
    name: "Yemeni Dinar",
    code: "YDD",
    numeric: 720,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const YUD: CurrencyCode = CurrencyCode {
    name: "New Yugoslavian Dinar",
    code: "YUD",
    numeric: 890,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const YUM: CurrencyCode = CurrencyCode {
    name: "New Dinar",
    code: "YUM",
    numeric: 891,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const YUN: CurrencyCode = CurrencyCode {
    name: "Yugoslavian Dinar",
    code: "YUN",
    numeric: 890,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZRN: CurrencyCode = CurrencyCode {
    name: "New Zaire",
    code: "ZRN",
    numeric: 180,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZRZ: CurrencyCode = CurrencyCode {
    name: "Zaire",
    code: "ZRZ",
    numeric: 180,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZMK: CurrencyCode = CurrencyCode {
    name: "Zambian Kwacha",
    code: "ZMK",
    numeric: 894,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZWC: CurrencyCode = CurrencyCode {
    name: "Rhodesian Dollar",
    code: "ZWC",
    numeric: 716,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZWD: CurrencyCode = CurrencyCode {
    name: "Zimbabwe Dollar (old)",
    code: "ZWD",
    numeric: 716,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZWN: CurrencyCode = CurrencyCode {
    name: "Zimbabwe Dollar (new)",
    code: "ZWN",
    numeric: 942,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const ZWR: CurrencyCode = CurrencyCode {
    name: "Zimbabwe Dollar",
    code: "ZWR",
    numeric: 935,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const XFO: CurrencyCode = CurrencyCode {
    name: "Gold-Franc",
    code: "XFO",
    numeric: -1,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const XRE: CurrencyCode = CurrencyCode {
    name: "RINET Funds Code",
    code: "XRE",
    numeric: -1,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

pub const XFU: CurrencyCode = CurrencyCode {
    name: "UIC-Franc",
    code: "XFU",
    numeric: -1,
    unit: -1,
    code_type: "historic",
    countries: &[],
};

///CurrencyCode map with  alpha3  str Code key
pub const ALL_MAP: Map<&str, CurrencyCode> = phf_map! {

"AFN" => AFN,
"EUR" => EUR,
"ALL" => ALL,
"DZD" => DZD,
"USD" => USD,
"AOA" => AOA,
"XCD" => XCD,
"ARS" => ARS,
"AMD" => AMD,
"AWG" => AWG,
"AUD" => AUD,
"AZN" => AZN,
"BSD" => BSD,
"BHD" => BHD,
"BDT" => BDT,
"BBD" => BBD,
"BYN" => BYN,
"BZD" => BZD,
"XOF" => XOF,
"BMD" => BMD,
"INR" => INR,
"BTN" => BTN,
"BOB" => BOB,
"BOV" => BOV,
"BAM" => BAM,
"BWP" => BWP,
"NOK" => NOK,
"BRL" => BRL,
"BND" => BND,
"BGN" => BGN,
"BIF" => BIF,
"CVE" => CVE,
"KHR" => KHR,
"XAF" => XAF,
"CAD" => CAD,
"KYD" => KYD,
"CLP" => CLP,
"CLF" => CLF,
"CNY" => CNY,
"COP" => COP,
"COU" => COU,
"KMF" => KMF,
"CDF" => CDF,
"NZD" => NZD,
"CRC" => CRC,
"CUP" => CUP,
"CUC" => CUC,
"ANG" => ANG,
"CZK" => CZK,
"DKK" => DKK,
"DJF" => DJF,
"DOP" => DOP,
"EGP" => EGP,
"SVC" => SVC,
"ERN" => ERN,
"SZL" => SZL,
"ETB" => ETB,
"FKP" => FKP,
"FJD" => FJD,
"XPF" => XPF,
"GMD" => GMD,
"GEL" => GEL,
"GHS" => GHS,
"GIP" => GIP,
"GTQ" => GTQ,
"GBP" => GBP,
"GNF" => GNF,
"GYD" => GYD,
"HTG" => HTG,
"HNL" => HNL,
"HKD" => HKD,
"HUF" => HUF,
"ISK" => ISK,
"IDR" => IDR,
"XDR" => XDR,
"IRR" => IRR,
"IQD" => IQD,
"ILS" => ILS,
"JMD" => JMD,
"JPY" => JPY,
"JOD" => JOD,
"KZT" => KZT,
"KES" => KES,
"KPW" => KPW,
"KRW" => KRW,
"KWD" => KWD,
"KGS" => KGS,
"LAK" => LAK,
"LBP" => LBP,
"LSL" => LSL,
"ZAR" => ZAR,
"LRD" => LRD,
"LYD" => LYD,
"CHF" => CHF,
"MOP" => MOP,
"MKD" => MKD,
"MGA" => MGA,
"MWK" => MWK,
"MYR" => MYR,
"MVR" => MVR,
"MRU" => MRU,
"MUR" => MUR,
"XUA" => XUA,
"MXN" => MXN,
"MXV" => MXV,
"MDL" => MDL,
"MNT" => MNT,
"MAD" => MAD,
"MZN" => MZN,
"MMK" => MMK,
"NAD" => NAD,
"NPR" => NPR,
"NIO" => NIO,
"NGN" => NGN,
"OMR" => OMR,
"PKR" => PKR,
"PAB" => PAB,
"PGK" => PGK,
"PYG" => PYG,
"PEN" => PEN,
"PHP" => PHP,
"PLN" => PLN,
"QAR" => QAR,
"RON" => RON,
"RUB" => RUB,
"RWF" => RWF,
"SHP" => SHP,
"WST" => WST,
"STN" => STN,
"SAR" => SAR,
"RSD" => RSD,
"SCR" => SCR,
"SLL" => SLL,
"SLE" => SLE,
"SGD" => SGD,
"XSU" => XSU,
"SBD" => SBD,
"SOS" => SOS,
"SSP" => SSP,
"LKR" => LKR,
"SDG" => SDG,
"SRD" => SRD,
"SEK" => SEK,
"CHE" => CHE,
"CHW" => CHW,
"SYP" => SYP,
"TWD" => TWD,
"TJS" => TJS,
"TZS" => TZS,
"THB" => THB,
"TOP" => TOP,
"TTD" => TTD,
"TND" => TND,
"TRY" => TRY,
"TMT" => TMT,
"UGX" => UGX,
"UAH" => UAH,
"AED" => AED,
"USN" => USN,
"UYU" => UYU,
"UYI" => UYI,
"UYW" => UYW,
"UZS" => UZS,
"VUV" => VUV,
"VES" => VES,
"VED" => VED,
"VND" => VND,
"YER" => YER,
"ZMW" => ZMW,
"ZWL" => ZWL,
"XBA" => XBA,
"XBB" => XBB,
"XBC" => XBC,
"XBD" => XBD,
"XTS" => XTS,
"XXX" => XXX,
"XAU" => XAU,
"XPD" => XPD,
"XPT" => XPT,
"XAG" => XAG,
"AFA" => AFA,
"FIM" => FIM,
"ALK" => ALK,
"ADP" => ADP,
"ESP" => ESP,
"FRF" => FRF,
"AOK" => AOK,
"AON" => AON,
"AOR" => AOR,
"ARA" => ARA,
"ARP" => ARP,
"ARY" => ARY,
"RUR" => RUR,
"ATS" => ATS,
"AYM" => AYM,
"AZM" => AZM,
"BYB" => BYB,
"BYR" => BYR,
"BEC" => BEC,
"BEF" => BEF,
"BEL" => BEL,
"BOP" => BOP,
"BAD" => BAD,
"BRB" => BRB,
"BRC" => BRC,
"BRE" => BRE,
"BRN" => BRN,
"BRR" => BRR,
"BGJ" => BGJ,
"BGK" => BGK,
"BGL" => BGL,
"BUK" => BUK,
"HRD" => HRD,
"HRK" => HRK,
"CYP" => CYP,
"CSJ" => CSJ,
"CSK" => CSK,
"ECS" => ECS,
"ECV" => ECV,
"GQE" => GQE,
"EEK" => EEK,
"XEU" => XEU,
"GEK" => GEK,
"DDM" => DDM,
"DEM" => DEM,
"GHC" => GHC,
"GHP" => GHP,
"GRD" => GRD,
"GNE" => GNE,
"GNS" => GNS,
"GWE" => GWE,
"GWP" => GWP,
"ITL" => ITL,
"ISJ" => ISJ,
"IEP" => IEP,
"ILP" => ILP,
"ILR" => ILR,
"LAJ" => LAJ,
"LVL" => LVL,
"LVR" => LVR,
"LSM" => LSM,
"ZAL" => ZAL,
"LTL" => LTL,
"LTT" => LTT,
"LUC" => LUC,
"LUF" => LUF,
"LUL" => LUL,
"MGF" => MGF,
"MVQ" => MVQ,
"MLF" => MLF,
"MTL" => MTL,
"MTP" => MTP,
"MRO" => MRO,
"MXP" => MXP,
"MZE" => MZE,
"MZM" => MZM,
"NLG" => NLG,
"NIC" => NIC,
"PEH" => PEH,
"PEI" => PEI,
"PES" => PES,
"PLZ" => PLZ,
"PTE" => PTE,
"ROK" => ROK,
"ROL" => ROL,
"STD" => STD,
"CSD" => CSD,
"SKK" => SKK,
"SIT" => SIT,
"RHD" => RHD,
"ESA" => ESA,
"ESB" => ESB,
"SDD" => SDD,
"SDP" => SDP,
"SRG" => SRG,
"CHC" => CHC,
"TJR" => TJR,
"TPE" => TPE,
"TRL" => TRL,
"TMM" => TMM,
"UGS" => UGS,
"UGW" => UGW,
"UAK" => UAK,
"SUR" => SUR,
"USS" => USS,
"UYN" => UYN,
"UYP" => UYP,
"VEB" => VEB,
"VEF" => VEF,
"VNC" => VNC,
"YDD" => YDD,
"YUD" => YUD,
"YUM" => YUM,
"YUN" => YUN,
"ZRN" => ZRN,
"ZRZ" => ZRZ,
"ZMK" => ZMK,
"ZWC" => ZWC,
"ZWD" => ZWD,
"ZWN" => ZWN,
"ZWR" => ZWR,
"XFO" => XFO,
"XRE" => XRE,
"XFU" => XFU,

};

///CurrencyCode map with  3 len numeric str Code key
pub const NUMERIC_MAP: Map<&str, CurrencyCode> = phf_map! {

"971" => AFN,
"978" => EUR,
"008" => ALL,
"012" => DZD,
"840" => USD,
"973" => AOA,
"951" => XCD,
"032" => ARS,
"051" => AMD,
"533" => AWG,
"036" => AUD,
"944" => AZN,
"044" => BSD,
"048" => BHD,
"050" => BDT,
"052" => BBD,
"933" => BYN,
"084" => BZD,
"952" => XOF,
"060" => BMD,
"356" => INR,
"064" => BTN,
"068" => BOB,
"984" => BOV,
"977" => BAM,
"072" => BWP,
"578" => NOK,
"986" => BRL,
"096" => BND,
"975" => BGN,
"108" => BIF,
"132" => CVE,
"116" => KHR,
"950" => XAF,
"124" => CAD,
"136" => KYD,
"152" => CLP,
"990" => CLF,
"156" => CNY,
"170" => COP,
"970" => COU,
"174" => KMF,
"976" => CDF,
"554" => NZD,
"188" => CRC,
"192" => CUP,
"931" => CUC,
"532" => ANG,
"203" => CZK,
"208" => DKK,
"262" => DJF,
"214" => DOP,
"818" => EGP,
"222" => SVC,
"232" => ERN,
"748" => SZL,
"230" => ETB,
"238" => FKP,
"242" => FJD,
"953" => XPF,
"270" => GMD,
"981" => GEL,
"936" => GHS,
"292" => GIP,
"320" => GTQ,
"826" => GBP,
"324" => GNF,
"328" => GYD,
"332" => HTG,
"340" => HNL,
"344" => HKD,
"348" => HUF,
"352" => ISK,
"360" => IDR,
"960" => XDR,
"364" => IRR,
"368" => IQD,
"376" => ILS,
"388" => JMD,
"392" => JPY,
"400" => JOD,
"398" => KZT,
"404" => KES,
"408" => KPW,
"410" => KRW,
"414" => KWD,
"417" => KGS,
"418" => LAK,
"422" => LBP,
"426" => LSL,
"710" => ZAR,
"430" => LRD,
"434" => LYD,
"756" => CHF,
"446" => MOP,
"807" => MKD,
"969" => MGA,
"454" => MWK,
"458" => MYR,
"462" => MVR,
"929" => MRU,
"480" => MUR,
"965" => XUA,
"484" => MXN,
"979" => MXV,
"498" => MDL,
"496" => MNT,
"504" => MAD,
"943" => MZN,
"104" => MMK,
"516" => NAD,
"524" => NPR,
"558" => NIO,
"566" => NGN,
"512" => OMR,
"586" => PKR,
"590" => PAB,
"598" => PGK,
"600" => PYG,
"604" => PEN,
"608" => PHP,
"985" => PLN,
"634" => QAR,
"946" => RON,
"643" => RUB,
"646" => RWF,
"654" => SHP,
"882" => WST,
"930" => STN,
"682" => SAR,
"941" => RSD,
"690" => SCR,
"694" => SLL,
"925" => SLE,
"702" => SGD,
"994" => XSU,
"090" => SBD,
"706" => SOS,
"728" => SSP,
"144" => LKR,
"938" => SDG,
"968" => SRD,
"752" => SEK,
"947" => CHE,
"948" => CHW,
"760" => SYP,
"901" => TWD,
"972" => TJS,
"834" => TZS,
"764" => THB,
"776" => TOP,
"780" => TTD,
"788" => TND,
"949" => TRY,
"934" => TMT,
"800" => UGX,
"980" => UAH,
"784" => AED,
"997" => USN,
"858" => UYU,
"940" => UYI,
"927" => UYW,
"860" => UZS,
"548" => VUV,
"928" => VES,
"926" => VED,
"704" => VND,
"886" => YER,
"967" => ZMW,
"932" => ZWL,
"955" => XBA,
"956" => XBB,
"957" => XBC,
"958" => XBD,
"963" => XTS,
"999" => XXX,
"959" => XAU,
"964" => XPD,
"962" => XPT,
"961" => XAG,
"004" => AFA,
"246" => FIM,
"020" => ADP,
"724" => ESP,
"250" => FRF,
"024" => AOK,
"982" => AOR,
"810" => RUR,
"040" => ATS,
"945" => AYM,
"031" => AZM,
"112" => BYB,
"974" => BYR,
"993" => BEC,
"056" => BEF,
"992" => BEL,
"070" => BAD,
"076" => BRB,
"987" => BRR,
"100" => BGJ,
"191" => HRD,
"196" => CYP,
"200" => CSK,
"218" => ECS,
"983" => ECV,
"226" => GQE,
"233" => EEK,
"954" => XEU,
"268" => GEK,
"278" => DDM,
"276" => DEM,
"288" => GHC,
"939" => GHP,
"300" => GRD,
"624" => GWE,
"380" => ITL,
"372" => IEP,
"428" => LVL,
"991" => ZAL,
"440" => LTL,
"989" => LUC,
"442" => LUF,
"988" => LUL,
"450" => MGF,
"466" => MLF,
"470" => MTL,
"478" => MRO,
"508" => MZE,
"528" => NLG,
"616" => PLZ,
"620" => PTE,
"642" => ROK,
"678" => STD,
"891" => CSD,
"703" => SKK,
"705" => SIT,
"716" => RHD,
"996" => ESA,
"995" => ESB,
"736" => SDD,
"740" => SRG,
"762" => TJR,
"626" => TPE,
"792" => TRL,
"795" => TMM,
"804" => UAK,
"998" => USS,
"862" => VEB,
"937" => VEF,
"720" => YDD,
"890" => YUD,
"180" => ZRN,
"894" => ZMK,
"942" => ZWN,
"935" => ZWR,

};

///CurrencyCode map with  3 len numeric str Code key
pub const COUNTRY_MAP: Map<&str, &'static [&'static CurrencyCode]> = phf_map! {

"AFG" => &[&AFN],
"ALB" => &[&ALL],
"DZA" => &[&DZD],
"ASM" => &[&USD],
"AND" => &[&EUR],
"AGO" => &[&AOA],
"AIA" => &[&XCD],
"ATG" => &[&XCD],
"ARG" => &[&ARS],
"ARM" => &[&AMD],
"ABW" => &[&AWG],
"AUS" => &[&AUD],
"AUT" => &[&EUR],
"AZE" => &[&AZN],
"BHS" => &[&BSD],
"BHR" => &[&BHD],
"BGD" => &[&BDT],
"BRB" => &[&BBD],
"BLR" => &[&BYN],
"BEL" => &[&EUR],
"BLZ" => &[&BZD],
"BEN" => &[&XOF],
"BMU" => &[&BMD],
"BTN" => &[&INR,&BTN],
"BOL" => &[&BOB],
"BES" => &[&USD],
"BIH" => &[&BAM],
"BWA" => &[&BWP],
"BVT" => &[&NOK],
"BRA" => &[&BRL],
"IOT" => &[&USD],
"VGB" => &[&USD],
"BRN" => &[&BND],
"BGR" => &[&BGN],
"BFA" => &[&XOF],
"BDI" => &[&BIF],
"CPV" => &[&CVE],
"KHM" => &[&KHR],
"CMR" => &[&XAF],
"CAN" => &[&CAD],
"CYM" => &[&KYD],
"CAF" => &[&XAF],
"TCD" => &[&XAF],
"CHL" => &[&CLP],
"CHN" => &[&CNY],
"HKG" => &[&HKD],
"MAC" => &[&MOP],
"CXR" => &[&AUD],
"CCK" => &[&AUD],
"COL" => &[&COP],
"COM" => &[&KMF],
"COG" => &[&XAF],
"COK" => &[&NZD],
"CRI" => &[&CRC],
"HRV" => &[&HRK],
"CUB" => &[&CUP,&CUC],
"CUW" => &[&ANG],
"CYP" => &[&EUR],
"CZE" => &[&CZK],
"CIV" => &[&XOF],
"PRK" => &[&KPW],
"COD" => &[&CDF],
"DNK" => &[&DKK],
"DJI" => &[&DJF],
"DMA" => &[&XCD],
"DOM" => &[&DOP],
"ECU" => &[&USD],
"EGY" => &[&EGP],
"SLV" => &[&SVC,&USD],
"GNQ" => &[&XAF],
"ERI" => &[&ERN],
"EST" => &[&EUR],
"SWZ" => &[&SZL],
"ETH" => &[&ETB],
"FRO" => &[&DKK],
"FJI" => &[&FJD],
"FIN" => &[&EUR],
"FRA" => &[&EUR],
"GUF" => &[&EUR],
"PYF" => &[&XPF],
"ATF" => &[&EUR],
"GAB" => &[&XAF],
"GMB" => &[&GMD],
"GEO" => &[&GEL],
"DEU" => &[&EUR],
"GHA" => &[&GHS],
"GIB" => &[&GIP],
"GRC" => &[&EUR],
"GRL" => &[&DKK],
"GRD" => &[&XCD],
"GLP" => &[&EUR],
"GUM" => &[&USD],
"GTM" => &[&GTQ],
"GGY" => &[&GBP],
"GIN" => &[&GNF],
"GNB" => &[&XOF],
"GUY" => &[&GYD],
"HTI" => &[&HTG,&USD],
"HMD" => &[&AUD],
"VAT" => &[&EUR],
"HND" => &[&HNL],
"HUN" => &[&HUF],
"ISL" => &[&ISK],
"IND" => &[&INR],
"IDN" => &[&IDR],
"IRN" => &[&IRR],
"IRQ" => &[&IQD],
"IRL" => &[&EUR],
"IMN" => &[&GBP],
"ISR" => &[&ILS],
"ITA" => &[&EUR],
"JAM" => &[&JMD],
"JPN" => &[&JPY],
"JEY" => &[&GBP],
"JOR" => &[&JOD],
"KAZ" => &[&KZT],
"KEN" => &[&KES],
"KIR" => &[&AUD],
"KWT" => &[&KWD],
"KGZ" => &[&KGS],
"LAO" => &[&LAK],
"LVA" => &[&EUR],
"LBN" => &[&LBP],
"LSO" => &[&LSL,&ZAR],
"LBR" => &[&LRD],
"LBY" => &[&LYD],
"LIE" => &[&CHF],
"LTU" => &[&EUR],
"LUX" => &[&EUR],
"MDG" => &[&MGA],
"MWI" => &[&MWK],
"MYS" => &[&MYR],
"MDV" => &[&MVR],
"MLI" => &[&XOF],
"MLT" => &[&EUR],
"MHL" => &[&USD],
"MTQ" => &[&EUR],
"MRT" => &[&MRU],
"MUS" => &[&MUR],
"MYT" => &[&EUR],
"MEX" => &[&MXN],
"FSM" => &[&USD],
"MCO" => &[&EUR],
"MNG" => &[&MNT],
"MNE" => &[&EUR],
"MSR" => &[&XCD],
"MAR" => &[&MAD],
"MOZ" => &[&MZN],
"MMR" => &[&MMK],
"NAM" => &[&NAD,&ZAR],
"NRU" => &[&AUD],
"NPL" => &[&NPR],
"NLD" => &[&EUR],
"NCL" => &[&XPF],
"NZL" => &[&NZD],
"NIC" => &[&NIO],
"NER" => &[&XOF],
"NGA" => &[&NGN],
"NIU" => &[&NZD],
"NFK" => &[&AUD],
"MNP" => &[&USD],
"NOR" => &[&NOK],
"OMN" => &[&OMR],
"PAK" => &[&PKR],
"PLW" => &[&USD],
"PAN" => &[&PAB,&USD],
"PNG" => &[&PGK],
"PRY" => &[&PYG],
"PER" => &[&PEN],
"PHL" => &[&PHP],
"PCN" => &[&NZD],
"POL" => &[&PLN],
"PRT" => &[&EUR],
"PRI" => &[&USD],
"QAT" => &[&QAR],
"KOR" => &[&KRW],
"MDA" => &[&MDL],
"ROU" => &[&RON],
"RUS" => &[&RUB],
"RWA" => &[&RWF],
"REU" => &[&EUR],
"BLM" => &[&EUR],
"SHN" => &[&SHP],
"KNA" => &[&XCD],
"LCA" => &[&XCD],
"MAF" => &[&EUR],
"SPM" => &[&EUR],
"VCT" => &[&XCD],
"WSM" => &[&WST],
"SMR" => &[&EUR],
"STP" => &[&STN],
"SAU" => &[&SAR],
"SEN" => &[&XOF],
"SRB" => &[&RSD],
"SYC" => &[&SCR],
"SLE" => &[&SLL],
"SGP" => &[&SGD],
"SXM" => &[&ANG],
"SVK" => &[&EUR],
"SVN" => &[&EUR],
"SLB" => &[&SBD],
"SOM" => &[&SOS],
"ZAF" => &[&ZAR],
"SSD" => &[&SSP],
"ESP" => &[&EUR],
"LKA" => &[&LKR],
"SDN" => &[&SDG],
"SUR" => &[&SRD],
"SJM" => &[&NOK],
"SWE" => &[&SEK],
"CHE" => &[&CHF],
"SYR" => &[&SYP],
"TJK" => &[&TJS],
"THA" => &[&THB],
"MKD" => &[&MKD],
"TLS" => &[&USD],
"TGO" => &[&XOF],
"TKL" => &[&NZD],
"TON" => &[&TOP],
"TTO" => &[&TTD],
"TUN" => &[&TND],
"TUR" => &[&TRY],
"TKM" => &[&TMT],
"TCA" => &[&USD],
"TUV" => &[&AUD],
"UGA" => &[&UGX],
"UKR" => &[&UAH],
"ARE" => &[&AED],
"GBR" => &[&GBP],
"TZA" => &[&TZS],
"UMI" => &[&USD],
"VIR" => &[&USD],
"USA" => &[&USD],
"URY" => &[&UYU],
"UZB" => &[&UZS],
"VUT" => &[&VUV],
"VEN" => &[&VES],
"VNM" => &[&VND],
"WLF" => &[&XPF],
"ESH" => &[&MAD],
"YEM" => &[&YER],
"ZMB" => &[&ZMW],
"ZWE" => &[&ZWL],
"ALA" => &[&EUR],

};

///ALL names
pub const ALL_NAME: &[&str] = &[
    "Afghani",
    "Euro",
    "Lek",
    "Algerian Dinar",
    "US Dollar",
    "Kwanza",
    "East Caribbean Dollar",
    "Argentine Peso",
    "Armenian Dram",
    "Aruban Florin",
    "Australian Dollar",
    "Azerbaijan Manat",
    "Bahamian Dollar",
    "Bahraini Dinar",
    "Taka",
    "Barbados Dollar",
    "Belarusian Ruble",
    "Belize Dollar",
    "CFA Franc BCEAO",
    "Bermudian Dollar",
    "Indian Rupee",
    "Ngultrum",
    "Boliviano",
    "Mvdol",
    "Convertible Mark",
    "Pula",
    "Norwegian Krone",
    "Brazilian Real",
    "Brunei Dollar",
    "Bulgarian Lev",
    "Burundi Franc",
    "Cabo Verde Escudo",
    "Riel",
    "CFA Franc BEAC",
    "Canadian Dollar",
    "Cayman Islands Dollar",
    "Chilean Peso",
    "Unidad de Fomento",
    "Yuan Renminbi",
    "Colombian Peso",
    "Unidad de Valor Real",
    "Comorian Franc ",
    "Congolese Franc",
    "New Zealand Dollar",
    "Costa Rican Colon",
    "Cuban Peso",
    "Peso Convertible",
    "Netherlands Antillean Guilder",
    "Czech Koruna",
    "Danish Krone",
    "Djibouti Franc",
    "Dominican Peso",
    "Egyptian Pound",
    "El Salvador Colon",
    "Nakfa",
    "Lilangeni",
    "Ethiopian Birr",
    "Falkland Islands Pound",
    "Fiji Dollar",
    "CFP Franc",
    "Dalasi",
    "Lari",
    "Ghana Cedi",
    "Gibraltar Pound",
    "Quetzal",
    "Pound Sterling",
    "Guinean Franc",
    "Guyana Dollar",
    "Gourde",
    "Lempira",
    "Hong Kong Dollar",
    "Forint",
    "Iceland Krona",
    "Rupiah",
    "SDR (Special Drawing Right)",
    "Iranian Rial",
    "Iraqi Dinar",
    "New Israeli Sheqel",
    "Jamaican Dollar",
    "Yen",
    "Jordanian Dinar",
    "Tenge",
    "Kenyan Shilling",
    "North Korean Won",
    "Won",
    "Kuwaiti Dinar",
    "Som",
    "Lao Kip",
    "Lebanese Pound",
    "Loti",
    "Rand",
    "Liberian Dollar",
    "Libyan Dinar",
    "Swiss Franc",
    "Pataca",
    "Denar",
    "Malagasy Ariary",
    "Malawi Kwacha",
    "Malaysian Ringgit",
    "Rufiyaa",
    "Ouguiya",
    "Mauritius Rupee",
    "ADB Unit of Account",
    "Mexican Peso",
    "Mexican Unidad de Inversion (UDI)",
    "Moldovan Leu",
    "Tugrik",
    "Moroccan Dirham",
    "Mozambique Metical",
    "Kyat",
    "Namibia Dollar",
    "Nepalese Rupee",
    "Cordoba Oro",
    "Naira",
    "Rial Omani",
    "Pakistan Rupee",
    "Balboa",
    "Kina",
    "Guarani",
    "Sol",
    "Philippine Peso",
    "Zloty",
    "Qatari Rial",
    "Romanian Leu",
    "Russian Ruble",
    "Rwanda Franc",
    "Saint Helena Pound",
    "Tala",
    "Dobra",
    "Saudi Riyal",
    "Serbian Dinar",
    "Seychelles Rupee",
    "Leone",
    "Leone",
    "Singapore Dollar",
    "Sucre",
    "Solomon Islands Dollar",
    "Somali Shilling",
    "South Sudanese Pound",
    "Sri Lanka Rupee",
    "Sudanese Pound",
    "Surinam Dollar",
    "Swedish Krona",
    "WIR Euro",
    "WIR Franc",
    "Syrian Pound",
    "New Taiwan Dollar",
    "Somoni",
    "Tanzanian Shilling",
    "Baht",
    "Pa’anga",
    "Trinidad and Tobago Dollar",
    "Tunisian Dinar",
    "Turkish Lira",
    "Turkmenistan New Manat",
    "Uganda Shilling",
    "Hryvnia",
    "UAE Dirham",
    "US Dollar (Next day)",
    "Peso Uruguayo",
    "Uruguay Peso en Unidades Indexadas (UI)",
    "Unidad Previsional",
    "Uzbekistan Sum",
    "Vatu",
    "Bolívar Soberano",
    "Bolívar Soberano",
    "Dong",
    "Yemeni Rial",
    "Zambian Kwacha",
    "Zimbabwe Dollar",
    "Bond Markets Unit European Composite Unit (EURCO)",
    "Bond Markets Unit European Monetary Unit (E.M.U.-6)",
    "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)",
    "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)",
    "Codes specifically reserved for testing purposes",
    "The codes assigned for transactions where no currency is involved",
    "Gold",
    "Palladium",
    "Platinum",
    "Silver",
    "Afghani",
    "Markka",
    "Old Lek",
    "Andorran Peseta",
    "Spanish Peseta",
    "French Franc",
    "Kwanza",
    "New Kwanza",
    "Kwanza Reajustado",
    "Austral",
    "Peso Argentino",
    "Peso",
    "Russian Ruble",
    "Schilling",
    "Azerbaijan Manat",
    "Azerbaijanian Manat",
    "Belarusian Ruble",
    "Belarusian Ruble",
    "Convertible Franc",
    "Belgian Franc",
    "Financial Franc",
    "Peso boliviano",
    "Dinar",
    "Cruzeiro",
    "Cruzado",
    "Cruzeiro",
    "New Cruzado",
    "Cruzeiro Real",
    "Lev A/52",
    "Lev A/62",
    "Lev",
    "Kyat",
    "Croatian Dinar",
    "Croatian Kuna",
    "Cyprus Pound",
    "Krona A/53",
    "Koruna",
    "Sucre",
    "Unidad de Valor Constante (UVC)",
    "Ekwele",
    "Kroon",
    "European Currency Unit (E.C.U)",
    "Georgian Coupon",
    "Mark der DDR",
    "Deutsche Mark",
    "Cedi",
    "Ghana Cedi",
    "Drachma",
    "Syli",
    "Syli",
    "Guinea Escudo",
    "Guinea-Bissau Peso",
    "Italian Lira",
    "Old Krona",
    "Irish Pound",
    "Pound",
    "Old Shekel",
    "Pathet Lao Kip",
    "Latvian Lats",
    "Latvian Ruble",
    "Loti",
    "Financial Rand",
    "Lithuanian Litas",
    "Talonas",
    "Luxembourg Convertible Franc",
    "Luxembourg Franc",
    "Luxembourg Financial Franc",
    "Malagasy Franc",
    "Maldive Rupee",
    "Mali Franc",
    "Maltese Lira",
    "Maltese Pound",
    "Ouguiya",
    "Mexican Peso",
    "Mozambique Escudo",
    "Mozambique Metical",
    "Netherlands Guilder",
    "Cordoba",
    "Sol",
    "Inti",
    "Sol",
    "Zloty",
    "Portuguese Escudo",
    "Leu A/52",
    "Old Leu",
    "Dobra",
    "Serbian Dinar",
    "Slovak Koruna",
    "Tolar",
    "Rhodesian Dollar",
    "Spanish Peseta",
    "\"A\" Account (convertible Peseta Account)",
    "Sudanese Dinar",
    "Sudanese Pound",
    "Surinam Guilder",
    "WIR Franc (for electronic)",
    "Tajik Ruble",
    "Timor Escudo",
    "Old Turkish Lira",
    "Turkmenistan Manat",
    "Uganda Shilling",
    "Old Shilling",
    "Karbovanet",
    "Rouble",
    "US Dollar (Same day)",
    "Old Uruguay Peso",
    "Uruguayan Peso",
    "Bolivar",
    "Bolivar Fuerte",
    "Old Dong",
    "Yemeni Dinar",
    "New Yugoslavian Dinar",
    "New Dinar",
    "Yugoslavian Dinar",
    "New Zaire",
    "Zaire",
    "Zambian Kwacha",
    "Rhodesian Dollar",
    "Zimbabwe Dollar (old)",
    "Zimbabwe Dollar (new)",
    "Zimbabwe Dollar",
    "Gold-Franc",
    "RINET Funds Code",
    "UIC-Franc",
];

///ALL codes
pub const ALL_CODE: &[&str] = &[
    "AFN", "EUR", "ALL", "DZD", "USD", "AOA", "XCD", "ARS", "AMD", "AWG", "AUD", "AZN", "BSD",
    "BHD", "BDT", "BBD", "BYN", "BZD", "XOF", "BMD", "INR", "BTN", "BOB", "BOV", "BAM", "BWP",
    "NOK", "BRL", "BND", "BGN", "BIF", "CVE", "KHR", "XAF", "CAD", "KYD", "CLP", "CLF", "CNY",
    "COP", "COU", "KMF", "CDF", "NZD", "CRC", "CUP", "CUC", "ANG", "CZK", "DKK", "DJF", "DOP",
    "EGP", "SVC", "ERN", "SZL", "ETB", "FKP", "FJD", "XPF", "GMD", "GEL", "GHS", "GIP", "GTQ",
    "GBP", "GNF", "GYD", "HTG", "HNL", "HKD", "HUF", "ISK", "IDR", "XDR", "IRR", "IQD", "ILS",
    "JMD", "JPY", "JOD", "KZT", "KES", "KPW", "KRW", "KWD", "KGS", "LAK", "LBP", "LSL", "ZAR",
    "LRD", "LYD", "CHF", "MOP", "MKD", "MGA", "MWK", "MYR", "MVR", "MRU", "MUR", "XUA", "MXN",
    "MXV", "MDL", "MNT", "MAD", "MZN", "MMK", "NAD", "NPR", "NIO", "NGN", "OMR", "PKR", "PAB",
    "PGK", "PYG", "PEN", "PHP", "PLN", "QAR", "RON", "RUB", "RWF", "SHP", "WST", "STN", "SAR",
    "RSD", "SCR", "SLL", "SLE", "SGD", "XSU", "SBD", "SOS", "SSP", "LKR", "SDG", "SRD", "SEK",
    "CHE", "CHW", "SYP", "TWD", "TJS", "TZS", "THB", "TOP", "TTD", "TND", "TRY", "TMT", "UGX",
    "UAH", "AED", "USN", "UYU", "UYI", "UYW", "UZS", "VUV", "VES", "VED", "VND", "YER", "ZMW",
    "ZWL", "XBA", "XBB", "XBC", "XBD", "XTS", "XXX", "XAU", "XPD", "XPT", "XAG", "AFA", "FIM",
    "ALK", "ADP", "ESP", "FRF", "AOK", "AON", "AOR", "ARA", "ARP", "ARY", "RUR", "ATS", "AYM",
    "AZM", "BYB", "BYR", "BEC", "BEF", "BEL", "BOP", "BAD", "BRB", "BRC", "BRE", "BRN", "BRR",
    "BGJ", "BGK", "BGL", "BUK", "HRD", "HRK", "CYP", "CSJ", "CSK", "ECS", "ECV", "GQE", "EEK",
    "XEU", "GEK", "DDM", "DEM", "GHC", "GHP", "GRD", "GNE", "GNS", "GWE", "GWP", "ITL", "ISJ",
    "IEP", "ILP", "ILR", "LAJ", "LVL", "LVR", "LSM", "ZAL", "LTL", "LTT", "LUC", "LUF", "LUL",
    "MGF", "MVQ", "MLF", "MTL", "MTP", "MRO", "MXP", "MZE", "MZM", "NLG", "NIC", "PEH", "PEI",
    "PES", "PLZ", "PTE", "ROK", "ROL", "STD", "CSD", "SKK", "SIT", "RHD", "ESA", "ESB", "SDD",
    "SDP", "SRG", "CHC", "TJR", "TPE", "TRL", "TMM", "UGS", "UGW", "UAK", "SUR", "USS", "UYN",
    "UYP", "VEB", "VEF", "VNC", "YDD", "YUD", "YUM", "YUN", "ZRN", "ZRZ", "ZMK", "ZWC", "ZWD",
    "ZWN", "ZWR", "XFO", "XRE", "XFU",
];

///ALL the CurrencyCode struct
pub const ALL_NUMERIC: &[i32] = &[
    971, 978, 008, 012, 840, 973, 951, 032, 051, 533, 036, 944, 044, 048, 050, 052, 933, 084, 952,
    060, 356, 064, 068, 984, 977, 072, 578, 986, 096, 975, 108, 132, 116, 950, 124, 136, 152, 990,
    156, 170, 970, 174, 976, 554, 188, 192, 931, 532, 203, 208, 262, 214, 818, 222, 232, 748, 230,
    238, 242, 953, 270, 981, 936, 292, 320, 826, 324, 328, 332, 340, 344, 348, 352, 360, 960, 364,
    368, 376, 388, 392, 400, 398, 404, 408, 410, 414, 417, 418, 422, 426, 710, 430, 434, 756, 446,
    807, 969, 454, 458, 462, 929, 480, 965, 484, 979, 498, 496, 504, 943, 104, 516, 524, 558, 566,
    512, 586, 590, 598, 600, 604, 608, 985, 634, 946, 643, 646, 654, 882, 930, 682, 941, 690, 694,
    925, 702, 994, 090, 706, 728, 144, 938, 968, 752, 947, 948, 760, 901, 972, 834, 764, 776, 780,
    788, 949, 934, 800, 980, 784, 997, 858, 940, 927, 860, 548, 928, 926, 704, 886, 967, 932, 955,
    956, 957, 958, 963, 999, 959, 964, 962, 961, 004, 246, 020, 724, 250, 024, 982, 810, 040, 945,
    031, 112, 974, 993, 056, 992, 070, 076, 987, 100, 191, 196, 200, 218, 983, 226, 233, 954, 268,
    278, 276, 288, 939, 300, 624, 380, 372, 428, 991, 440, 989, 442, 988, 450, 466, 470, 478, 508,
    528, 616, 620, 642, 678, 891, 703, 705, 716, 996, 995, 736, 740, 762, 626, 792, 795, 804, 998,
    862, 937, 720, 890, 180, 894, 942, 935,
];

///ALL the CurrencyCode struct
pub const ALL_NUMERIC_STR: &[&str] = &[
    "971", "978", "008", "012", "840", "973", "951", "032", "051", "533", "036", "944", "044",
    "048", "050", "052", "933", "084", "952", "060", "356", "064", "068", "984", "977", "072",
    "578", "986", "096", "975", "108", "132", "116", "950", "124", "136", "152", "990", "156",
    "170", "970", "174", "976", "554", "188", "192", "931", "532", "203", "208", "262", "214",
    "818", "222", "232", "748", "230", "238", "242", "953", "270", "981", "936", "292", "320",
    "826", "324", "328", "332", "340", "344", "348", "352", "360", "960", "364", "368", "376",
    "388", "392", "400", "398", "404", "408", "410", "414", "417", "418", "422", "426", "710",
    "430", "434", "756", "446", "807", "969", "454", "458", "462", "929", "480", "965", "484",
    "979", "498", "496", "504", "943", "104", "516", "524", "558", "566", "512", "586", "590",
    "598", "600", "604", "608", "985", "634", "946", "643", "646", "654", "882", "930", "682",
    "941", "690", "694", "925", "702", "994", "090", "706", "728", "144", "938", "968", "752",
    "947", "948", "760", "901", "972", "834", "764", "776", "780", "788", "949", "934", "800",
    "980", "784", "997", "858", "940", "927", "860", "548", "928", "926", "704", "886", "967",
    "932", "955", "956", "957", "958", "963", "999", "959", "964", "962", "961", "004", "246",
    "020", "724", "250", "024", "982", "810", "040", "945", "031", "112", "974", "993", "056",
    "992", "070", "076", "987", "100", "191", "196", "200", "218", "983", "226", "233", "954",
    "268", "278", "276", "288", "939", "300", "624", "380", "372", "428", "991", "440", "989",
    "442", "988", "450", "466", "470", "478", "508", "528", "616", "620", "642", "678", "891",
    "703", "705", "716", "996", "995", "736", "740", "762", "626", "792", "795", "804", "998",
    "862", "937", "720", "890", "180", "894", "942", "935",
];

///ALL the CurrencyCode struct
pub const ALL_CODES: &[CurrencyCode] = &[
    AFN, EUR, ALL, DZD, USD, AOA, XCD, ARS, AMD, AWG, AUD, AZN, BSD, BHD, BDT, BBD, BYN, BZD, XOF,
    BMD, INR, BTN, BOB, BOV, BAM, BWP, NOK, BRL, BND, BGN, BIF, CVE, KHR, XAF, CAD, KYD, CLP, CLF,
    CNY, COP, COU, KMF, CDF, NZD, CRC, CUP, CUC, ANG, CZK, DKK, DJF, DOP, EGP, SVC, ERN, SZL, ETB,
    FKP, FJD, XPF, GMD, GEL, GHS, GIP, GTQ, GBP, GNF, GYD, HTG, HNL, HKD, HUF, ISK, IDR, XDR, IRR,
    IQD, ILS, JMD, JPY, JOD, KZT, KES, KPW, KRW, KWD, KGS, LAK, LBP, LSL, ZAR, LRD, LYD, CHF, MOP,
    MKD, MGA, MWK, MYR, MVR, MRU, MUR, XUA, MXN, MXV, MDL, MNT, MAD, MZN, MMK, NAD, NPR, NIO, NGN,
    OMR, PKR, PAB, PGK, PYG, PEN, PHP, PLN, QAR, RON, RUB, RWF, SHP, WST, STN, SAR, RSD, SCR, SLL,
    SLE, SGD, XSU, SBD, SOS, SSP, LKR, SDG, SRD, SEK, CHE, CHW, SYP, TWD, TJS, TZS, THB, TOP, TTD,
    TND, TRY, TMT, UGX, UAH, AED, USN, UYU, UYI, UYW, UZS, VUV, VES, VED, VND, YER, ZMW, ZWL, XBA,
    XBB, XBC, XBD, XTS, XXX, XAU, XPD, XPT, XAG, AFA, FIM, ALK, ADP, ESP, FRF, AOK, AON, AOR, ARA,
    ARP, ARY, RUR, ATS, AYM, AZM, BYB, BYR, BEC, BEF, BEL, BOP, BAD, BRB, BRC, BRE, BRN, BRR, BGJ,
    BGK, BGL, BUK, HRD, HRK, CYP, CSJ, CSK, ECS, ECV, GQE, EEK, XEU, GEK, DDM, DEM, GHC, GHP, GRD,
    GNE, GNS, GWE, GWP, ITL, ISJ, IEP, ILP, ILR, LAJ, LVL, LVR, LSM, ZAL, LTL, LTT, LUC, LUF, LUL,
    MGF, MVQ, MLF, MTL, MTP, MRO, MXP, MZE, MZM, NLG, NIC, PEH, PEI, PES, PLZ, PTE, ROK, ROL, STD,
    CSD, SKK, SIT, RHD, ESA, ESB, SDD, SDP, SRG, CHC, TJR, TPE, TRL, TMM, UGS, UGW, UAK, SUR, USS,
    UYN, UYP, VEB, VEF, VNC, YDD, YUD, YUM, YUN, ZRN, ZRZ, ZMK, ZWC, ZWD, ZWN, ZWR, XFO, XRE, XFU,
];
