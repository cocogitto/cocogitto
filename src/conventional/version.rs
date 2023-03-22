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
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Increment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (increment, other) if increment == other => Some(Ordering::Equal),
            (Increment::Major, _) => Some(Ordering::Greater),
            (_, Increment::Major) => Some(Ordering::Less),
            (Increment::Minor, _) => Some(Ordering::Greater),
            (_, Increment::Minor) => Some(Ordering::Less),
            (Increment::Patch, Increment::Patch) => Some(Ordering::Equal),
            (Increment::NoBump, Increment::NoBump) => Some(Ordering::Equal),
            (Increment::Patch, Increment::NoBump) => Some(Ordering::Greater),
            (Increment::NoBump, Increment::Patch) => Some(Ordering::Less),
        }
    }
}

#[cfg(test)]
// Auto version tests resides in test/ dir since it rely on git log
// To generate the version
mod test {
    // TODO
}
