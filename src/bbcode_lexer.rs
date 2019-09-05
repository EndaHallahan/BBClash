use rctree::Node;
use phf::{phf_map, phf_set};
use super::Instruction; 
use super::GroupType;
use super::ASTElement;

/// Struct for lexing BBCode Instructions into an ASTElement tree.
pub struct BBCodeLexer {
	current_node: Node<ASTElement>,
	anchor: Node<ASTElement>
}
impl BBCodeLexer {
	/// Creates a new BBCodeLexer.
	pub fn new() -> BBCodeLexer {
		let anchor = Node::new(ASTElement::new(GroupType::Anchor));
		let current_node = Node::new(ASTElement::new(GroupType::Document));
		BBCodeLexer{current_node, anchor}
	}
	/// Lexes a vector of Instructions.
	pub fn lex(&mut self, instructions: &Vec<Instruction>) -> Node<ASTElement> {
		self.anchor.append(Node::new(ASTElement::new(GroupType::Document)));
		self.current_node = self.anchor.first_child().unwrap();
		self.new_group(GroupType::Paragraph);
		for instruction in instructions {
			self.execute(instruction);
		}
		self.end_group(GroupType::Paragraph);
		self.current_node.root()
	}
	/// Matches Instruction types.
	fn execute(&mut self, instruction: &Instruction) {
		match instruction {
			Instruction::Text(param) => {
				self.new_group(GroupType::Text);
				self.current_node.borrow_mut().add_text(&param);
				self.end_group(GroupType::Text);
			}
			Instruction::Tag(param, arg) => {self.parse_tag(&param, arg);},
			Instruction::Parabreak => {
				self.end_group(GroupType::Paragraph);
				self.new_group(GroupType::Paragraph);
			}
			Instruction::Linebreak => {
				self.new_group(GroupType::Br);
				self.current_node.borrow_mut().set_void(true);
				self.end_group(GroupType::Br);
			}
			Instruction::Scenebreak => {
				self.new_group(GroupType::Scenebreak);
				self.current_node.borrow_mut().set_void(true);
				self.end_group(GroupType::Scenebreak);
			}
			_ => {}
		}
	}
	/// Creates a new ASTElement.
	fn new_group(&mut self, ele_type: GroupType) {
		self.current_node.append(Node::new(ASTElement::new(ele_type)));
		self.current_node = self.current_node.last_child().unwrap();
	}
	/// Moves current working node up to the current node's parent.
	fn end_group(&mut self, ele_type: GroupType) {
		if self.current_node.borrow_mut().ele_type() == &ele_type {
			match self.current_node.parent() {
				None => {},
				Some(parent) => {
					if !self.current_node.has_children() 
					&& !self.current_node.borrow().has_text() 
					&& !self.current_node.borrow().is_void() {
						self.current_node.detach();
					}
					self.current_node = parent;
				}
			};
		} else {
			let mut group_stack = Vec::new();
			let mut go = true;
			while go {
				let my_type = self.current_node.borrow_mut().ele_type().clone();
				match my_type {
					GroupType::Paragraph if ele_type != GroupType::Paragraph => {
						go = false;
					},
					GroupType::Document if ele_type != GroupType::Document => {
						go = false;
					},
					_ => {
						if my_type == ele_type {
							go = false;
						} else {
							group_stack.push(my_type);
						}
						match self.current_node.parent() {
							None => {
								go = false;
							},
							Some(parent) => {
								if !self.current_node.has_children() 
								&& !self.current_node.borrow().has_text() 
								&& !self.current_node.borrow().is_void() {
									self.current_node.detach();
								}
								self.current_node = parent;
							}
						};
					}
				}
			}
			while group_stack.len() > 0 {
				self.new_group(group_stack.pop().unwrap().clone());
			}
		}	
	}
	/// Parses tag Instructions.
	fn parse_tag(&mut self, tag: &str, args: &Option<String>) {
		match args {
			Some(primary_arg) => {
				match ONE_ARG_CMD.get(tag) {
					Some(cmd) => cmd(self, primary_arg),
					None => self.execute(&Instruction::Text(format!("[{}={}]", tag, primary_arg)))
				}
			},
			None => {
				match NO_ARG_CMD.get(tag) {
					Some(cmd) => cmd(self),
					None => self.execute(&Instruction::Text(format!("[{}]", tag)))
				}
			}
		}
	}

