#![deny(
	absolute_paths_not_starting_with_crate,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::f32;

use egui::epaint::RectShape;
use egui::{Button, Color32, Image, Shape, Widget as _, vec2};

struct App {
	search: String,
}

impl App {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		egui_extras::install_image_loaders(&cc.egui_ctx);
		Self {
			search: String::new(),
		}
	}
}

fn search_score(haystack: &str, needle: &str) -> Option<usize> {
	let mut needle_chars = needle.chars().peekable();
	let mut score = 0;
	for haystack_ch in haystack.chars() {
		let Some(&needle_ch) = needle_chars.peek() else {
			break;
		};
		if haystack_ch == needle_ch {
			_ = needle_chars.next();
		} else {
			score += 1;
		}
	}
	if needle_chars.next().is_some() {
		return None;
	}
	Some(score)
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
			std::process::exit(1);
		}
		egui::CentralPanel::default().show(ctx, |ui| {
			let search_res = egui::TextEdit::multiline(&mut self.search)
				.desired_rows(1)
				.clip_text(true)
				.desired_width(f32::INFINITY)
				.ui(ui);
			let search_submitted = self.search.contains('\n');
			self.search.retain(|ch| ch != '\n');
			ui.memory_mut(|mem| {
				if mem.focused().is_none() {
					mem.request_focus(search_res.id);
				}
			});
			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.horizontal_wrapped(|ui| {
					let emojis = if self.search.is_empty() {
						emojis::iter()
							.enumerate()
							.map(|(i, emoji)| (emoji, i, 0))
							.collect()
					} else {
						let mut emojis = emojis::iter()
							.enumerate()
							.filter_map(|(i, emoji)| Some((emoji, i, search_score(emoji.name(), &self.search)?)))
							.collect::<Vec<_>>();
						emojis.sort_by_key(|&(_emoji, _i, score)| score);
						emojis
					};
					for (emoji, i, _score) in emojis {
						let Some(twemoji) = twemoji_assets::png::PngTwemojiAsset::from_emoji(emoji.as_str())
						else {
							continue;
						};
						let image = Image::from_bytes(format!("bytes://{i}.png`"), twemoji.data.0);
						let focus_rect_pos = ui.painter().add(Shape::Noop);
						let res = Button::image(image).frame(false).ui(ui);
						if res.has_focus() {
							let focus_rect = res
								.rect
								.expand2(vec2(ui.style().spacing.item_spacing.x / 4.0, 0.0));
							let focus_rect = RectShape::stroke(
								focus_rect,
								1.0,
								(1.0, Color32::ORANGE),
								egui::StrokeKind::Outside,
							);
							ui.painter().set(focus_rect_pos, focus_rect);
						}
						if search_submitted || res.clicked() {
							print!("{}", emoji.as_str());
							std::process::exit(0);
						}
						res.on_hover_text(emoji.name());
					}
				});
				// Ensure that scroll bar is right-aligned -- looks better.
				_ = ui.allocate_space(vec2(ui.max_rect().width(), 0.0));
			});
		});
	}
}

fn main() {
	let native_options = eframe::NativeOptions {
		centered: true,
		window_builder: Some(Box::new(|mut win| {
			win.app_id = Some("emojipicker".into());
			win.inner_size = Some(vec2(500.0, 400.0));
			win
		})),
		..Default::default()
	};
	eframe::run_native(
		"Emoji Picker",
		native_options,
		Box::new(|cc| Ok(Box::new(App::new(cc)))),
	)
	.unwrap();
	std::process::exit(1);
}
