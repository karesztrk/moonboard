use kontroll::Kontroll;
use moonsweeper::{
    model::{Keyboard, KeyboardLayout::Norman, KeyboardModel::Moonlander},
    service::{Animation, Torpedo},
};

#[tokio::main]
async fn main() {
    let api_result = Kontroll::new(None).await;
    match api_result {
        Ok(_) => (),
        Err(_) => println!("Keymapp is not running"),
    }

    let api = api_result.unwrap();

    let moonlander = Keyboard::new(Moonlander, Norman);
    let animation = Torpedo::new(moonlander, 'p');
    animation.run(&api).await;
    animation.clean(&api).await;
}