	/*-- COMMANDS --*/
	fn cmd_bold_open(&mut self) {
		self.new_group(GroupType::Bold);
	}
	fn cmd_bold_close(&mut self) {
		self.end_group(GroupType::Bold);
	}

	fn cmd_italic_open(&mut self) {
		self.new_group(GroupType::Italic);
	}
	fn cmd_italic_close(&mut self) {
		self.end_group(GroupType::Italic);
	}

	fn cmd_strong_open(&mut self) {
		self.new_group(GroupType::Strong);
	}
	fn cmd_strong_close(&mut self) {
		self.end_group(GroupType::Strong);
	}

	fn cmd_emphasis_open(&mut self) {
		self.new_group(GroupType::Emphasis);
	}
	fn cmd_emphasis_close(&mut self) {
		self.end_group(GroupType::Emphasis);
	}

	fn cmd_underline_open(&mut self) {
		self.new_group(GroupType::Underline);
	}
	fn cmd_underline_close(&mut self) {
		self.end_group(GroupType::Underline);
	}

	fn cmd_smallcaps_open(&mut self) {
		self.new_group(GroupType::Smallcaps);
	}
	fn cmd_smallcaps_close(&mut self) {
		self.end_group(GroupType::Smallcaps);
	}

	fn cmd_strikethrough_open(&mut self) {
		self.new_group(GroupType::Strikethrough);
	}
	fn cmd_strikethrough_close(&mut self) {
		self.end_group(GroupType::Strikethrough);
	}

	fn cmd_monospace_open(&mut self) {
		self.new_group(GroupType::Monospace);
	}
	fn cmd_monospace_close(&mut self) {
		self.end_group(GroupType::Monospace);
	}

	fn cmd_subscript_open(&mut self) {
		self.new_group(GroupType::Subscript);
	}
	fn cmd_subscript_close(&mut self) {
		self.end_group(GroupType::Subscript);
	}

	fn cmd_superscript_open(&mut self) {
		self.new_group(GroupType::Superscript);
	}
	fn cmd_superscript_close(&mut self) {
		self.end_group(GroupType::Superscript);
	}

	fn cmd_spoiler_open(&mut self) {
		self.new_group(GroupType::Spoiler);
	}
	fn cmd_spoiler_close(&mut self) {
		self.end_group(GroupType::Spoiler);
	}

	fn cmd_colour_open(&mut self, arg: &String) {
		if arg.starts_with("#") && arg.len() == 7 || arg.len() == 4 
		&& arg.trim_start_matches('#').chars().all(|c| c.is_ascii_hexdigit()) {
			self.new_group(GroupType::Colour);
			self.current_node.borrow_mut().set_arg(arg);
		} else if WEB_COLOURS.contains(arg.as_str()) {
			self.new_group(GroupType::Colour);
			self.current_node.borrow_mut().set_arg(arg);
		} else {
			self.new_group(GroupType::Broken);
			self.current_node.borrow_mut().set_arg(&format!("color={}", arg));
		}
	}
	fn cmd_colour_close(&mut self) {
		self.end_group(GroupType::Colour);
	}

	fn cmd_url_open(&mut self, arg: &String) {
		if arg.starts_with("https://") || arg.starts_with("http://") {
			self.new_group(GroupType::Url);
			self.current_node.borrow_mut().set_arg(arg);
		} else if arg.starts_with("www.") {
			self.new_group(GroupType::Url);
			self.current_node.borrow_mut().set_arg(&format!("http://{}", arg));
		} else {
			self.new_group(GroupType::Broken);
			self.current_node.borrow_mut().set_arg(&format!("url={}", arg));
		}
	}
	fn cmd_url_close(&mut self) {
		self.end_group(GroupType::Url);
	}

