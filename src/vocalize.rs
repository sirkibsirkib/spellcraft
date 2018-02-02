use std::fmt

pub trait Vocalize {
	fn vocalize()
}


pub struct Phoneme(String);
pub struct Word(Vec<Phoneme>);
pub struct Sentence(Vec<Word>);
pub struct Paragraph(Vec<Sentence>);


impl Paragraph {
	pub fn serialize(&self) -> String {
		let mut s = String::new();
		for sentence in self.0.iter() {
			for word in sentence.0.iter() {
				s.push(' ');
				for phoneme in word.0.iter() {
					s.push_str(&phoneme.0);
				}
			}
			s.push('.');
		}
		if s.len() > 0 {
			s.remove(0); //remove leading space
		}
		s
	}
}


pub struct Vocalizer {

}

impl Vocalizer {
	pub fn utter(&mut self, Phoneme) {

	}
	pub fn 
}