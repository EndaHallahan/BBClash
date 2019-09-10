use rctree::{Node, NodeEdge};
use std::cell::Ref;
use super::GroupType;
use super::ASTElement; 

/// Struct for generation of HTML strings.
pub struct HTMLConstructor {
	output_string: String,
}
impl HTMLConstructor {
	/// Creates a new HTMLConstructor.
	pub fn new (out_len: usize) -> HTMLConstructor {
		let output_string = String::with_capacity(out_len + out_len/2);
		HTMLConstructor{output_string}
	}

	/// Generates an HTML string from an ASTElement
	pub fn construct(&mut self, ast: Node<ASTElement>) -> String {
		for node_edge in ast.traverse() {
			match node_edge {
				NodeEdge::Start(node) => {self.start_element(node.borrow())},
				NodeEdge::End(node) => {self.end_element(node.borrow())}
			}
		}	
		self.output_string.clone()
	}

	/// Opens an HTML tag.
	fn start_element(&mut self, element: Ref<ASTElement>) {
		match element.ele_type() {
			GroupType::Text => {
				if let Some(text) = element.text_contents() {
					self.output_string.push_str(text)
				}	
			},
			GroupType::Paragraph => {self.output_string.push_str("<p>")},
			GroupType::Bold => {self.output_string.push_str("<b>")},
			GroupType::Strong => {self.output_string.push_str("<strong>")},
			GroupType::Italic => {self.output_string.push_str("<i>")},
			GroupType::Emphasis => {self.output_string.push_str("<em>")},
			GroupType::Underline => {self.output_string.push_str("<span class=\"underline\">")},
			GroupType::Strikethrough => {self.output_string.push_str("<s>")},
			GroupType::Smallcaps => {self.output_string.push_str("<span class=\"smallcaps\">")},
			GroupType::Monospace => {self.output_string.push_str("<span class=\"monospace\">")},
			GroupType::Subscript => {self.output_string.push_str("<sub>")},
			GroupType::Superscript => {self.output_string.push_str("<sup>")},
			GroupType::Spoiler => {self.output_string.push_str("<span class=\"spoiler\">")},
			GroupType::Hr => {self.output_string.push_str("<hr>")},
			GroupType::Br => {self.output_string.push_str("<br>")},
			GroupType::Scenebreak => {self.output_string.push_str("<br><br><br>")},
			GroupType::Center => {self.output_string.push_str("<div class=\"center\">")},
			GroupType::Right => {self.output_string.push_str("<div class=\"right\">")},
			GroupType::Colour => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<span style=\"color:{};\">", arg));
				}	
			},
			GroupType::Url => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<a href=\"{}\" rel=\"nofollow\">", arg));
				}	
			},
			GroupType::Opacity => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<span style=\"opacity:{};\">", arg));
				}
			},
			GroupType::Size => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<span style=\"font-size:{}rem;\">", arg));
				}
			},
			GroupType::Image => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<img src=\"{}\">", arg));
				}
			},
			GroupType::Quote => {
				if let Some(arg) = element.argument() {
					self.output_string.push_str(&format!("<blockquote data-author=\"{}\">", arg));
				} else {
					self.output_string.push_str(&format!("<blockquote>"));
				}
			},
			_ => return
		};
	}

	/// Closes an HTML tag.
	fn end_element(&mut self, element: Ref<ASTElement>) {
		match element.ele_type() {
			GroupType::Paragraph => {self.output_string.push_str("</p>")},
			GroupType::Bold => {self.output_string.push_str("</b>")},
			GroupType::Strong => {self.output_string.push_str("</strong>")},
			GroupType::Italic => {self.output_string.push_str("</i>")},
			GroupType::Emphasis => {self.output_string.push_str("</em>")},
			GroupType::Subscript => {self.output_string.push_str("</sub>")},
			GroupType::Superscript => {self.output_string.push_str("</sup>")},
			GroupType::Strikethrough => {self.output_string.push_str("</s>")},
			GroupType::Url => {self.output_string.push_str("</a>")},
			GroupType::Quote => {self.output_string.push_str("</blockquote>")},
			GroupType::Underline |
			GroupType::Smallcaps |
			GroupType::Monospace |
			GroupType::Spoiler |
			GroupType::Colour |
			GroupType::Opacity |
			GroupType::Size
				=> {self.output_string.push_str("</span>")},
			GroupType::Center |
			GroupType::Right
				=> {self.output_string.push_str("</div>")}
			_ => return
		};
	}
}