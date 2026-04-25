//! A wrapper for `native_dialog::FileDialog` that allows you to quickly and easily request the path to a folder or file

use iced::{Task, advanced::graphics::futures::MaybeSend};
use native_dialog::FileDialog;
use std::path::PathBuf;

/// Specifies the purpose of the requested path (the save path does not have to exist)
pub enum TypeAction {
    Save,
    Open,
}

/// Type of requested path.
/// When selecting a file, you need to specify the purpose of the path and which files are suitable (first by specifying the file type and then the extensions that match it)
pub enum FileTypes {
    Files(TypeAction, Vec<(&'static str, &'static [&'static str])>),
    Dir,
}

/// A wrapper for `native_dialog::FileDialog` that allows you to quickly and easily request the path to a folder or file
/// ```no_run
/// use iced_helper::ui_elements::select_file::{
///     select_file,
///     FileTypes,
///     TypeAction,
/// };
/// use std::path::PathBuf;
/// 
/// 
/// #[derive(Clone)]
/// enum Message {
///     SetPathParam(PathBuf),
/// }
/// 
/// select_file::<_, Message, Message>(
///     Message::SetPathParam,
///     FileTypes::Files(
///         TypeAction::Save,
///         vec![
///             (
///                 "image",
///                 &[
///                     "avif", "bmp", "dds", "exr", "ff", "gif", "hdr", "ico", "jpeg",
///                     "png", "pnm", "qoi", "tga", "tiff", "webp", "raw",
///                 ],
///             ),
///             ("text", &["txt"]),
///         ],
///     ),
/// );
/// ```
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
