use druid::
    {
        AppLauncher,
        WindowDesc,
    };

mod components;
mod data;

use crate::data::{ AppState, build_root };

fn main() {
    let main_window = WindowDesc::new(build_root)
        .title("Lab Soup")
        .window_size((400.,600.));

    AppLauncher::with_window(main_window)
        .launch(AppState::new())
        .expect("bad stuff");
}
