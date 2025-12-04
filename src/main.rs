slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = MainWindow::new()?;

    Ok(app.run()?)
}