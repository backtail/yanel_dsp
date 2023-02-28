#[macro_use]
extern crate num_derive;

mod gtk_app_deps;
mod gtk_modules;

use {
    audio_module::{AudioModule, Widget},
    gtk::{prelude::*, Orientation, Window, WindowPosition, WindowType},
    gtk_app_deps::*,
    gtk_modules::simple_delay::SimpleDelayModule,
};

fn main() {
    run_main::<SimpleDelayModule>();
}

fn run_main<Module: AudioModule>() {
    if gtk::init().is_err() {
        println!("Error initializing GTK");
        return;
    }

    let (command_sender, command_receiver) = crossbeam_channel::bounded(1024);

    let sample_rate = 48_000;
    let _audio_streams = audio_thread::start_audio::<Module>(command_receiver, sample_rate)
        .expect("Failed to start audio");

    let window = Window::new(WindowType::Toplevel);
    window.set_title("simple delay");
    window.set_default_size(230, 300);
    window.set_position(WindowPosition::Center);
    window.set_resizable(false);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let container = gtk::Box::new(Orientation::Horizontal, 4);
    window.add(&container);

    for id in 0..Module::parameter_count() {
        let parameter = Module::parameter(id);
        let widget = match parameter.widget() {
            Widget::Slider => {
                gtk_parameter_slider::make_slider(parameter, id, command_sender.clone())
            }
            Widget::Button => {
                gtk_parameter_toggle::make_toggle(parameter, id, command_sender.clone())
            }
        };
        container.pack_start(&widget, false, true, 5);
    }
    window.show_all();
    gtk::main();
}
