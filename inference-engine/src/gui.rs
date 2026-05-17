use dioxus::{desktop::{Config, WindowBuilder, trayicon::dpi::LogicalSize}, prelude::*};
use crate::{exporter::ExportFormat, parser::{parser, parser2}};
use crate::types::*;
use crate::engine::*;
use crate::visualize::proof_tree_to_png;
use std::io::Cursor;
use base64::{engine::general_purpose, Engine};
use image::{Rgb, ImageBuffer};
use std::path::Path;
use dioxus_primitives::{scroll_area::{ScrollArea, ScrollDirection}, radio_group::{RadioGroup, RadioItem}};
use std::hash::{Hash, DefaultHasher, Hasher};
use crate::exporter::{ExportFormat::*, export_to_file, export_knowledge_base_fmt};

pub fn window_config() -> Config {
    let main_window = WindowBuilder::new()
        .with_always_on_top(false)
		.with_title("Inference Engine")
		.with_resizable(true)
        .with_inner_size(LogicalSize::new(1250.0, 720.0))
        .with_min_inner_size(LogicalSize::new(800.0, 600.0));

    Config::default()
        .with_window(main_window)
}

#[component]
pub fn App() -> Element {
	static CSS: Asset = asset!("/assets/main.css");
	let mut to_parse = use_signal(|| String::new());
	let mut backwards = use_signal(|| String::new());
	let mut kb = use_signal(|| KnowledgeBase::new());
	let mut back_tree = use_signal(|| None);
	let mut parse_err = use_signal(|| None);
	let mut filename = use_signal(|| String::new());
	let mut export_fmt = use_signal(|| Compact);
	let mut export_err = use_signal(|| None);

	rsx! {
		document::Stylesheet { href: CSS }
		div { id: "title",
			h1 { "Inference Engine" }
		}
		div { id: "all",
			div { id: "content",
				div { id: "left",
					div { id: "parser",
						textarea {
							id: "parser_input",
							value: "{to_parse}",
							placeholder: "Enter rules and facts",
							rows: "3",
							oninput: move |event| to_parse.set(event.value()),
							onkeydown: move |key| {
							    if key.key() != Key::Enter {
							        return;
							    }
							    key.prevent_default();
							    let mut kb = kb.write();
							    parse_err.set(parse_button(&to_parse(), &mut kb));
							    to_parse.set(String::new());
							},
						}
						button {
							onclick: move |_| {
							    let mut kb = kb.write();
							    parse_err.set(parse_button(&to_parse(), &mut kb));
							    to_parse.set(String::new());
							},
							"Submit"
						}
						{parse_err}
					}
					div { id: "file_input",
						h3 { "Use a file instead" }
						input {
							r#type: "file",
							accept: "txt",
							multiple: false,
							onchange: move |e| {
							    async move {
							        let mut kb = kb.write();
							        if !e.files().is_empty() {
							            if let Ok(content) = e.files()[0].read_string().await {
							                parse_err.set(parse_button(&content, &mut kb));
							            }
							        }
							    }
							},
						}
					}
					div { id: "kb",
						h2 { "Knowledge Base" }
						ScrollArea {
							id: "kb_txt_display",
							direction: ScrollDirection::Both,
							pre { margin: "0.5em", "{export_knowledge_base_fmt(&kb(), Pretty)}" }
						}
						div { id: "kb_interact",
							button {
								onclick: move |_| {
								    let mut kb = kb.write();
								    forward::forward_chain(&mut kb);
								},
								"Find all new facts"
							}
							button { onclick: move |_| { kb.set(KnowledgeBase::new()) },
								"Clear knowledge base"
							}
						}
					}
				}
				div { id: "backwards",
					div { id: "chain",
						h2 { "Backwards chaining" }
						textarea {
							id: "backwards_input",
							placeholder: "Enter a fact you wish to prove",
							value: "{backwards}",
							oninput: move |event| {
							    backwards.set(event.value());
							},
							onkeydown: move |key| {
							    if key.key() != Key::Enter {
							        return;
							    }
							    key.prevent_default();
							    back_tree.set(back_proof(&backwards(), kb()));
							    backwards.set(String::new());
							},
						}
						button {
							onclick: move |_| {
							    back_tree.set(back_proof(&backwards(), kb()));
							    backwards.set(String::new());
							},
							"Search for this fact"
						}
					}
					div { id: "back_img", {back_tree} }
				}
			}
			div { id: "export",
				textarea {
					id: "export_input",
					placeholder: "Enter the filename to save the data",
					rows: "1",
					cols: "35",
					value: "{filename}",
					oninput: move |evt| {
					    filename.set(evt.value());
					},
					onkeydown: move |key| {
					    if key.key() != Key::Enter {
					        return;
					    }
					    key.prevent_default();
					    export_err.set(Some(export_btn(&kb(), filename(), export_fmt())));
					    filename.set(String::new());
					},
				}
				RadioGroup {
					id: "fmt_select",
					value: "option1",
					on_value_change: move |value: String| {
					    match value {
					        x if x == "Compact".to_string() => export_fmt.set(Compact),
					        _ => export_fmt.set(Prolog),
					    }
					},
					RadioItem { index: 0usize, value: "Compact", "Compact" }
					RadioItem { index: 1usize, value: "Prolog", "Prolog" }
				}
				button {
					onclick: move |_| {
					    export_err.set(Some(export_btn(&kb(), filename(), export_fmt())));
					    filename.set(String::new());
					},
					"Export the current state of the knowledge base"
				}
				{export_err}
			}
		}
	}
}


