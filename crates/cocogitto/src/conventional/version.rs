use cocogitto_tag::increment::Increment;

#[derive(Debug, PartialEq, Eq, Default)]
pub enum IncrementCommand {
    Major,
    Minor,
    Patch,
    #[default]
    Auto,
    NoBump,
    AutoPackage(String),
    AutoMonoRepoGlobal(Option<Increment>),
    Manual(String),
}

impl From<Increment> for IncrementCommand {
    fn from(value: Increment) -> Self {
        match value {
            Increment::Major => IncrementCommand::Major,
            Increment::Minor => IncrementCommand::Minor,
            Increment::Patch => IncrementCommand::Patch,
            Increment::NoBump => IncrementCommand::NoBump,
        }
    }
}

#[cfg(test)]
// Auto version tests resides in test/ dir since it rely on git log
// To generate the version
mod test {
    // TODO
}