	fn cmd_img(&mut self, arg: &String) {
		if arg.starts_with("https://") || arg.starts_with("http://") {
			if let Some(index) = arg.rfind(".") {
				if let Some(suffix) = arg.get(index..) {
					if ACCEPTED_IMAGE_TYPES.contains(suffix) {
						self.new_group(GroupType::Image);
						self.current_node.borrow_mut().set_void(true);
						self.current_node.borrow_mut().set_arg(arg);
						self.end_group(GroupType::Image);
					} else {
						self.new_group(GroupType::Broken);
						self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
						self.end_group(GroupType::Broken);
					}
				} else {
					self.new_group(GroupType::Broken);
					self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
					self.end_group(GroupType::Broken);
				}
			} else {
				self.new_group(GroupType::Broken);
				self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
				self.end_group(GroupType::Broken);
			}

		} else if arg.starts_with("www.") {
			if let Some(index) = arg.rfind(".") {
				if let Some(suffix) = arg.get(index..) {
					if ACCEPTED_IMAGE_TYPES.contains(suffix) {
						self.new_group(GroupType::Image);
						self.current_node.borrow_mut().set_void(true);
						self.current_node.borrow_mut().set_arg(&format!("http://{}", arg));
						self.end_group(GroupType::Image);
					} else {
						self.new_group(GroupType::Broken);
						self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
						self.end_group(GroupType::Broken);
					}
				} else {
					self.new_group(GroupType::Broken);
					self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
					self.end_group(GroupType::Broken);
				}
			} else {
				self.new_group(GroupType::Broken);
				self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
				self.end_group(GroupType::Broken);
			}
		} else {
			self.new_group(GroupType::Broken);
			self.current_node.borrow_mut().set_arg(&format!("img={}", arg));
			self.end_group(GroupType::Broken);
		}
	}

	fn cmd_opacity_open(&mut self, arg: &String) {
		let mut divisor = 1.0;
		let arg_string;
		if arg.ends_with("%") {
			arg_string = arg.trim_end_matches('%');
			divisor = 100.0;
		} else {
			arg_string = arg;
		}
		match arg_string.parse::<f32>() {
			Ok(mut val) => {
				val = val/divisor;
				if val < 0.0 {
					val = 0.0;
				} else if val > 1.0 {
					val = 1.0;
				}
				self.new_group(GroupType::Opacity);
				self.current_node.borrow_mut().set_arg(&val.to_string());
			}
			Err(_) => {
				self.new_group(GroupType::Broken);
				self.current_node.borrow_mut().set_arg(&format!("opacity={}", arg));
			}
		}
	}
	fn cmd_opacity_close(&mut self) {
		self.end_group(GroupType::Opacity);
	}

	fn cmd_size_open(&mut self, arg: &String) {
		let mut divisor = 1.0;
		let arg_string;
		if arg.ends_with("em") {
			arg_string = arg.trim_end_matches("em");
		} else {
			arg_string = arg;
			divisor = 16.0;
		}
		match arg_string.parse::<f32>() {
			Ok(mut val) => {
				val = val/divisor;
				if val < 0.5 {
					val = 0.5;
				} else if val > 2.0 {
					val = 2.0;
				}
				self.new_group(GroupType::Size);
				self.current_node.borrow_mut().set_arg(&val.to_string());
			}
			Err(_) => {
				self.new_group(GroupType::Broken);
				self.current_node.borrow_mut().set_arg(&format!("size={}", arg));
			}
		}
	}
	fn cmd_size_close(&mut self) {
		self.end_group(GroupType::Size);
	}

	fn cmd_quote_open(&mut self) {
		self.new_group(GroupType::Quote);
	}
	fn cmd_quote_arg_open(&mut self, arg: &String) {
		self.new_group(GroupType::Quote);
		self.current_node.borrow_mut().set_arg(arg);
	}
	fn cmd_quote_close(&mut self) {
		self.end_group(GroupType::Quote);
	}

	fn cmd_hr(&mut self) {
		self.new_group(GroupType::Hr);
		self.current_node.borrow_mut().set_void(true);
		self.end_group(GroupType::Hr);
	}

