use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct AppState {
    screenshot_path: String,
}

fn build_ui() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _: &Env| {
        format!("Screenshot Path: {}", data.screenshot_path)
    });

    let button = Button::new("Capture Screenshot")
        .on_click(|_, data: &mut AppState, _| {
            // Aggiungi qui la logica per la cattura dello schermo
            data.screenshot_path = "/path/to/screenshot.png".to_string(); // Sostituisci con il percorso effettivo
        });

    Flex::column()
        .with_child(label)
        .with_spacer(10.0)
        .with_child(button)
        .padding(20.0)
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title(LocalizedString::new("Screen Capture Utility"))
        .window_size((400.0, 200.0));

    let initial_state = AppState {
        screenshot_path: String::new(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Failed to launch application");
}
