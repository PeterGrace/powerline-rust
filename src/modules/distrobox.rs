use std::marker::PhantomData;
use crate::modules::{Module};
use crate::{Segment, R};
use crate::terminal::Color;
use subprocess::{Exec};

pub struct DistroBox<S>(PhantomData<S>);

pub trait DistroBoxScheme {
    const DISTROBOX_FG: Color;
    const DISTROBOX_BG: Color;
    const DISTROBOX_SYMBOL: &'static str = "🧰";
}

impl<S: DistroBoxScheme> DistroBox<S> {
    pub fn new() -> DistroBox<S> {
        if std::env::var("INSIDE_DISTROBOX").is_err() {
            let proc = Exec::shell("distrobox-host-exec true").capture();
            let context = match proc {
                Ok(c) => c,
                Err(_) => return DistroBox(PhantomData)
            };
            if context.success() {
                unsafe {
                    std::env::set_var("INSIDE_DISTROBOX","true");
                }
            }
        }
        DistroBox(PhantomData)
    }
}

impl<S: DistroBoxScheme> Module for DistroBox<S> {
    fn append_segments(&mut self, segments: &mut Vec<Segment>) -> R<()> {

        let distrobox = std::env::var("INSIDE_DISTROBOX").is_ok();
        
        if distrobox {
            segments.push(Segment::simple(format!(" {} ", S::DISTROBOX_SYMBOL), S::DISTROBOX_FG, S::DISTROBOX_BG));
        }
        Ok(())
    }
}
