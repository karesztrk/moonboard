use kontroll::Kontroll;
use moonsweeper::{
    model::{Keyboard, KeyboardModel::Moonlander},
    service::{Animation, Wipe},
};

#[tokio::main]
async fn main() {
    let api_result = Kontroll::new(None).await;
    Ok(())
}
