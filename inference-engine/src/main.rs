use inference_engine::gui;

fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(gui::window_config())
        .launch(gui::App);
}
