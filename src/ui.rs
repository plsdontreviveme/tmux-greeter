use crate::command;
use crate::file;
use cursive::views::{Button, Dialog, LinearLayout};
use cursive::Cursive;

pub fn create_ui() {
    //create cursive window
    let mut siv = cursive::default();
    //create linear layout
    let linear_layout = create_linearlayout();
    //set theme
    siv.load_toml(&file::get_theme()).unwrap();
    //add keybind to quit
    siv.add_global_callback('q', Cursive::quit);
    
    //create dialog window and attach linear layout to it
    siv.add_layer(
        Dialog::around(LinearLayout::vertical()
                       .child(linear_layout))
        .title("Tmux Greeter")
    );

    //start cursive
    siv.run();
}

fn create_linearlayout() -> LinearLayout {
    //create linear layout with new window button
    let mut linear_layout = LinearLayout::vertical()
        .child(Button::new_raw("New Window", |s| command::new_window(s)));
    
    //load config file 
    let config: file::Config = file::get_config();
    
    //add directories from config file into the linear layout
    for i in 0..config.directories.paths.len() {
        let path = config.directories.paths[i].clone();
        let title = config.directories.titles[i].clone();

        linear_layout.add_child(Button::new_raw(title, move |s| command::directory(s, &path)));
    }
    
    //add executables from config file into linear layout
    for i in 0..config.executables.commands.len() {
        let title = config.executables.titles[i].clone();
        let command = config.executables.commands[i].clone();
        linear_layout.add_child(Button::new_raw(title, move |s| command::executable(s, &command)));
    }
    
    //add quit button to linear layout after the entries loaded from the config file
    linear_layout.add_child(Button::new_raw("Quit", |s| s.quit()));

    linear_layout
}
