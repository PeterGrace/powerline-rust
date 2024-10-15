use std::{marker::PhantomData,};
use std::fs::File;
use serde_yaml::Value;
use subprocess::{Exec, Redirection};
use super::Module;
use crate::{terminal::Color, Segment, R};
pub struct K8sContext<S: K8sContextScheme> {
	scheme: PhantomData<S>,
}

pub trait K8sContextScheme {
	const K8S_BG: Color;
	const K8S_FG: Color;
}

impl<S: K8sContextScheme> K8sContext<S> {
	pub fn new() -> K8sContext<S> {
		K8sContext {
			scheme: PhantomData,
		}
	}

}

impl<S: K8sContextScheme> Module for K8sContext<S> {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {
		let mut context: String = String::default();
		let mut namespace: String = String::default();
		if let Ok(fd) = File::open("/home/pgrace/.kube/config") {
			if let Ok(v) = serde_yaml::from_reader::<File, Value>(fd) {
				context = v["current-context"].as_str().unwrap_or("").to_string();
				if let Some(contexts) = v.get("contexts") {
					if let Some(array) = contexts.as_sequence() {
						let cur_context = array.iter().find(|x| x.get("name").unwrap().as_str() == Some(&context)).unwrap();
						namespace = cur_context["context"]["namespace"].as_str().unwrap_or("").to_string();
					}
				}
			}
		}
		segments.push(Segment::simple(format!("\u{2388} {}/{} ", context.trim_end(), namespace.trim_end()), S::K8S_FG, S::K8S_BG));
		Ok(())
	}
}
