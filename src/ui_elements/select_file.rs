use crate::ui_elements::updated::Updated;
use native_dialog::FileDialog;
use std::path::PathBuf;

pub enum FileTypes {
    #[allow(dead_code)]
    Files(Vec<(&'static str, &'static [&'static str])>),
    Dir,
}

pub fn select_file<F, M, FM>(updated: &mut impl Updated<M>, message: F, file_types: FileTypes)
where
    F: Fn(PathBuf) -> FM + Clone,
    M: From<FM>,
{
    let mut dialog = FileDialog::new().set_location("~/Desktop");
    let path = match file_types {
        FileTypes::Files(file_types) => {
            for (file_type, extensions) in file_types {
                dialog = dialog.clone().add_filter(file_type, extensions);
            }
            dialog.show_open_single_file()
        }
        FileTypes::Dir => dialog.show_open_single_dir(),
    };

    if let Some(str) = path.ok().flatten() {
        updated.update(message(str).into());
    }
}
