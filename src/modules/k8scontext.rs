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
        let  k8s_csv= Exec::shell("yq eval '.current-context as $current| .contexts[]|select(.name==$current)|[.context.cluster,.context.namespace]|@csv' ~/.kube/config")
            .stdout(Redirection::Pipe)
            .capture()?.stdout_str();
        let (context,namespace) = k8s_csv.split_once(",").unwrap();


/*        let context = Exec::shell("kubectl config current-context")
            .stdout(Redirection::Pipe)
            .capture()?.stdout_str();
        let namespace = Exec::shell(format!("kubectl config view -o=jsonpath='{{.contexts[?(@.name==\"{}\")].context.namespace}}'",context.trim_end()))
            .stdout(Redirection::Pipe)
            .capture()?.stdout_str();
*/
		segments.push(Segment::simple(format!("\u{2388} {}/{} ", context.trim_end(), namespace.trim_end()), S::K8S_FG, S::K8S_BG));
		Ok(())
	}
}
