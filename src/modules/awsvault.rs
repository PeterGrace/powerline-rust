use super::Module;
use crate::{terminal::Color, Segment, R};
use std::{env, marker::PhantomData};

pub struct AWSVault<S: AWSVaultScheme> {
	scheme: PhantomData<S>,
}

pub trait AWSVaultScheme {
	const AWS_BG: Color;
	const AWS_FG: Color;
}

impl<S: AWSVaultScheme> AWSVault<S> {
	pub fn new() -> AWSVault<S> {
		AWSVault { scheme: PhantomData }
	}
}

impl<S: AWSVaultScheme> Module for AWSVault<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		match env::var("AWS_VAULT") {
			Ok(val) => {
				segments.push(Segment::simple(format!(" (aws:{}) ", val), S::AWS_FG, S::AWS_BG));
				Ok(())
			}
			Err(_) => Ok(()),
		}
	}
}
