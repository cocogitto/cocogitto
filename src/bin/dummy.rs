use cocha::get_changelog_from_tags;
use cocha::get_changelog;

fn main() {
  let oid_ch = get_changelog("8806a55727b6c1767cca5d494599623fbb5dd1dd", "8c6ca0db0e67173e519507cad1aec7de69d0f9d9" ).unwrap();
  println!("{}", oid_ch);
  //let tag_ch = get_changelog_from_tags("0.1.0", "0.2.0").unwrap();
}