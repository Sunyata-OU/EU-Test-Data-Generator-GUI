use eframe::egui;
use rand::thread_rng;

use eu_test_data_generator::{iban, personal_id};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "EU Test Data Generator",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

#[derive(PartialEq)]
enum Tab {
    Iban,
    PersonalId,
}

struct App {
    tab: Tab,

    // IBAN state
    iban_country: String,
    iban_count: u32,
    iban_spaces: bool,
    iban_results: Vec<IbanRow>,

    // Personal ID state
    id_country: String,
    id_count: u32,
    id_gender: GenderChoice,
    id_year: String,
    id_results: Vec<IdRow>,

    registry: personal_id::Registry,
    iban_countries: Vec<&'static str>,
    id_countries: Vec<(String, String)>,

    copied_index: Option<(Tab, usize)>,
}

#[derive(Clone)]
struct IbanRow {
    raw: String,
    formatted: String,
    valid: bool,
}

#[derive(Clone)]
struct IdRow {
    code: String,
    gender: String,
    dob: String,
    valid: bool,
}

#[derive(PartialEq)]
enum GenderChoice {
    Any,
    Male,
    Female,
}

impl App {
    fn new() -> Self {
        let registry = personal_id::Registry::new();
        let iban_countries = iban::supported_countries();
        let id_countries: Vec<(String, String)> = registry
            .list_countries()
            .iter()
            .map(|(c, n)| (c.to_string(), n.to_string()))
            .collect();

        Self {
            tab: Tab::Iban,
            iban_country: "DE".to_string(),
            iban_count: 5,
            iban_spaces: true,
            iban_results: Vec::new(),
            id_country: "EE".to_string(),
            id_count: 5,
            id_gender: GenderChoice::Any,
            id_year: String::new(),
            id_results: Vec::new(),
            registry,
            iban_countries,
            id_countries,
            copied_index: None,
        }
    }

    fn generate_ibans(&mut self) {
        let mut rng = thread_rng();
        self.iban_results.clear();
        self.copied_index = None;
        let country = if self.iban_country == "Random" {
            None
        } else {
            Some(self.iban_country.as_str())
        };
        for _ in 0..self.iban_count {
            if let Ok(code) = iban::generate_iban(country, &mut rng) {
                let valid = iban::validate_iban(&code);
                self.iban_results.push(IbanRow {
                    formatted: iban::format_iban(&code),
                    raw: code,
                    valid,
                });
            }
        }
    }

    fn generate_ids(&mut self) {
        let mut rng = thread_rng();
        self.id_results.clear();
        self.copied_index = None;
        let gender = match self.id_gender {
            GenderChoice::Any => None,
            GenderChoice::Male => Some(personal_id::date::Gender::Male),
            GenderChoice::Female => Some(personal_id::date::Gender::Female),
        };
        let year: Option<u16> = self.id_year.parse().ok();
        let opts = personal_id::GenOptions { gender, year };
        for _ in 0..self.id_count {
            if let Some(code) = self.registry.generate(&self.id_country, &opts, &mut rng)
                && let Some(parsed) = self.registry.parse(&self.id_country, &code)
            {
                self.id_results.push(IdRow {
                    code: parsed.code,
                    gender: parsed.gender.unwrap_or_default(),
                    dob: parsed.dob.unwrap_or_default(),
                    valid: parsed.valid,
                });
            }
        }
    }

