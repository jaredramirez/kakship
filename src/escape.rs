#[derive(PartialEq, Debug)]
pub enum Token<'a> {
	Percent,
	Str(&'a str),
	Block(&'a str),
}

pub struct EscapeIterator<'a> {
	remainder: &'a str,
}

impl<'a> EscapeIterator<'a> {
	pub fn new(string: &'a str) -> Self {
		Self { remainder: string }
	}

	// (TODO: handle block escape)
	pub fn yield_block(&mut self, header_size: usize) -> Option<Token<'a>> {
		let mut count = 1;
		let mut end = header_size;
		for c in self.remainder[header_size..].chars() {
			if count == 0 {
				break;
			}
			if c == '{' {
				count += 1;
			} else if c == '}' {
				count -= 1;
			}
			end += 1;
		}
		if end < self.remainder.len() {
			let chunk = &self.remainder[..end];
			self.remainder = &self.remainder[end..];
			return Some(Token::Block(chunk));
		} else {
			let chunk = self.remainder;
			self.remainder = "";
			return Some(Token::Block(chunk));
		}
	}

	pub fn yield_percent(&mut self) -> Option<Token<'a>> {
		self.remainder = &self.remainder[1..];
		Some(Token::Percent)
	}

	pub fn yield_remainder(&mut self) -> Option<Token<'a>> {
		let chunk = self.remainder;
		self.remainder = "";
		Some(Token::Str(chunk))
	}

	pub fn yield_chunk(&mut self, end: usize) -> Option<Token<'a>> {
		let chunk = &self.remainder[..end];
		self.remainder = &self.remainder[end..];
		Some(Token::Str(chunk))
	}
}

/// Iterator that either yield
/// - a percent (to be escaped)
/// - a string that doesn't have any %
/// - a block with matching braces (%opt %var, %sh, %)
impl<'a> Iterator for EscapeIterator<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		return if self.remainder.is_empty() {
			None
		} else if self.remainder.starts_with("%") {
			if self.remainder[1..].starts_with("{") {
				self.yield_block(2)
			} else if self.remainder[1..].starts_with("sh{") {
				self.yield_block(4)
			} else if self.remainder[1..].starts_with("opt{") || self.remainder[1..].starts_with("val{") {
				self.yield_block(5)
			} else {
				self.yield_percent()
			}
		} else {
			match self.remainder.find("%") {
				None => {
					self.yield_remainder()
				}
				Some(end) => {
					self.yield_chunk(end)
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty() {
		let tokens: Vec<_> = EscapeIterator::new("").collect();
		assert_eq!(tokens, &[]);
	}

	#[test]
	fn percent() {
		let tokens: Vec<_> = EscapeIterator::new("%").collect();
		assert_eq!(tokens, vec![Token::Percent]);
	}

	#[test]
	fn string() {
		let tokens: Vec<_> = EscapeIterator::new("hello world !").collect();
		assert_eq!(tokens, vec![Token::Str("hello world !")]);
	}

	#[test]
	fn opt() {
		let tokens: Vec<_> = EscapeIterator::new("%opt{filetype}").collect();
		assert_eq!(tokens, vec![Token::Block("%opt{filetype}")]);
	}

	#[test]
	fn val() {
		let tokens: Vec<_> = EscapeIterator::new("%val{session}").collect();
		assert_eq!(tokens, vec![Token::Block("%val{session}")]);
	}

	#[test]
	fn sh() {
		let tokens: Vec<_> = EscapeIterator::new("%sh{basename \"$kak_file\"}").collect();
		assert_eq!(tokens, vec![Token::Block("%sh{basename \"$kak_file\"}")]);
	}

	#[test]
	fn expansion() {
		let tokens: Vec<_> = EscapeIterator::new("%{echo %opt{filetype}}").collect();
		assert_eq!(
			tokens,
			vec![
				Token::Block("%{echo %opt{filetype}}")
			]
		);
	}

	#[test]
	fn mixed() {
		let tokens: Vec<_> = EscapeIterator::new("%opt{filetype} 98% %val{session}").collect();
		assert_eq!(
			tokens,
			vec![
				Token::Block("%opt{filetype}"),
				Token::Str(" 98"),
				Token::Percent,
				Token::Str(" "),
				Token::Block("%val{session}")
			]
		);
	}

	#[test]
	fn noescape_inside_block() {
		let tokens: Vec<_> = EscapeIterator::new("%{date +%T}").collect();
		assert_eq!(tokens, vec![Token::Block("%{date +%T}")]);
	}
}
