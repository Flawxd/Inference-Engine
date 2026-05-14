use dioxus::{desktop::{Config, WindowBuilder, trayicon::dpi::LogicalSize}, prelude::*};
use crate::parser::parser;
use crate::types::*;
use crate::engine::*;
use crate::visualize::proof_tree_to_png;
use std::io::Cursor;
use base64::{engine::general_purpose, Engine};
use image::{Rgb, ImageBuffer};
use std::path::Path;
use dioxus_primitives::scroll_area::{ScrollArea, ScrollDirection};

pub fn window_config() -> Config {
    let main_window = WindowBuilder::new()
        .with_always_on_top(false)
		.with_title("Inference Engine")
		.with_resizable(true)
        .with_inner_size(LogicalSize::new(1200.0, 720.0))
        .with_min_inner_size(LogicalSize::new(800.0, 500.0));

    Config::default()
        .with_window(main_window)
}

#[component]
pub fn App() -> Element {
	static CSS: Asset = asset!("/assets/main.css");
	let mut to_parse = use_signal(|| "".to_string());
	let mut backwards = use_signal(|| "".to_string());
	let mut kb = use_signal(|| KnowledgeBase::new());
	let mut back_tree = use_signal(|| None);
	rsx! {
		document::Stylesheet { href: CSS }
		div { id: "title",
			h1 { "Inference Engine" }
		}
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
						    parse_button(&to_parse(), &mut kb);
						    to_parse.set(String::new());
						},
					}
					button {
						onclick: move |_| {
						    let mut kb = kb.write();
						    parse_button(&to_parse(), &mut kb);
						    to_parse.set(String::new());
						},
						"Submit"
					}
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
						                parse_button(&content, &mut kb);
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
						pre { margin: "0.5em", "{kb}" }
					}
					button {
						onclick: move |_| {
						    let mut kb = kb.write();
						    forward::forward_chain(&mut kb);
						},
						"Find all new facts"
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
				{back_tree}
			}
		}
	}
}


fn back_proof(backwards: &str, kb: KnowledgeBase) -> Option<Element> {
	if backwards.is_empty() {
		return None;
	}
	let tree = backward::backward_chain(
		&parser::parse(backwards).facts[0].term,
		&kb,
	);
	match tree {
		Some(t) => {
			let path = Path::new("./assets/back_proof_tree.png").to_str();
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

fn parse_button(to_parse: &str, kb: &mut KnowledgeBase) {
	let nb = parser::parse(&to_parse);
	nb.facts.into_iter().for_each(|f| {kb.add_fact(f);});
	nb.rules.into_iter().for_each(|r| {kb.add_rule(r);});
}
