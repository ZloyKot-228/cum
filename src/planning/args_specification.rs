use crate::parsing::arg_parser::Args;

// Templates
pub trait ArgsSpec
where
    Self: Sized,
{
    fn is_satisfied_by(&self, item: &Args) -> bool;

    fn and<B: ArgsSpec>(self, other: B) -> AndArgsSpec<Self, B> {
        AndArgsSpec {
            left: self,
            right: other,
        }
    }

    fn or<B: ArgsSpec>(self, other: B) -> OrArgsSpec<Self, B> {
        OrArgsSpec {
            left: self,
            right: other,
        }
    }

    fn not(self) -> NotArgsSpec<Self> {
        NotArgsSpec { wrapped: self }
    }
}

pub struct AndArgsSpec<A, B> {
    left: A,
    right: B,
}

pub struct OrArgsSpec<A, B> {
    left: A,
    right: B,
}

pub struct NotArgsSpec<W> {
    wrapped: W,
}

impl<A, B> ArgsSpec for AndArgsSpec<A, B>
where
    A: ArgsSpec,
    B: ArgsSpec,
{
    fn is_satisfied_by(&self, item: &Args) -> bool {
        self.left.is_satisfied_by(item) && self.right.is_satisfied_by(item)
    }
}

impl<A, B> ArgsSpec for OrArgsSpec<A, B>
where
    A: ArgsSpec,
    B: ArgsSpec,
{
    fn is_satisfied_by(&self, item: &Args) -> bool {
        self.left.is_satisfied_by(item) || self.right.is_satisfied_by(item)
    }
}

impl<W> ArgsSpec for NotArgsSpec<W>
where
    W: ArgsSpec,
{
    fn is_satisfied_by(&self, item: &Args) -> bool {
        !self.wrapped.is_satisfied_by(item)
    }
}

// Custom specifications
pub struct IncrementalBuild;
pub struct FullBuild;
pub struct IncrementalRun;
pub struct FullRun;

pub struct InitProject;

pub struct PrintHelp;
pub struct PrintVersion;

impl ArgsSpec for IncrementalBuild {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.as_ref().map(|s| s == "build").unwrap_or(false)
            && !item.have_flag("preset")
            && !item.have_flag("force")
            && !item.have_flag("f")
    }
}

impl ArgsSpec for IncrementalRun {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.as_ref().map(|s| s == "run").unwrap_or(false)
            && !item.have_flag("preset")
            && !item.have_flag("force")
            && !item.have_flag("f")
    }
}

impl ArgsSpec for FullBuild {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.as_ref().map(|s| s == "build").unwrap_or(false)
            && (item.have_flag("preset") || item.have_flag("force") || item.have_flag("f"))
    }
}

impl ArgsSpec for FullRun {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.as_ref().map(|s| s == "run").unwrap_or(false)
            && (item.have_flag("preset") || item.have_flag("force") || item.have_flag("f"))
    }
}

impl ArgsSpec for InitProject {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.as_ref().map(|s| s == "init").unwrap_or(false)
    }
}

impl ArgsSpec for PrintHelp {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.is_none() && (item.have_flag("h") || item.have_flag("help"))
    }
}

impl ArgsSpec for PrintVersion {
    fn is_satisfied_by(&self, item: &Args) -> bool {
        item.command.is_none() && (item.have_flag("v") || item.have_flag("version"))
    }
}
