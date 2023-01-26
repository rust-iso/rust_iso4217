#!/usr/bin/env python3.9
import pandas as pd
import re
import sys
import csv
list_one_xls = pd.ExcelFile(r"list-one.xls")
table_active = list_one_xls.parse(list_one_xls.sheet_names[0])

active_keys = ["entity", "currency", "code", "numeric", "unit"]
codes = []
codes_map = {}
for row in table_active.index:
    code = {}
    for i, k in enumerate(active_keys):
        code[k] = table_active.iloc[row][i]
    if pd.isna(code["code"]):
        continue
    if code["entity"] == "ENTITY":
        continue
    code["type"] = "currency"
    if code["code"] in ["BOV", "CLF", "COU", "CHW", "CHE", "MXV", "USN", "UYI", "UYW"]:
        code["type"] = "funds"
    del code["entity"]
    if code["code"] not in codes_map:
        codes_map[code["code"]] = code
        codes.append(code)
    else:
        pass


"""table_funds = list_one_xls.parse(list_one_xls.sheet_names[1])
for row in table_funds.index:
    code = {}
    code["code"] = table_funds.iloc[row][0].strip()
    if code["code"] == "Code":
        continue
    code["type"] = "funds"
    if code["code"] == "BOV":
        code["entity"] = "BOLIVIA"
        code["currency"] = "Mvdol"
        code["numeric"] = "984"
        code["unit"] = 2
    elif code["code"] == "CLF":
        code["entity"] = "CHILE"
        code["currency"] = "Unidad de Fomento"
        code["numeric"] = "990"
        code["unit"] = 4
    elif code["code"] == "COU":
        code["entity"] = "COLOMBIA"
        code["currency"] = "Unidad de Valor Real (UVR)"
        code["numeric"] = "970"
        code["unit"] = 2

    elif code["code"] == "MXV":
        code["entity"] = "MEXICO"
        code["currency"] = "Mexican Unidad de Inversion (UDI)"
        code["numeric"] = "979"
        code["unit"] = 2
    elif code["code"] == "CHW":
        code["entity"] = "SWITZERLAND(WIR Bank)"
        code["currency"] = "WIR Franc"
        code["numeric"] = "948"
        code["unit"] = 2
    elif code["code"] == "CHE":
        code["entity"] = "SWITZERLAND(WIR Bank)"
        code["currency"] = "WIR Euro"
        code["numeric"] = "947"
        code["unit"] = 2

    elif code["code"] == "USN":
        code["entity"] = "UNITED STATES"
        code["currency"] = "US Dollar"
        code["numeric"] = "997"
        code["unit"] = 2
    elif code["code"] == "UYI":
        code["entity"] = "URUGUAY"
        code["currency"] = "Uruguay Peso en Unidades Indexadas  (UI)"
        code["numeric"] = "940"
        code["unit"] = 0
    elif code["code"] == "UYW":
        code["entity"] = "URUGUAY"
        code["currency"] = "Unidad Previsional  (UP)"
        code["numeric"] = "927"
        code["unit"] = 4
    del code["entity"]
    if code["code"] not in codes_map:
        codes_map[code["code"]] = code
        codes.append(code)
    else:
        pass
"""
list_three_xls = pd.ExcelFile(r"list-three.xls")
table_historic = list_three_xls.parse(list_three_xls.sheet_names[0])

for row in table_historic.index:
    code = {}
    for i, k in enumerate(active_keys):
        code[k] = table_historic.iloc[row][i]
    if pd.isna(code["code"]):
        continue
    if code["entity"] == "ENTITY":
        continue

    code["type"] = "historic"
    code["unit"] = "-1"
    del code["entity"]
    if code["code"] not in codes_map:
        codes_map[code["code"]] = code
        codes.append(code)
    else:
        pass
f = csv.reader(open('country-codes_csv.csv', 'r'),
               delimiter=',', quotechar='"')
currency_country = {}
country_currency = {}
for x in f:
    iso3166_code = x[2]
    iso3166_numeric = x[5]
    iso4217_name = x[18]
    iso4217_code = x[22]
    if "ISO3166-1-Alpha-3" == iso3166_code:
        continue
    if iso4217_code not in currency_country:
        currency_country[iso4217_code] = []
    if iso3166_code != "":
        currency_country[iso4217_code].append(
            {"iso3166_code": iso3166_code, "iso3166_numeric": iso3166_numeric})
    if iso3166_code not in country_currency:
        country_currency[iso3166_code] = []
    if iso4217_code != "":
        country_currency[iso3166_code].append(
            {"iso4217_code": iso4217_code, "iso4217_name": iso4217_name})


prefix = """use phf::{phf_map, Map};
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
        // self.individual_languages.into_serde().unwrap();
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

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn all_active_code() -> Array {
    let mut vector: Vec<&str> = Vec::new();
    for i in 0..ALL_ACTIVE_CODE.len() {
        vector.push(ALL_ACTIVE_CODE[i].clone())
    }
    vector.into_iter().map(JsValue::from).collect()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn all_funds_code() -> Array {
    let mut vector: Vec<&str> = Vec::new();
    for i in 0..ALL_FUNDS_CODE.len() {
        vector.push(ALL_FUNDS_CODE[i].clone())
    }
    vector.into_iter().map(JsValue::from).collect()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn all_historic_code() -> Array {
    let mut vector: Vec<&str> = Vec::new();
    for i in 0..ALL_HISTORIC_CODE.len() {
        vector.push(ALL_HISTORIC_CODE[i].clone())
    }
    vector.into_iter().map(JsValue::from).collect()
}

"""
print(prefix)

