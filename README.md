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
let country = rust_iso3166::from_alpha2("AU");
let country = rust_iso3166::from_alpha3("AUS");
let country = rust_iso3166::from_numeric(036);
let country = rust_iso3166::from_numeric_str("036");

println!("{:?}", rust_iso3166::ALL);

println!("{:?}", rust_iso3166::ALL_ALPHA2);   
println!("{:?}", rust_iso3166::ALL_ALPHA3);   
println!("{:?}", rust_iso3166::ALL_NAME);   
println!("{:?}", rust_iso3166::ALL_NUMERIC);   
println!("{:?}", rust_iso3166::ALL_NUMERIC_STR);   

println!("{:?}", rust_iso3166::NUMERIC_MAP);  
println!("{:?}", rust_iso3166::ALPHA3_MAP);  
println!("{:?}", rust_iso3166::ALPHA2_MAP);  

// for ISO 3166-2
let country = rust_iso3166::from_alpha2("GB").unwrap();
let subdivisions = country.subdivisions();
assert!(subdivisions.unwrap().len() > 0);
let country = rust_iso3166::iso3166_2::from_code("GB-EDH");
assert_eq!("Edinburgh, City of", country.unwrap().name); 

// for ISO 3166-3
let sub = rust_iso3166::iso3166_3::from_code("PZPA");
assert_eq!("Panama Canal Zone", sub.unwrap().name);
```

Data sample:

``` rust
CountryCode { 
    name: "Australia",
    alpha2: "AU", 
    alpha3: "AUS", 
    numeric: 36 
}

 iso3166_2::Subdivision {
    name: "Bādghīs",
    code: "AF-BDG",
    subdivision_type: "Province",
    country_name: "Afghanistan",
    country_code: "AF",
    region_code: "AF-BDG",
}

iso3166_3::CountryCode3 {
    code: "BQAQ",
    name: "British Antarctic Territory",
    former: CountryCode { 
        name: "British Antarctic Territory",
        alpha2: "BQ", 
        alpha3: "ATB", 
        numeric: 0 
    },
    new_countries: [
        CountryCode { 
            name: "Antarctica",
            alpha2: "AQ", 
            alpha3: "ATA", 
            numeric: 010
        },    
    ],
    validity: [1974,1979],
    desc: "Merged into Antarctica",
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
