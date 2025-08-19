use crate::{error::Error, powerline::Segment};

mod cmd;
mod cwd;
mod git;
mod host;
mod readonly;
mod user;
mod newline;
mod k8scontext;
mod awsvault;
mod distrobox;

pub use cmd::{Cmd, CmdScheme};
pub use cwd::{Cwd, CwdScheme};
pub use git::{Git, GitScheme};
pub use host::{Host, HostScheme};
pub use readonly::{ReadOnly, ReadOnlyScheme};
pub use user::{User, UserScheme};
pub use newline::{NewLine, NewLineScheme};
pub use k8scontext::{K8sContext, K8sContextScheme};
pub use awsvault::{AWSVault, AWSVaultScheme};
pub use distrobox::{DistroBox, DistroBoxScheme};

pub trait Module: Sized {
	fn append_segments(&mut self, segments: &mut Vec<Segment>) -> Result<(), Error>;
	#[inline]
	fn into_segments(mut self) -> Result<Vec<Segment>, Error> {
		self.get_segments()
	}

	#[inline]
	fn get_segments(&mut self) -> Result<Vec<Segment>, Error> {
		let mut vec = Vec::new();
		self.append_segments(&mut vec).map(|_| vec)
	}
}
