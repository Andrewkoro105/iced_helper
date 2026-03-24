use iced::{Task, advanced::graphics::futures::MaybeSend};
use native_dialog::FileDialog;
use std::path::PathBuf;

pub enum TypeAction {
    Save,
    Open,
}

pub enum FileTypes {
    #[allow(dead_code)]
    Files(TypeAction, Vec<(&'static str, &'static [&'static str])>),
    Dir,
}

pub fn select_file<F, M, FM>(message: F, file_types: FileTypes) -> Task<M>
where
    F: Fn(PathBuf) -> FM + Clone,
    M: From<FM> + MaybeSend + 'static,
{
    let mut dialog = FileDialog::new().set_location("~/Desktop");
    let path = match file_types {
        FileTypes::Files(type_action, file_types) => {
            for (file_type, extensions) in file_types {
                dialog = dialog.clone().add_filter(file_type, extensions);
            }
            match type_action {
                TypeAction::Save => dialog.show_save_single_file(),
                TypeAction::Open => dialog.show_open_single_file(),
            }
            
        }
        FileTypes::Dir => dialog.show_open_single_dir(),
    };

    if let Some(str) = path.ok().flatten() {
        Task::done(message(str).into())
    } else {
        Task::none()
    }
}
