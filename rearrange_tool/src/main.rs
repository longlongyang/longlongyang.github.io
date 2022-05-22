#![feature(path_file_prefix)]

use std::{fs, path::Path};

use regex::Regex;

fn main() {
    let src_scan_path = Path::new("blog\\src");
    let docs_scan_path = Path::new("docs\\blog");
    for entry in src_scan_path.read_dir().expect("read src dir call failed") {
        if let Ok(entry) = entry {
            if entry.path().extension().is_some()
                && entry.path().extension().unwrap() == "rs"
                && entry.file_name() != "lib.rs"
            {
                let blog_name = entry.path().file_prefix().unwrap().to_owned();
                let blog_constants =
                    fs::read_to_string(entry.path()).expect("Unable to read file.");
                let const_re = Regex::new(r#"pub\sconst\s(.*?)\s*:.*?;"#).unwrap();
                let const_str: Vec<&str> = const_re
                    .captures_iter(&blog_constants)
                    .map(|c| c.get(1).unwrap().as_str())
                    .collect();

                for html_entry in docs_scan_path
                    .read_dir()
                    .expect("read html dir call failed")
                {
                    if let Ok(html_entry) = html_entry {
                        if html_entry.path().file_prefix().is_some()
                            && html_entry.path().file_prefix().unwrap() == blog_name
                            && html_entry.path().join("index.html").exists()
                        {
                            let html_entry_index = html_entry.path().join("index.html");
                            let content = fs::read_to_string(html_entry_index.clone())
                                .expect("Unable to read file.");

                            let content = content.replace("\n", "");

                            let div_re =
                                Regex::new(r#"<div\sclass="item-row">.*?</div></div>"#).unwrap();
                            let div_str: Vec<&str> =
                                div_re.find_iter(&content).map(|m| m.as_str()).collect();
                            let mut div_left: Vec<&str> = div_re.split(&content).collect();
                            div_left.reverse();

                            // reconstruct the content
                            let mut new_content = div_left.pop().unwrap().to_owned();
                            for cstr in const_str.clone().into_iter() {
                                for dstr in div_str.clone().into_iter() {
                                    let mut target_str = "::".to_owned();
                                    target_str.push_str(cstr);
                                    if dstr.contains(&target_str) {
                                        new_content.push_str(dstr);
                                        break;
                                    }
                                }
                            }
                            fs::write(html_entry_index, new_content).expect("Can't write to file.");
                        }
                    }
                }
            }
        }
    }
}