for code in codes:
    if pd.isna(code["numeric"]):
        code["numeric"] = "-1"
    if code["unit"] == "N.A.":
        code["unit"] = "-1"
    code["currency"] = code["currency"].replace("\"", "\\\"")
for code in codes:
    countries = ""
    if code["code"] in currency_country:
        cs = currency_country[code["code"]]
        country_list = []
        for c in cs:
            country_list.append("\"" + c["iso3166_code"]+"\"")
        countries = ",".join(country_list)
    print("""
pub const %s: CurrencyCode = CurrencyCode {
    name: "%s",
    code: "%s",
    numeric: %s,
    unit: %s,
    code_type: "%s",
    countries: &[%s],

};
""" % (code["code"].upper(), code["currency"], code["code"].upper(), code["numeric"], code["unit"], code["type"], countries))


print("""
///CurrencyCode map with  alpha3  str Code key
pub const ALL_MAP: Map<&str, CurrencyCode> = phf_map! {
    """)
for x in codes:
    print("\"%s\" => %s, " % (x["code"].upper(), x["code"].upper()))
print("""
};
""")

exclude_map_keys = ["ALK", "AON", "ARA",
                    "ARP", "ARY", "BOP", "BRC", "BRE", "BRN", "BGK", "BGL", "BUK", "CHC", "CSJ", "GNE", "GNS", "GWP", "HRK", "ILP", "ILR", "LAJ", "LSM", "LVR", "LTT", "MTP", "MXP", "MZM", "NIC", "PEI", "PEH", "PES", "ROL", "SDP", "UGW", "UYN", "VNC", "UYP", "UGS", "UGW", "MVQ", "ZRZ", "ZWD", "ISJ", "SUR", "YUM", "YUN", "ZWC"]
print("""
///CurrencyCode map with  3 len numeric str Code key
pub const NUMERIC_MAP: Map<&str, CurrencyCode> = phf_map! {
    """)
for x in codes:
    if x["code"] in exclude_map_keys or x["numeric"] == "-1":
        continue
    print("\"%s\" => %s, " % (x["numeric"], x["code"].upper()))
print("""
};
""")

print("""
///CurrencyCode map with  3 len numeric str Code key
pub const COUNTRY_MAP: Map<&str,&'static [&'static CurrencyCode]> = phf_map! {
    """)
for x in country_currency:

    if len(country_currency[x]) == 0:
        continue
    currencies = ""
    currency_list = []
    for c in country_currency[x]:

        iso4217_code = c["iso4217_code"].upper()
        iso4217_codes = iso4217_code.split(",")
        iso4217_code = ",&".join(iso4217_codes)
        currency_list.append("&"+iso4217_code)
    currencies = ",".join(currency_list)
    print("\"%s\" => &[%s], " % (x, currencies))
print("""
};
""")


print("""
///ALL names
pub const ALL_NAME: & [ & str] = &[
    """)
for x in codes:
    print("\"%s\"," % (x["currency"]))
print("""
];
""")

print("""
///ALL codes
pub const ALL_CODE: & [ & str] = &[
    """)
for x in codes:
    print("\"%s\"," % (x["code"]))
print("""
];
""")

print("""
///ALL Active codes
pub const ALL_ACTIVE_CODE: & [ & str] = &[
    """)
for x in codes:
    if x["type"] == "currency":
        print("\"%s\"," % (x["code"]))
print("""
];
""")

print("""
///ALL Active codes
pub const ALL_FUNDS_CODE: & [ & str] = &[
    """)
for x in codes:
    if x["type"] == "funds":
        print("\"%s\"," % (x["code"]))
print("""
];
""")

print("""
///ALL Historic codes
pub const ALL_HISTORIC_CODE: & [ & str] = &[
    """)
for x in codes:
    if x["type"] == "historic":
        print("\"%s\"," % (x["code"]))
print("""
];
""")


print("""
///ALL the CurrencyCode struct
pub const ALL_NUMERIC: & [i32] = &[
""")
for x in codes:
    if x["code"] in exclude_map_keys or x["numeric"] == "-1":
        continue
    print("%s," % (x["numeric"]))
print("""
];
""")

print("""
///ALL the CurrencyCode struct
pub const ALL_NUMERIC_STR: & [ & str] = &[
""")
for x in codes:
    if x["code"] in exclude_map_keys or x["numeric"] == "-1":
        continue
    print("\"%s\"," % (x["numeric"]))
print("""
];
""")

print("""
///ALL the CurrencyCode struct
pub const ALL_CODES: & [CurrencyCode] = &[
""")
for x in codes:
    print("%s," % (x["code"].upper()))
print("""
];
""")
