use slint::{Timer, TimerMode};
slint::include_modules!();
mod market_price;

fn update_price(ui_handle: slint::Weak<AppWindow>, currency: &str, set_price_fn: impl Fn(&AppWindow, slint::SharedString) + 'static) {
    if let Some(ui) = ui_handle.upgrade() {
        match market_price::get_market_price(currency) {
            Ok(price) => {
                let formatted_price = format!("{:.2}", price);
                set_price_fn(&ui, slint::SharedString::from(formatted_price));
            }
            Err(e) => {
                println!("Falha ao obter a cotação de {}: {}", currency, e);
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle_btc = ui.as_weak();
    let ui_handle_eth = ui.as_weak();

    // Função para atualizar BTC
    let update_btc = {
        let ui_handle_btc = ui_handle_btc.clone();
        move || {
            update_price(ui_handle_btc.clone(), "bitcoin", |ui, price| {
                ui.set_btcPrice(price.clone());
            });
        }
    };

    // Função para atualizar ETH
    let update_eth = {
        let ui_handle_eth = ui_handle_eth.clone();
        move || {
            update_price(ui_handle_eth.clone(), "ethereum", |ui, price| {
                ui.set_ethPrice(price.clone());
            });
        }
    };

    // Chamar as funções de atualização imediatamente na inicialização
    update_btc();
    update_eth();

    // Configura o timer para chamar as funções de atualização periodicamente
    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_secs(5), {
        let update_btc = update_btc.clone();
        let update_eth = update_eth.clone();
        move || {
            update_btc();
            update_eth();
        }
    });

    // Liga as callbacks dos botões à função de atualização
    ui.on_request_update_btc({
        let update_btc = update_btc.clone();
        move || {
            update_btc();
        }
    });

    ui.on_request_update_eth({
        let update_eth = update_eth.clone();
        move || {
            update_eth();
        }
    });

    ui.run()
}
