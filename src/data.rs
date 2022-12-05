use druid::
    {
        AppLauncher,
        WindowDesc,
        Widget,
        Data,
        Lens,
        im::Vector,
        WidgetExt,
        Env,
    };
use druid::widget::
    {
        Label,
        Scroll,
        List,
    };

use crate::components::filetree::FileTree;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    name: String, 
    file: FileTree,
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            name: "".into(),
            file: FileTree {},
        }
    }
}
pub fn build_root() -> impl Widget<AppState> {
    let label = Label::new(|data: &AppState, _env: &Env| 
        data.name.clone());
    label
}

