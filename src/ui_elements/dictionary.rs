#[derive(Default, Clone, Debug)]
pub enum Languages {
    #[default]
    En,
    Ru,
    Other(Dictionary),
}

#[derive(Clone, Debug)]
pub struct Dictionary {
    pub select_folder: String,
    pub select_file: String,
}

impl From<Languages> for Dictionary {
    fn from(value: Languages) -> Self {
        match value {
            Languages::En => Dictionary {
                select_folder: "Select folder".to_owned(),
                select_file: "Select file".to_owned(),
            },
            Languages::Ru => Dictionary {
                select_folder: "Выбрать папку".to_owned(),
                select_file: "Выбрать файл".to_owned(),
            },
            Languages::Other(other) => other,
        }
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Languages::default().into()
    }
}
