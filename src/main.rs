extern crate imlib_rs;

use clap::Parser;
use std::{fs, path::Path};
use x11::xlib::{Pixmap, Window};

#[derive(Debug)]
struct Monitor {
    root: Window,
    pixmap: Pixmap,
    width: usize,
    height: usize,
    render_context: imlib_rs::Imlib_Context,
}

#[derive(Parser, Debug)]
struct CliImagePath {
    #[arg(
        short,
        long,
        help = "Path to the directory containing the bitmap images"
    )]
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CliImagePath::parse();

    let image_dir = Path::new(&args.path);
    let total_images = fs::read_dir(image_dir)
        .expect("Failed to open bitmap directory")
        .count();

    unsafe {
        let display = x11::xlib::XOpenDisplay(std::ptr::null());

        let screen_count = x11::xlib::XScreenCount(display);

        let mut monitors: Vec<Monitor> = Vec::with_capacity(screen_count as usize);

        for current_screen in 0..=screen_count {
            let width = x11::xlib::XDisplayWidth(display, current_screen);
            let height = x11::xlib::XDisplayHeight(display, current_screen);
            let depth = x11::xlib::XDefaultDepth(display, current_screen);
            let visual = x11::xlib::XDefaultVisual(display, current_screen);
            let cm = x11::xlib::XDefaultColormap(display, current_screen);
            let root = x11::xlib::XRootWindow(display, current_screen);
            let pixmap =
                x11::xlib::XCreatePixmap(display, root, width as u32, height as u32, depth as u32);

            monitors.push(Monitor {
                root,
                pixmap,
                width: width as usize,
                height: height as usize,
                render_context: imlib_rs::imlib_context_new(),
            });

            imlib_rs::imlib_context_push(monitors[current_screen as usize].render_context);
            imlib_rs::imlib_context_set_display(display as *mut imlib_rs::_XDisplay);
            imlib_rs::imlib_context_set_visual(visual.cast());
        }

        println!("{:#?}", monitors);
    }

    Ok(())
}