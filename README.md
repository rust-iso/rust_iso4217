# rust_iso/rust_iso4217

A rust crate providing ISO 4217 support.

## What is ISO 4217

> Currency code" redirects here. Not to be confused with Currency symbol.
> ISO 4217 is a standard published by International Organization for Standardization (ISO) that defines alpha codes and numeric codes for the representation of currencies and provides information about the relationships between individual currencies and their minor units. This data is published in three tables:
> * Table A.1 – Current currency & funds code list[1](https://en.wikipedia.org/wiki/ISO_4217#cite_note-tablea1-1)
> * Table A.2 – Current funds codes[2](https://en.wikipedia.org/wiki/ISO_4217#cite_note-2)
> * Table A.3 – List of codes for historic denominations of currencies & funds[3](https://en.wikipedia.org/wiki/ISO_4217#cite_note-tablea3-3)

## Installing

``` sh
[dependencies]
rust_iso4217 = "0.1.0"
```

## Using

See [using](https://crates.io/crates/rust_iso4217) section of the documentation.

Quick guide:

``` rust
let currency = rust_iso4217::from_code("EUR");
let currency = rust_iso4217::from_numeric(360);
let currency = rust_iso4217::from_numeric_str("643");
let currencies = rust_iso4217::from_country("CHN");

println!("{:?}", rust_iso4217::ALL);
println!("{:?}", rust_iso4217::ALL_MAP);
```

For Wasm you can get all codes by 
```javascript
let code_strs = rust_iso4217.all_active_code();
let code_strs = rust_iso4217.all_funds_code();
let code_strs = rust_iso4217.all_historic_code();
```

Struct
``` rust
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
    pub code_type: &'static str, //currency,funds,historic
    pub countries: &'static [&'static str], //iso 3166 code
}
```

## Contributing

Feel free to submit a pull request or create an issue.
or request to [rust-iso](https://github.com/rust-iso) 

## License

rust-iso/rust_iso4217 is licensed under the Apache-2.0 license.

## Source(s)

* [Currency Codes](https://datahub.io/core/currency-codes#data) by [DATA HUB](https://datahub.io)
* [Country Currency Codes - ISO 4217 Standard - ISO 3166-1 Alpha 2 & 3](http://www.contactlesspaymentcards.com/country-currency-codes.php) by [Contactless Payment Cards](http://www.wikipedia.org)
* [ISO 4217](https://en.wikipedia.org/wiki/ISO_4217) by [Wikipedia](http://www.wikipedia.org)
* [www.iso.org](http://www.iso.org)
