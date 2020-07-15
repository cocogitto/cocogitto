pub struct Commit {
    pub commit_type: String,
    pub scope: Option<String>,
    pub description: String,
}

impl Commit {
  pub fn from_raw_message(message: &str) -> Self {

    
    let split : Vec<&str> = message.split(": ").collect();
    let description = split[1].to_owned();

    let left_part: Vec<&str> = split[0].split("(").collect();
    let commit_type = left_part[0].to_owned();
    let scope = left_part
    .get(1)
    .map(|scope| scope[0..scope.len() -1 ].to_owned());

    Commit {
        commit_type, 
        scope,
        description,
    }
  }
}


#[cfg(test)] 
mod test {
  use super::Commit;

  #[test]
  fn should_map_conventional_commit_message_to_struct() {
    // Arrange
    let message = "feat(database): add postgresql driver";

    // Act
    let commit = Commit::from_raw_message(message);
    
    // Assert
    assert_eq!(commit.commit_type, "feat".to_owned());
    assert_eq!(commit.scope, Some("database".to_owned()));
    assert_eq!(commit.description, "add postgresql driver".to_owned());
  }
}