fn back_proof(backwards: &str, kb: KnowledgeBase) -> Option<Element> {
	if backwards.is_empty() {
		return None;
	}
	let term = match parse(backwards) {
		Ok(k) => {
			if k.facts.is_empty() {
			return Some(rsx! {
				p { color: "red", "please enter a fact" }
			})}
			&k.facts[0].term.clone()
		}
		Err(e) => return Some(rsx!{
			p { color: "red", "{e}" }
		})
	};
	let tree = backward::backward_chain(
		term,
		&kb,
	);
	match tree {
		Some(t) => {
			let mut h = DefaultHasher::new();
			term.to_string().hash(&mut h);
			let n = h.finish();
			let path_str = format!("./data/back_proof_tree_{}.png", n);
			let path = Path::new(&path_str).to_str();
			match proof_tree_to_png(&t, path?) {
				Ok(img) => {
					let img_src = get_img_src(img);
					Some( rsx!{
						h2 { "Proof Tree : " }
						img { src: "{img_src}" }
					})
				},
				Err(e) => Some(rsx!{
					p { color: "red", "An error happened: {e}" }
				}),
			}
		},
		None =>  Some(rsx!{
			p { color: "red", "This fact is not provable with current information" }
		})
	}
}

fn get_img_src(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
	let mut bytes: Vec<u8> = Vec::new();
	img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
	let b64 = general_purpose::STANDARD.encode(bytes);
	format!("data:image/png;base64,{b64}")
}

fn parse_button(to_parse: &str, kb: &mut KnowledgeBase) -> Option<Element> {
	let nb = match parse(to_parse) {
		Ok(k) => k,
		Err(e) => return Some(rsx!{
			p { color: "red", "{e}" }
		})
	};
	nb.facts.into_iter().for_each(|f| {kb.add_fact(f);});
	nb.rules.into_iter().for_each(|r| {kb.add_rule(r);});
	None
}

fn parse(input: &str) -> Result<KnowledgeBase, String> {
	match parser::parse(input) {
		(_, Some(_)) => parser2::parse_naturel_fr(input),
		(kb, None) => Ok(kb)
	}
}

fn export_btn(kb: &KnowledgeBase, filename: String, fmt: ExportFormat) -> Element {
	let path = format!("data/{filename}.txt");
	match export_to_file(kb, &path, fmt) {
		Ok(_) => rsx! {
			p { color: "green", "file saved at path {path}." }
		},
		Err(e) => rsx! {
			p { color: "red", "{e}" }
		},
	}
}
