#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use eframe::{
    egui::{Button, CentralPanel, Context, ScrollArea, SidePanel, Visuals},
    epaint::{mutex::RwLock, Stroke},
    App, CreationContext, Frame,
};
use raytracer::{color, vector3, Material, Scene, Sky, Sphere};

struct RaytracingGui {
    scene: Arc<RwLock<Scene>>,
    selected: Option<usize>,
}

impl App for RaytracingGui {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {});
        SidePanel::left("scene").show(ctx, |ui| {
            ui.heading("Scene");
            ui.separator();

            let spacing = ui.spacing().interact_size.y;
            ScrollArea::vertical().show_rows(
                ui,
                spacing,
                self.scene.read().objects.len(),
                |ui, rows| {
                    for row in rows {
                        let selected = self.selected.is_some() && self.selected.unwrap() == row;

                        let response = ui.add_sized(
                            (ui.available_width(), spacing),
                            Button::new(self.scene.read().objects[row].name()).stroke(
                                if selected {
                                    Stroke::new(2.0, ctx.style().visuals.hyperlink_color)
                                } else {
                                    Stroke::none()
                                },
                            ),
                        );

                        if response.clicked() {
                            self.selected = Some(row)
                        }
                    }
                },
            );
        });

        if let Some(idx) = self.selected {
            SidePanel::right("inspector").show(ctx, |ui| {
                ui.heading("Inspector");
                ui.separator();
            });
        }
    }
}

impl RaytracingGui {
    fn new() -> Self {
        Self {
            selected: None,
            scene: Arc::new(RwLock::new(Scene {
                sky: Sky {
                    top: color(0.5, 0.7, 1.0),
                    bottom: color(1.0, 1.0, 1.0),
                },
                objects: vec![
                    Box::new(Sphere {
                        center: vector3(0.0, 0.0, -1.0),
                        radius: 0.5,
                        material: Material::Lambertian {
                            albedo: color(1.0, 0.3, 0.3),
                        },
                    }),
                    Box::new(Sphere {
                        center: vector3(0.0, 0.0, -1.0),
                        radius: 0.5,
                        material: Material::Lambertian {
                            albedo: color(1.0, 0.3, 0.3),
                        },
                    }),
                ],
            })),
        }
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "raytracing gui",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(match dark_light::detect() {
                dark_light::Mode::Dark => Visuals::dark(),
                dark_light::Mode::Light => Visuals::light(),
            });
            Box::new(RaytracingGui::new())
        }),
    );
}