	fn cmd_center_open(&mut self) {
		self.end_group(GroupType::Paragraph);
		self.new_group(GroupType::Center);
		self.new_group(GroupType::Paragraph);
	}
	fn cmd_center_close(&mut self) {
		self.end_group(GroupType::Paragraph);
		self.end_group(GroupType::Center);
		self.new_group(GroupType::Paragraph);
	}

	fn cmd_right_open(&mut self) {
		self.end_group(GroupType::Paragraph);
		self.new_group(GroupType::Right);
		self.new_group(GroupType::Paragraph);
	}
	fn cmd_right_close(&mut self) {
		self.end_group(GroupType::Paragraph);
		self.end_group(GroupType::Right);
		self.new_group(GroupType::Paragraph);
	}
}
/// Static compile-time map of tags without arguments to lexer commands.
static NO_ARG_CMD: phf::Map<&'static str, fn(&mut BBCodeLexer)> = phf_map! {
    "b" => BBCodeLexer::cmd_bold_open,
    "/b" => BBCodeLexer::cmd_bold_close,
    "i" => BBCodeLexer::cmd_italic_open,
	"/i" => BBCodeLexer::cmd_italic_close,
	"s" => BBCodeLexer::cmd_strikethrough_open,
	"/s" => BBCodeLexer::cmd_strikethrough_close,
	"strong" => BBCodeLexer::cmd_strong_open,
	"/strong" => BBCodeLexer::cmd_strong_close,
	"em" => BBCodeLexer::cmd_emphasis_open,
	"/em" => BBCodeLexer::cmd_emphasis_close,
	"u" => BBCodeLexer::cmd_underline_open,
	"/u" => BBCodeLexer::cmd_underline_close,
	"smcaps" => BBCodeLexer::cmd_smallcaps_open,
	"/smcaps" => BBCodeLexer::cmd_smallcaps_close,
	"mono" => BBCodeLexer::cmd_monospace_open,
	"/mono" => BBCodeLexer::cmd_monospace_close,
	"sub" => BBCodeLexer::cmd_subscript_open,
	"/sub" => BBCodeLexer::cmd_subscript_close,
	"sup" => BBCodeLexer::cmd_superscript_open,
	"/sup" => BBCodeLexer::cmd_superscript_close,
	"spoiler" => BBCodeLexer::cmd_spoiler_open,
	"/spoiler" => BBCodeLexer::cmd_spoiler_close,
	"hr" => BBCodeLexer::cmd_hr,
	"center" => BBCodeLexer::cmd_center_open,
	"/center" => BBCodeLexer::cmd_center_close,
	"right" => BBCodeLexer::cmd_right_open,
	"/right" => BBCodeLexer::cmd_right_close,
	"/color" => BBCodeLexer::cmd_colour_close,
	"/colour" => BBCodeLexer::cmd_colour_close,
	"/opacity" => BBCodeLexer::cmd_opacity_close,
	"/size" => BBCodeLexer::cmd_size_close,
	"/url" => BBCodeLexer::cmd_url_close,
	"quote" => BBCodeLexer::cmd_quote_open,
	"/quote" => BBCodeLexer::cmd_quote_close,
};
/// Static compile-time map of tags with single arguments to lexer commands.
static ONE_ARG_CMD: phf::Map<&'static str, fn(&mut BBCodeLexer, &String)> = phf_map! {
    "color" => BBCodeLexer::cmd_colour_open,
	"colour" => BBCodeLexer::cmd_colour_open,
	"url" => BBCodeLexer::cmd_url_open,
	"img" => BBCodeLexer::cmd_img,
	"opacity" => BBCodeLexer::cmd_opacity_open,
	"size" => BBCodeLexer::cmd_size_open,
	"quote" => BBCodeLexer::cmd_quote_arg_open,
};
/// Static compile-time set of valid HTML web colours.
static WEB_COLOURS: phf::Set<&'static str> = phf_set! {
	"aliceblue",
    "antiquewhite",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgrey",
    "darkgreen",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "grey",
    "green",
    "greenyellow",
    "honeydew",
    "hotpink",
    "indianred ",
    "indigo ",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgrey",
    "lightgreen",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "transparant",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
    "Aliceblue",
    "Antiquewhite",
    "Aqua",
    "Aquamarine",
    "Azure",
    "Beige",
    "Bisque",
    "Black",
    "Blanchedalmond",
    "Blue",
    "Blueviolet",
    "Brown",
    "Burlywood",
    "Cadetblue",
    "Chartreuse",
    "Chocolate",
    "Coral",
    "Cornflowerblue",
    "Cornsilk",
    "Crimson",
    "Cyan",
    "Darkblue",
    "Darkcyan",
    "Darkgoldenrod",
    "Darkgray",
    "Darkgrey",
    "Darkgreen",
    "Darkkhaki",
    "Darkmagenta",
    "Darkolivegreen",
    "Darkorange",
    "Darkorchid",
    "Darkred",
    "Darksalmon",
    "Darkseagreen",
    "Darkslateblue",
    "Darkslategray",
    "Darkslategrey",
    "Darkturquoise",
    "Darkviolet",
    "Deeppink",
    "Deepskyblue",
    "Dimgray",
    "Dimgrey",
    "Dodgerblue",
    "Firebrick",
    "Floralwhite",
    "Forestgreen",
    "Fuchsia",
    "Gainsboro",
    "Ghostwhite",
    "Gold",
    "Goldenrod",
    "Gray",
    "Grey",
    "Green",
    "Greenyellow",
    "Honeydew",
    "Hotpink",
    "Indianred ",
    "Indigo ",
    "Ivory",
    "Khaki",
    "Lavender",
    "Lavenderblush",
    "Lawngreen",
    "Lemonchiffon",
    "Lightblue",
    "Lightcoral",
    "Lightcyan",
    "Lightgoldenrodyellow",
    "Lightgray",
    "Lightgrey",
    "Lightgreen",
    "Lightpink",
    "Lightsalmon",
    "Lightseagreen",
    "Lightskyblue",
    "Lightslategray",
    "Lightslategrey",
    "Lightsteelblue",
    "Lightyellow",
    "Lime",
    "Limegreen",
    "Linen",
    "Magenta",
    "Maroon",
    "Mediumaquamarine",
    "Mediumblue",
    "Mediumorchid",
    "Mediumpurple",
    "Mediumseagreen",
    "Mediumslateblue",
    "Mediumspringgreen",
    "Mediumturquoise",
    "Mediumvioletred",
    "Midnightblue",
    "Mintcream",
    "Mistyrose",
    "Moccasin",
    "Navajowhite",
    "Navy",
    "Oldlace",
    "Olive",
    "Olivedrab",
    "Orange",
    "Orangered",
    "Orchid",
    "Palegoldenrod",
    "Palegreen",
    "Paleturquoise",
    "Palevioletred",
    "Papayawhip",
    "Peachpuff",
    "Peru",
    "Pink",
    "Plum",
    "Powderblue",
    "Purple",
    "Rebeccapurple",
    "Red",
    "Rosybrown",
    "Royalblue",
    "Saddlebrown",
    "Salmon",
    "Sandybrown",
    "Seagreen",
    "Seashell",
    "Sienna",
    "Silver",
    "Skyblue",
    "Slateblue",
    "Slategray",
    "Slategrey",
    "Snow",
    "Springgreen",
    "Steelblue",
    "Tan",
    "Teal",
    "Thistle",
    "Tomato",
    "Turquoise",
    "Transparant",
    "Violet",
    "Wheat",
    "White",
    "Whitesmoke",
    "Yellow",
    "Yellowgreen"
};

/// Static compile-time set of accepted image types.
static ACCEPTED_IMAGE_TYPES: phf::Set<&'static str> = phf_set! {
	".jpg",
	".jpeg",
	".pjpeg",
	".pjp",
	".jfif",
	".png",
	".apng",
	".gif",
	".bmp",
	//".svg", Dangerous!
	".webp",
	".ico",
	".cur"
};