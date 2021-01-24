extern crate gtk;
extern crate gdk_pixbuf;
extern crate glib;

use gtk::*;
use gdk_pixbuf::{Pixbuf, Colorspace};
use glib::Bytes;
use std::process;

struct App {
    window: Window,
    header: Header,
    content: Content,
}

struct Header {
    container: HeaderBar,
}

struct Content {
    area: Image,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("Pathtrace");
        Window::set_default_icon_name("iconname");

        window.add(&content.area);

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        App { window, header, content }
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title(Some("Pathtrace"));
        container.set_show_close_button(true);

        Header { container }
    }
}

impl Content {
    fn new() -> Content {
        // Create a test image
        let imagebuff = vec![0; 300];
        let buff = Pixbuf::from_bytes(&Bytes::from(&imagebuff), Colorspace::Rgb, false, 24, 10, 10, 30);
        let area = Image::from_pixbuf(Some(&buff));

        Content { area }
    }
}

fn main() {
    if gtk::init().is_err() {
        eprintln!("failed to init GTK");
        process::exit(1);
    }

    let app = App::new();

    app.window.show_all();

    gtk::main();
}
