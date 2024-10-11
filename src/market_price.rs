use rust_decimal::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PriceResponse {
    data: PriceRate,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct PriceRate {
    rateUsd: String,
}

pub fn get_market_price(asset: &str) -> Result<Decimal, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!("https://api.coincap.io/v2/rates/{}", asset))?;
    let body = resp.json::<PriceResponse>()?;

    let price = match Decimal::from_str(&body.data.rateUsd) {
        Ok(num) => num,
        Err(_) => {
            println!("Erro na conversão do preço");
            let a = Decimal::new(0, 1);
            a
        }
    };

    Ok(price)
}
