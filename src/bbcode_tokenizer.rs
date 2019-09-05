use super::Instruction;

/// Tokenizer modes.
#[derive(Debug, PartialEq)]
enum ReadMode {
	ParseText,
	ParseEscape,
	ParseTag,
	ParseTagPrimaryArg,
	ParseParabreak,
	ParseScenebreak,
}

/// Struct for BBCode tokenization.
pub struct BBCodeTokenizer {
	mode: ReadMode,
	current_instruction: Instruction,
	instructions: Vec<Instruction>
}
impl BBCodeTokenizer {
	/// Creates a new BBCodeTokenizer
	pub fn new() -> BBCodeTokenizer {
		let mode = ReadMode::ParseText;
		let current_instruction = Instruction::Null;
		let instructions = Vec::new();
		BBCodeTokenizer{mode, current_instruction, instructions}
	}
	/// Reads and tokenizes BBCode into individual Instructions.
	pub fn tokenize(&mut self, bbcode: &str) -> &Vec<Instruction> {
		let bbcode_chars = bbcode.chars();
		for character in bbcode_chars {
			match &self.mode {
				ReadMode::ParseText => {self.parse_text(character);},
				ReadMode::ParseEscape => {self.parse_escape(character);},
				ReadMode::ParseTag => {self.parse_tag(character);},
				ReadMode::ParseTagPrimaryArg => {self.parse_tag_primary_arg(character);},
				ReadMode::ParseParabreak => {self.parse_parabreak(character);},
				ReadMode::ParseScenebreak => {self.parse_scenebreak(character);},
			}
		}
		self.set_cur_instruction();
		&self.instructions
	}
	/// Parses characters.
	fn parse_text(&mut self, character: char) {
		match character {
			'\\' => {
				self.mode = ReadMode::ParseEscape
			},
			'[' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseTag;
			},
			'\n' | '\r' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseParabreak;
			},
			'>' | '<' | '&' | '"' | '\'' => {
				let san_char = self.sanitize(character);
				match self.current_instruction {
					Instruction::Text(ref mut contents) => {
						contents.push_str(&san_char);
					},
					_ => {
						self.current_instruction = Instruction::Text(san_char);
					}
				}
			},
			_ => {
				match self.current_instruction {
					Instruction::Text(ref mut contents) => {
						contents.push(character);
					},
					_ => {
						self.current_instruction = Instruction::Text(character.to_string());
					}
				}
			}
		}
	}
	/// Parses paragraph breaks.
	fn parse_parabreak(&mut self, character: char) {
		match character {
			'\t' => {
				self.set_new_instruction(Instruction::Parabreak);
				self.mode = ReadMode::ParseText;
			},
			'\n' | '\r' => {
				self.mode = ReadMode::ParseScenebreak;
			},
			' ' => {},
			_ => {
				self.set_new_instruction(Instruction::Linebreak);
				self.mode = ReadMode::ParseText;
				self.parse_text(character);
			}
		}
		
	}
	/// Parses scen breaks (three newlines).
	fn parse_scenebreak(&mut self, character: char) {
		match character {
			'\n' | '\r' => {
				self.set_new_instruction(Instruction::Scenebreak);
				self.mode = ReadMode::ParseText;
			},
			' ' => {},
			_ => {
				self.set_new_instruction(Instruction::Parabreak);
				self.mode = ReadMode::ParseText;
				self.parse_text(character);
			}
		}
		
	}
	/// Parses escaped charcters.
	fn parse_escape(&mut self, character: char) {
		self.mode = ReadMode::ParseText;
		match character {
			'>' | '<' | '&' | '"' | '\'' | '\\' => {
				let san_char = self.sanitize(character);
				match self.current_instruction {
					Instruction::Tag(ref mut contents, _) => {
						contents.push_str(&san_char);
					},
					_ => {
						self.current_instruction = Instruction::Text(san_char);
					}
				}
			},
			_ => {
				match self.current_instruction {
					Instruction::Text(ref mut contents) => {
						contents.push(character);
					},
					_ => {
						self.current_instruction = Instruction::Text(character.to_string());
					}
				}
			}
		}	
	}
	/// Parses BBCode tags.
	fn parse_tag(&mut self, character: char) {
		match character {
			']' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseText;
			},
			'=' => {
				self.mode = ReadMode::ParseTagPrimaryArg;
			},
			'>' | '<' | '&' | '"' | '\'' | '\\' => {
				let san_char = self.sanitize(character);
				match self.current_instruction {
					Instruction::Tag(ref mut contents, _) => {
						contents.push_str(&san_char);
					},
					_ => {
						self.current_instruction = Instruction::Tag(san_char, None);
					}
				}
			},
			_ => {
				match self.current_instruction {
					Instruction::Tag(ref mut contents, _) => {
						contents.push(character);
					},
					_ => {
						self.current_instruction = Instruction::Tag(character.to_string(), None);
					}
				}
			}
		}	
	}
	/// Parses BBCode tag arguments.
	fn parse_tag_primary_arg(&mut self, character: char) {
		match character {
			']' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseText;
			},
			'>' | '<' | '&' | '"' | '\'' | '\\' => {
				let san_char = self.sanitize(character);
				match self.current_instruction {
					Instruction::Tag(ref mut contents, ref mut args) => {
						match args {
							Some(ref mut primarg) => {
								primarg.push_str(&san_char);
							},
							None => {
								self.current_instruction = Instruction::Tag(contents.to_string(), Some(san_char.to_string()));
							}
						}
					},
					_ => {
						unreachable!();
					}
				}
			},
			_ => {
				match self.current_instruction {
					Instruction::Tag(ref mut contents, ref mut args) => {
						match args {
							Some(ref mut primarg) => {
								primarg.push(character);
							},
							None => {
								self.current_instruction = Instruction::Tag(contents.to_string(), Some(character.to_string()));
							}
						}
					},
					_ => {
						unreachable!();
					}
				}
			}
		}
	}
	/// Adds current instruction to instruction vector and restes current instruction.
	fn set_cur_instruction(&mut self) {
		if self.current_instruction != Instruction::Null {
			self.instructions.push(self.current_instruction.clone());
			self.current_instruction = Instruction::Null;
		}
	}
	/// Adds a given instruction to instruction vector and resets current instruction.
	fn set_new_instruction(&mut self, instruction: Instruction) {
		self.instructions.push(instruction.clone());
		self.current_instruction = Instruction::Null;
	}
	/// Sanitizes characters for HTML.
	fn sanitize(&mut self, character: char) -> String {
		return match character {
			'<' => "&lt",
			'>' => "&gt",
			'&' => "&amp",
			'"' => "&quot",
			'\'' => "&#x27",
			'\\' => "&#x2F",
			_ => unreachable!()
		}.to_string();
	}
}