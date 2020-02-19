use std::{marker::PhantomData,};
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
        let context = Exec::shell("kubectl config current-context")
            .stdout(Redirection::Pipe)
            .capture()?.stdout_str();
        let namespace = Exec::shell(format!("kubectl config view -o=jsonpath='{{.contexts[?(@.name==\"{}\")].context.namespace}}'",context.trim_right()))
            .stdout(Redirection::Pipe)
            .capture()?.stdout_str();
		segments.push(Segment::simple(format!(" {}/{} ", context.trim_end(), namespace.trim_end()), S::K8S_FG, S::K8S_BG));
		Ok(())
	}
}
