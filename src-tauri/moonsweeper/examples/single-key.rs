use kontroll::Kontroll;
use moonsweeper::{
    model::{Keyboard, KeyboardModel::Moonlander},
    service::{Animation, SingleKey },
};

#[tokio::main]
async fn main() {
    let api_result = Kontroll::new(None).await;
    match api_result {
        Ok(_) => (),
        Err(_) => println!("Keymapp is not running"),
    }

    let api = api_result.unwrap();

    let moonlander = Keyboard::new(Moonlander);
    let animation = SingleKey::new(moonlander,  'y');
    animation.run(&api).await;
}
