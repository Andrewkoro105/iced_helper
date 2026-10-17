//! A wrapper for `native_dialog::FileDialog` that allows you to quickly and easily request the path to a folder or file

use iced::{Task, advanced::graphics::futures::MaybeSend};
use rfd::{AsyncFileDialog, FileHandle};
use std::path::{Path, PathBuf};

/// Specifies the purpose of the requested path (the save path does not have to exist)
pub enum TypeAction {
    Save,
    Open,
}

/// Type of requested path.
/// When selecting a file, you need to specify the purpose of the path and which files are suitable (first by specifying the file type and then the extensions that match it)
pub enum FileTypes {
    File(TypeAction, Vec<(&'static str, &'static [&'static str])>),
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
///     FileTypes::File(
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
///     PathBuf::new(),
///     "",
/// );
/// ```
pub fn select_file<F, M, FM>(
    message: F,
    file_types: FileTypes,
    directory: PathBuf,
    file_name: impl Into<String> + Send + 'static,
) -> Task<M>
where
    FM: 'static,
    F: Fn(PathBuf) -> FM + Clone + Send + 'static,
    M: From<FM> + MaybeSend + 'static,
{
    Task::future(async {
        let mut file_dialog = AsyncFileDialog::new()
            .set_directory(directory)
            .set_file_name(file_name);
        let file = match file_types {
            FileTypes::File(type_action, items) => {
                for (name, extensions) in items {
                    file_dialog = file_dialog.add_filter(name, extensions);
                }
                match type_action {
                    TypeAction::Save => file_dialog.save_file().await,
                    TypeAction::Open => file_dialog.pick_file().await,
                }
            }
            FileTypes::Dir => file_dialog.pick_folder().await,
        };

        file.as_ref()
            .map(FileHandle::path)
            .map(Path::to_path_buf)
            .map(message)
            .map(Into::<M>::into)
    })
    .and_then(|m| Task::done(m))
}
