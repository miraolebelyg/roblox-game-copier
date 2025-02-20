use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};

pub fn create_gui() {
    let application = gtk::Application::new(Some("com.example.roblox_game_copier"), Default::default());
    application.connect_activate(|app| {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Roblox Game Copier");
        window.set_default_size(350, 70);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let entry = Entry::new();
        let button = Button::with_label("Copy Game");
        let label = Label::new(None);

        button.connect_clicked(move |_| {
            let game_id = entry.get_text().unwrap();
            let result = copy_game(&game_id);
            label.set_text(&result);
        });

        vbox.pack_start(&entry, true, true, 0);
        vbox.pack_start(&button, true, true, 0);
        vbox.pack_start(&label, true, true, 0);
        window.add(&vbox);
        window.show_all();
    });
    application.run();
}

fn copy_game(game_id: &str) -> String {
    let game = fetch_game(game_id);
    let data = download_game(&game);
    save_game(&game, data);
    format!("Game {} copied successfully.", game.name)
}

fn fetch_game(game_id: &str) -> Game {
    let client = Client::new();
    let response: Game = client.get(&format!("https://api.roblox.com/games/{}", game_id))
        .send()
        .unwrap()
        .json()
        .unwrap();
    response
}