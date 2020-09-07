use cocha::get_changelog_from_tags;
use cocha::get_changelog;

fn main() {
  let oid_ch = get_changelog("ed849a93096262f4eb706061dee71af730572946", "396c3f29c55e0905609e6641b99897b8a5d50f33").unwrap();
  println!("{}", oid_ch);
  println!("TAGS =================");
  let tag_ch = get_changelog_from_tags("0.1.0", "0.2.0").unwrap();
  println!("{}", tag_ch);
}