    fn copy_all_to_clipboard(&self, ui: &mut egui::Ui) {
        match self.tab {
            Tab::Iban => {
                if !self.iban_results.is_empty() && ui.button("Copy all").clicked() {
                    let text: String = self
                        .iban_results
                        .iter()
                        .map(|r| {
                            if self.iban_spaces {
                                r.formatted.clone()
                            } else {
                                r.raw.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.ctx().copy_text(text);
                }
            }
            Tab::PersonalId => {
                if !self.id_results.is_empty() && ui.button("Copy all").clicked() {
                    let text: String = self
                        .id_results
                        .iter()
                        .map(|r| r.code.clone())
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.ctx().copy_text(text);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("EU Test Data Generator");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Iban, "IBAN");
                ui.selectable_value(&mut self.tab, Tab::PersonalId, "Personal ID");
            });
            ui.separator();

            match self.tab {
                Tab::Iban => {
                    ui.horizontal(|ui| {
                        ui.label("Country:");
                        egui::ComboBox::from_id_salt("iban_country")
                            .selected_text(&self.iban_country)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.iban_country,
                                    "Random".to_string(),
                                    "Random",
                                );
                                for &cc in &self.iban_countries {
                                    ui.selectable_value(&mut self.iban_country, cc.to_string(), cc);
                                }
                            });

                        ui.label("Count:");
                        ui.add(egui::DragValue::new(&mut self.iban_count).range(1..=100));

                        ui.checkbox(&mut self.iban_spaces, "Spaces");

                        if ui.button("Generate").clicked() {
                            self.generate_ibans();
                        }
                        self.copy_all_to_clipboard(ui);
                    });

                    ui.add_space(8.0);

                    let spaces = self.iban_spaces;
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("iban_grid")
                            .striped(true)
                            .num_columns(3)
                            .show(ui, |ui| {
                                ui.strong("IBAN");
                                ui.strong("Valid");
                                ui.strong("");
                                ui.end_row();
                                for (i, row) in self.iban_results.iter().enumerate() {
                                    let display = if spaces {
                                        &row.formatted
                                    } else {
                                        &row.raw
                                    };
                                    ui.monospace(display);
                                    ui.label(if row.valid { "Yes" } else { "No" });
                                    let copied =
                                        matches!(self.copied_index, Some((Tab::Iban, idx)) if idx == i);
                                    if ui
                                        .button(if copied { "Copied!" } else { "Copy" })
                                        .clicked()
                                    {
                                        ui.ctx().copy_text(if spaces {
                                            row.formatted.clone()
                                        } else {
                                            row.raw.clone()
                                        });
                                        self.copied_index = Some((Tab::Iban, i));
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                }

                Tab::PersonalId => {
                    ui.horizontal(|ui| {
                        ui.label("Country:");
                        egui::ComboBox::from_id_salt("id_country")
                            .selected_text(&self.id_country)
                            .show_ui(ui, |ui| {
                                for (code, name) in &self.id_countries {
                                    ui.selectable_value(
                                        &mut self.id_country,
                                        code.clone(),
                                        format!("{code} - {name}"),
                                    );
                                }
                            });

                        ui.label("Count:");
                        ui.add(egui::DragValue::new(&mut self.id_count).range(1..=100));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Gender:");
                        ui.selectable_value(&mut self.id_gender, GenderChoice::Any, "Any");
                        ui.selectable_value(&mut self.id_gender, GenderChoice::Male, "Male");
                        ui.selectable_value(&mut self.id_gender, GenderChoice::Female, "Female");

                        ui.label("Year:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.id_year)
                                .desired_width(50.0)
                                .hint_text("any"),
                        );

                        if ui.button("Generate").clicked() {
                            self.generate_ids();
                        }
                        self.copy_all_to_clipboard(ui);
                    });

                    ui.add_space(8.0);

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        egui::Grid::new("id_grid")
                            .striped(true)
                            .num_columns(5)
                            .show(ui, |ui| {
                                ui.strong("Code");
                                ui.strong("Gender");
                                ui.strong("Date of Birth");
                                ui.strong("Valid");
                                ui.strong("");
                                ui.end_row();
                                for (i, row) in self.id_results.iter().enumerate() {
                                    ui.monospace(&row.code);
                                    ui.label(&row.gender);
                                    ui.label(&row.dob);
                                    ui.label(if row.valid { "Yes" } else { "No" });
                                    let copied = matches!(self.copied_index, Some((Tab::PersonalId, idx)) if idx == i);
                                    if ui
                                        .button(if copied { "Copied!" } else { "Copy" })
                                        .clicked()
                                    {
                                        ui.ctx().copy_text(row.code.clone());
                                        self.copied_index = Some((Tab::PersonalId, i));
                                    }
                                    ui.end_row();
                                }
                            });
                    });
                }
            }
        });
    }
}
