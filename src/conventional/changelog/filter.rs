use std::collections::HashMap;
use tera::{dotted_pointer, to_value, try_get_value, Value};

// From git-cliff: https://github.com/orhun/git-cliff/blob/main/git-cliff-core/src/template.rs
pub fn upper_first_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let mut s = tera::try_get_value!("upper_first_filter", "value", String, value);
    let mut c = s.chars();
    s = match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };
    Ok(tera::to_value(&s)?)
}

// filter commit with no scope
pub fn unscoped(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let mut arr = try_get_value!("unscoped", "scope", Vec<Value>, value);
    if arr.is_empty() {
        return Ok(arr.into());
    }

    let value = args.get("value").unwrap_or(&Value::Null);

    arr = arr
        .into_iter()
        .filter(|v| {
            let val = dotted_pointer(v, "scope").unwrap_or(&Value::Null);
            if value.is_null() {
                val == value
            } else {
                !val.is_null()
            }
        })
        .collect::<Vec<_>>();

    Ok(to_value(arr).unwrap())
}

// group commits and order by type
pub fn group_by_type(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
    let arr = try_get_value!("group_by_type", "type", Vec<Value>, value);
    let mut map = HashMap::new();

    for v in arr {
        let val = dotted_pointer(&v, "type").unwrap_or(&Value::Null);
        let val = val.as_str().unwrap_or_default();
        let val = val.to_string();

        let entry = map.entry(val).or_insert_with(Vec::new);
        entry.push(v);
    }

    // Sort the output by keys, checking the sort_order on the commit
    let mut keys = map.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| {
        let a = a.as_str();
        let b = b.as_str();
        let a = &map[a][0];
        let b = &map[b][0];
        let a = &a["type_order"].as_u64().unwrap_or(0);
        let b = &b["type_order"].as_u64().unwrap_or(0);
        a.cmp(b)
    });

    let mut out_vec = Vec::new();

    for key in keys {
        out_vec.push((key, map[key].clone()));
    }

    Ok(to_value(out_vec).unwrap())
}

// get all unique contributors from commits
pub fn unique_contributors(
    value: &Value,
    _: &HashMap<String, Value>,
) -> Result<Value, tera::Error> {
    let arr = try_get_value!("contributors", "value", Vec<Value>, value);
    let mut contributors = std::collections::HashSet::new();

    for commit in arr {
        // Try committer_username first
        if let Some(username) = dotted_pointer(&commit, "committer_username") {
            if let Some(username_str) = username.as_str() {
                if !username_str.is_empty() {
                    contributors.insert(username_str.to_string());
                    continue;
                }
            }
        }

        // Try author_username next
        if let Some(username) = dotted_pointer(&commit, "author_username") {
            if let Some(username_str) = username.as_str() {
                if !username_str.is_empty() {
                    contributors.insert(username_str.to_string());
                    continue;
                }
            }
        }

        // Fallback to raw git committer
        if let Some(committer) = dotted_pointer(&commit, "commit.committer") {
            if let Some(committer_str) = committer.as_str() {
                if !committer_str.is_empty() {
                    contributors.insert(committer_str.to_string());
                    continue;
                }
            }
        }

        // Fallback to raw git author
        if let Some(author) = dotted_pointer(&commit, "commit.author") {
            if let Some(author_str) = author.as_str() {
                if !author_str.is_empty() {
                    contributors.insert(author_str.to_string());
                }
            }
        }
    }

    let mut contributors_vec: Vec<String> = contributors.into_iter().collect();
    contributors_vec.sort();

    Ok(to_value(contributors_vec).unwrap())
}
