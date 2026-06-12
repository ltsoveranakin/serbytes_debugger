use eframe::egui::Ui;
use egui_extras::{Column, TableBuilder};
use serbytes::prelude::ReadByteBufferOwned;
use std::cell::OnceCell;
use std::path::PathBuf;
use std::{fs, io};

const CELL_SIZE: f32 = 25.0;
const CELL_COLUMNS: usize = 8;

#[derive(Default)]
pub(super) struct BinaryPanel {
    pub(super) binary_file: Option<BinaryFile>,
}

impl BinaryPanel {
    pub(super) fn render(&mut self, ui: &mut Ui) {
        if ui.button("Pick file").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.binary_file = Some(BinaryFile::new(path));
            }
        }

        ui.separator();

        if let Some(binary_file) = &self.binary_file {
            binary_file.render(ui);
        }
    }
}

#[derive(Default)]
pub(super) struct BinaryFile {
    path: PathBuf,
    buffer: OnceCell<io::Result<ReadByteBufferOwned>>,
}

impl BinaryFile {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            buffer: OnceCell::new(),
        }
    }

    pub(super) fn get_buffer(&self) -> &io::Result<ReadByteBufferOwned> {
        self.buffer
            .get_or_init(|| fs::read(&self.path).map(|vec| ReadByteBufferOwned::from_vec(vec)))
    }

    fn render(&self, ui: &mut Ui) {
        let buf = if let Ok(rbb) = self.get_buffer() {
            rbb.buf()
        } else {
            ui.label("Unable to read file");

            return;
        };

        let buf_len = buf.len();

        let plur = if buf_len == 1 { "" } else { "s" };

        ui.label(format!("{} byte{}", buf_len, plur));

        TableBuilder::new(ui)
            .columns(Column::exact(CELL_SIZE), CELL_COLUMNS)
            .body(|body| {
                body.rows(CELL_SIZE, buf_len / CELL_COLUMNS, |mut row| {
                    let row_index = row.index() * CELL_COLUMNS;

                    let row_bytes = &buf[row_index..(row_index + CELL_COLUMNS).min(buf_len)];

                    for byte in row_bytes {
                        row.col(|ui| {
                            ui.label(byte.to_string());
                        });
                    }
                })
            });
    }
}
