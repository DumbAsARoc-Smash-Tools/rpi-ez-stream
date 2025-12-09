mod slint_import;
mod css_module;
mod settings;

use slint_import::*;

fn open_css_window(player_id: PlayerType) {
    let css = EZStreamCharacterSelectScreen::new().unwrap();
    css.set_player_to_decide_for(player_id);
    css.show().unwrap();
}

fn main() -> anyhow::Result<()> {
    let app = MainWindow::new()?;

    app.on_player_1_tag_changed(|new_tag| {
        println!("Player 1 Tag changed: {new_tag}");
    });

    app.on_player_2_tag_changed(|new_tag| {
        println!("Player 2 Tag changed: {new_tag}");
    });

    app.on_player_1_score_changed(|new_score| {
        println!("Player 1 Score changed: {new_score}");
    });

    app.on_player_2_score_changed(|new_score| {
        println!("Player 2 Score changed: {new_score}");
    });

    app.on_player_1_css_button_clicked(|| {
        open_css_window(PlayerType::Player1);
    });

    app.on_player_2_css_button_clicked(|| {
        open_css_window(PlayerType::Player2);
    });

    app.show()?;

    Ok(slint::run_event_loop()?)
}
