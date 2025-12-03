// Disable command prompt on launch for Windows
#![windows_subsystem = "windows"]

mod application_data;
mod get_endpoints;
mod main_application;
mod ui;

#[cfg(not(test))]
use env_logger::{self, Env};
use main_application::MainApplication;

/// Sets up the logger for output, depending
/// on whether we are in a debug context
/// or not.
///
/// Debug contexts will log up through "trace".<br>
/// Release contexts will log up through "warn".
#[cfg(not(test))]
fn initialize_logger() {
    #[cfg(debug_assertions)]
    env_logger::init_from_env(Env::default().default_filter_or("trace"));

    #[cfg(not(debug_assertions))]
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
}

#[cfg(not(test))]
#[tokio::main]
async fn main() -> gtk4::glib::ExitCode {
    initialize_logger();

    let handle = tokio::spawn(get_endpoints::run_webserver());

    let app = MainApplication::new();
    let exitcode = app.run_application();

    handle.abort();

    exitcode
}

// ---------------
// ---- TESTS ----
// ---------------

#[cfg(test)]
mod tests {
    use crate::MainApplication;
    use gtk_tester::create_test;

    create_test! {
        test_if_app_opens,
        MainApplication,
        |_win| {

        }
    }
}
