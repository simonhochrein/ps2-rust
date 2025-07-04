use crate::tabs::Tab;
use crate::{AppState, VirtualFile};
use eframe::egui::{ComboBox, Id, Ui};
use ps2_filetypes::IconSys;
use std::path::PathBuf;

pub struct IconSysViewer {
    title: String,
    file: String,
    pub icon_file: String,
    pub icon_copy_file: String,
    pub icon_delete_file: String,
}

impl IconSysViewer {
    pub fn new(file: VirtualFile) -> Self {
        let buf = std::fs::read(&file.file_path).expect("File not found");

        let sys = IconSys::new(buf);

        Self {
            title: sys.title.clone(),
            icon_file: sys.icon_file.clone(),
            icon_copy_file: sys.icon_copy_file.clone(),
            icon_delete_file: sys.icon_delete_file.clone(),
            file: file
                .file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui, app: &mut AppState) {
        let files: Vec<String> = app
            .files
            .iter()
            .filter_map(|file| {
                let name = file.name.clone();
                if matches!(
                    PathBuf::from(&name)
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap_or(""),
                    "icn" | "ico"
                ) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        ui.centered_and_justified(|ui| {
            eframe::egui::Grid::new(Id::from("IconSysEditor"))
                .num_columns(2)
                .show(
                    ui,
                    |ui| {
                        ui.label("Title");
                        ui.text_edit_singleline(&mut self.title);
                        ui.end_row();
                        ui.label("Icon");
                        file_select(ui, "list_icon", &mut self.icon_file, &files);
                        ui.end_row();
                        ui.label("Copy Icon");
                        file_select(ui, "copy_icon", &mut self.icon_copy_file, &files);
                        ui.end_row();
                        ui.label("Delete Icon");
                        file_select(ui, "delete_icon", &mut self.icon_delete_file, &files);
                        ui.end_row();
                    },
                );
        });
    }
}

impl Tab for IconSysViewer {
    fn get_id(&self) -> &str {
        &self.file
    }

    fn get_title(&self) -> String {
        self.file.clone()
    }

    fn get_modified(&self) -> bool {
        false
    }

    fn save(&mut self) {
        todo!()
    }
}

fn file_select(ui: &mut Ui, name: impl Into<String>, value: &mut String, files: &[String]) {
    ComboBox::from_id_salt(name.into())
        .selected_text(&*value)
        .show_ui(ui, |ui| {
            files.iter().for_each(|file| {
                ui.selectable_value(value, file.clone(), file.clone());
            });
        });
}
