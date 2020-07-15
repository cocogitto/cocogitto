use cocha::get_changelog_from_tags;
use cocha::get_changelog;

fn main() {
  get_changelog("f09ead914c65ceedf06d0daf5d920d7bd26e6a84", "6cb24ca7befdbf24dee1c98a3f29d3c4e0474b75").unwrap();
  println!("TAGS =================");
  get_changelog_from_tags("0.0.1", "0.0.2").unwrap();
}