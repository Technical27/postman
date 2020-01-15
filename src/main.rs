mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = app::App::new();
    app.start()
}
