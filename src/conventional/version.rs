use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum IncrementCommand {
    Major,
    Minor,
    Patch,
    Auto,
    NoBump,
    AutoPackage(String),
    AutoMonoRepoGlobal(Option<Increment>),
    Manual(String),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Increment {
    Major,
    Minor,
    Patch,
    NoBump,
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

impl Ord for Increment {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (increment, other) if increment == other => Ordering::Equal,
            (Increment::Major, _) => Ordering::Greater,
            (_, Increment::Major) => Ordering::Less,
            (Increment::Minor, _) => Ordering::Greater,
            (_, Increment::Minor) => Ordering::Less,
            (Increment::Patch, Increment::Patch) => Ordering::Equal,
            (Increment::NoBump, Increment::NoBump) => Ordering::Equal,
            (Increment::Patch, Increment::NoBump) => Ordering::Greater,
            (Increment::NoBump, Increment::Patch) => Ordering::Less,
        }
    }
}

impl PartialOrd for Increment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
// Auto version tests resides in test/ dir since it rely on git log
// To generate the version
mod test {
    // TODO
}
