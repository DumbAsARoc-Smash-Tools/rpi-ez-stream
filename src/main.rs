slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = MainWindow::new()?;

    app.on_player_1_tag_changed(|new_tag| {
        println!("Player 1 Tag changed: {new_tag}");
    });

    app.on_player_2_tag_changed(|new_tag| {
        println!("Player 2 Tag changed: {new_tag}");
    });

    Ok(app.run()?)
}