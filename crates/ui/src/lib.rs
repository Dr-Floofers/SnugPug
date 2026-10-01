use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use gtk::{Align, Box, Button, CenterBox, CssProvider, Image, Label, gdk};

const APP_ID: &str = "com.drfloofers.SnugPug";

pub fn start(is_active_project: bool) {
    let app: Application = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app: &Application| build_ui(app, is_active_project));

    app.run();
}

fn build_ui(app: &Application, is_active_project: bool) {

    // === Header Bar === //

    let top_bar: CenterBox = CenterBox::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .build();
    top_bar.add_css_class("top-bar");

    let logo: Image = Image::from_file("./assets/icons/logo.svg");
    logo.set_pixel_size(35);
    logo.set_halign(Align::Start);
    logo.set_valign(Align::Center);
    logo.set_margin_top(8);
    logo.set_margin_bottom(8);
    logo.set_margin_start(8);
    top_bar.set_start_widget(Some(&logo));

    let nav_box: Box = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let new_button: Button = Button::builder()
        .label("New")
        .has_frame(false)
        .focusable(false)
        .focus_on_click(false)
        .receives_default(false)
        .vexpand(false)
        .valign(Align::Center)
        .build();
    new_button.add_css_class("flat");
    new_button.set_cursor_from_name(Some("pointer"));
    nav_box.append(&new_button);

    let open_button: Button = Button::builder()
        .label("Open")
        .has_frame(false)
        .focusable(false)
        .focus_on_click(false)
        .receives_default(false)
        .vexpand(false)
        .valign(Align::Center)
        .build();
    open_button.add_css_class("flat");
    open_button.set_cursor_from_name(Some("pointer"));
    nav_box.append(&open_button);

    top_bar.set_center_widget(Some(&nav_box));

    let window_close_button: Button = Button::from_icon_name("window-close-symbolic");
    window_close_button.set_vexpand(false);
    window_close_button.set_valign(Align::Center);
    window_close_button.set_has_frame(false);
    window_close_button.set_focusable(false);
    window_close_button.set_focus_on_click(false);
    window_close_button.set_receives_default(false);
    window_close_button.set_css_classes(&["flat"]);
    window_close_button.set_margin_end(8);
    top_bar.set_end_widget(Some(&window_close_button));

    // === Main Content === //

    let content: Box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .vexpand(true)
        .build();

    let starter_text: Label = Label::builder()
        .use_markup(true)
        .label("<b>You have no projects active right now...</b>")
        .margin_bottom(16)
        .build();

    let new_button_c: Button = Button::builder()
        .label("New")
        .has_frame(false)
        .focusable(false)
        .focus_on_click(false)
        .receives_default(false)
        .hexpand(false)
        .halign(Align::Center)
        .build();
    new_button_c.add_css_class("flat");
    new_button_c.set_cursor_from_name(Some("pointer"));

    let open_button_c: Button = Button::builder()
        .label("Open")
        .has_frame(false)
        .focusable(false)
        .focus_on_click(false)
        .receives_default(false)
        .hexpand(false)
        .halign(Align::Center)
        .build();
    open_button_c.add_css_class("flat");
    open_button_c.set_cursor_from_name(Some("pointer"));

    let button_box: Box = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .halign(Align::Center)
        .build();

    if !is_active_project {
        content.append(&Box::builder().vexpand(true).build());

        content.append(&starter_text);
        button_box.append(&new_button_c);
        button_box.append(&open_button_c);
        content.append(&button_box);

        content.append(&Box::builder().vexpand(true).build());
    }

    // === New/Open Button Code === //

    // TODO: Add functinality to new/open buttons with the file libary

    // === Root & Window Code === //

    let root: Box = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    root.append(&top_bar);
    root.append(&content);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("SnugPug")
        .default_width(400)
        .default_height(300)
        .content(&root)
        .build();

    window_close_button.connect_clicked({
        let window: ApplicationWindow = window.clone();
        move |_| {
            window.close();
        }
    });

    window.present();
}

fn load_css() {
    let provider: CssProvider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}