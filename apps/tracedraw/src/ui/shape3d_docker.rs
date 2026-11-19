//! The 3D Revolve and Inflate docker (Effects > 3D Revolve and Inflate):
//! the mode, Revolve's facets, tilt, turn, sweep and axis side, Inflate's
//! depth, the light and shine, the art (Use Selection, place and size),
//! Keep original and a full-width Apply.

use crate::app::App;
use crate::i18n::{tr, trf};
use crate::shape3d::Mode;
use crate::ui::docker_form::{indented, row, section};
use crate::ui::tips::RichTip;
use egui::Ui;

fn slider(ui: &mut Ui, v: &mut f64, lo: f64, hi: f64, key: &str) {
    ui.add(crate::ui::Slider::new(v, lo..=hi).max_decimals(0))
        .rich_tip(crate::ui::tips::keyed(key));
}

pub fn shape_3d(app: &mut App, ui: &mut Ui) {
    ui.spacing_mut().slider_width = 110.0;
    let mut pick = false;
    {
        let s = &mut app.shape_3d;
        row(ui, &tr("docker.shape3d_mode"), |ui| {
            for (m, key) in [
                (Mode::Revolve, "docker.shape3d_revolve"),
                (Mode::Inflate, "docker.shape3d_inflate"),
            ] {
                ui.selectable_value(&mut s.mode, m, tr(key))
                    .rich_tip(crate::ui::tips::keyed(key));
            }
        });
        section(ui, None);
        match s.mode {
            Mode::Revolve => {
                row(ui, &tr("docker.shape3d_segments"), |ui| {
                    let mut n = s.segments;
                    if ui
                        .add(crate::ui::field::NumField::new(&mut n).range(3..=256))
                        .rich_tip(crate::ui::tips::keyed("docker.shape3d_segments"))
                        .changed()
                    {
                        s.segments = n;
                    }
                });
                row(ui, &tr("docker.shape3d_tilt"), |ui| {
                    slider(ui, &mut s.tilt, -80.0, 80.0, "docker.shape3d_tilt")
                });
                row(ui, &tr("docker.shape3d_turn"), |ui| {
                    slider(ui, &mut s.turn, 0.0, 360.0, "docker.shape3d_turn")
                });
                row(ui, &tr("docker.shape3d_sweep"), |ui| {
                    slider(ui, &mut s.sweep, 10.0, 360.0, "docker.shape3d_sweep")
                });
                indented(ui, |ui| {
                    ui.checkbox(&mut s.axis_right, tr("docker.shape3d_axis_right"))
                        .rich_tip(crate::ui::tips::keyed("docker.shape3d_axis_right"));
                });
            }
            Mode::Inflate => {
                row(ui, &tr("docker.shape3d_depth"), |ui| {
                    slider(ui, &mut s.depth, 10.0, 200.0, "docker.shape3d_depth")
                });
            }
        }
        section(ui, None);
        row(ui, &tr("docker.shape3d_light_angle"), |ui| {
            slider(
                ui,
                &mut s.light_angle,
                0.0,
                360.0,
                "docker.shape3d_light_angle",
            )
        });
        row(ui, &tr("docker.shape3d_light_height"), |ui| {
            slider(
                ui,
                &mut s.light_height,
                0.0,
                90.0,
                "docker.shape3d_light_height",
            )
        });
        row(ui, &tr("docker.shape3d_shine"), |ui| {
            slider(ui, &mut s.shine, 0.0, 100.0, "docker.shape3d_shine")
        });
        section(ui, None);
    }
    row(ui, &tr("docker.shape3d_art"), |ui| {
        pick = ui
            .add_enabled(
                !app.selection.is_empty(),
                egui::Button::new(tr("docker.scatter_use_selection")),
            )
            .rich_tip(crate::ui::tips::keyed("docker.shape3d_art"))
            .rich_disabled_tip(crate::ui::tips::keyed("docker.shape3d_art"))
            .clicked();
        if !app.shape_3d.art.is_empty() {
            ui.label(trf(
                "docker.scatter_picked_count",
                &[("n", &app.shape_3d.art.len().to_string())],
            ));
            if ui
                .small_button("×")
                .rich_tip(tr("docker.shape3d_art_clear"))
                .clicked()
            {
                app.shape_3d.art.clear();
            }
        }
    });
    if pick && !app.pick_3d_art() {
        app.status = tr("status.scatter_no_symbols");
    }
    {
        let s = &mut app.shape_3d;
        if !s.art.is_empty() {
            row(ui, &tr("docker.shape3d_art_u"), |ui| {
                slider(ui, &mut s.art_u, 0.0, 100.0, "docker.shape3d_art_u")
            });
            row(ui, &tr("docker.shape3d_art_v"), |ui| {
                slider(ui, &mut s.art_v, 0.0, 100.0, "docker.shape3d_art_v")
            });
            row(ui, &tr("docker.shape3d_art_size"), |ui| {
                slider(ui, &mut s.art_size, 1.0, 100.0, "docker.shape3d_art_size")
            });
        }
        indented(ui, |ui| {
            ui.checkbox(&mut s.keep_original, tr("docker.shape3d_keep"))
                .rich_tip(crate::ui::tips::keyed("docker.shape3d_keep"));
        });
    }
    let tip = crate::ui::tips::pair("docker.apply", "docker.shape3d_apply_desc");
    let ready = !app.selection.is_empty();
    if crate::ui::docker_form::wide_button(ui, &tr("docker.apply"), ready, &tip) {
        let s = app.shape_3d.clone();
        app.apply_3d(&s);
    }
}
