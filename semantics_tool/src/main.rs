#![feature(path_file_prefix)]

use std::{fs, path::Path};

fn main() {
    let main_js =
        fs::read_to_string(Path::new("docs").join("main.js")).expect("Unable to read main.js");
    let main_js = main_js.replace("<h3>Crates</h3>", "<h3>Navigation</h3>");
    let main_js = main_js.replace("Constants", "Sections");
    let main_js = main_js.replace("Modules", "Posts");
    fs::write(Path::new("docs").join("main.js"), main_js).expect("Can't write to main.js.");

    let docs_scan_path = Path::new("docs\\blog");

    let index =
        fs::read_to_string(docs_scan_path.join("index.html")).expect("Unable to read index.html.");

    let index = index.replace("blog - Rust", "Cryp.ren");
    let index = index.replace("Crate", "Navigation");
    let index = index.replace("Crates", "Navigation");
    let index = index.replace("Modules", "Posts");

    fs::write(docs_scan_path.join("index.html"), index).expect("Can't write to index.html.");

    let all =
        fs::read_to_string(docs_scan_path.join("all.html")).expect("Unable to read all.html.");

    let all = all.replace("Crate", "Navigation");
    let all = all.replace("Constants", "Sections");

    fs::write(docs_scan_path.join("all.html"), all).expect("Can't write to all.html.");

    // every post
    for html_entry in docs_scan_path
        .read_dir()
        .expect("read html dir call failed")
    {
        if let Ok(html_entry) = html_entry {
            if html_entry.path().join("index.html").exists() {
                for single_html_entry in html_entry
                    .path()
                    .read_dir()
                    .expect("read single file call failed")
                {
                    if let Ok(single_html_entry) = single_html_entry {
                        if single_html_entry.path().extension().unwrap() == "html" {
                            let html_entry_single = single_html_entry.path();
                            let content = fs::read_to_string(html_entry_single.clone())
                                .expect("Unable to read file.");

                            let content = content.replace("Module", "Post");
                            let content = content.replace("Constant", "Section");
                            let content = content.replace("Constants", "Sections");

                            fs::write(html_entry_single, content).expect("Can't write to file.");
                        }
                    }
                }
            }
        }
    }
}
