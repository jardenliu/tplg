use std::fs;
use std::path::Path;
use crate::utils::{get_user_info, get_version, get_date, get_name_variants};

pub fn generate_files(template_dir: &Path, output_dir: &Path) {
    let name = output_dir.file_name().and_then(|n| n.to_str()).unwrap_or("default");

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory: {}", e);
        return;
    }

    if let Ok(entries) = fs::read_dir(template_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let src_path = entry.path();
                let file_name = src_path.file_name().unwrap().to_str().unwrap().replace("{{name}}", name).replace(".tpl", "");
                let dest_path = output_dir.join(file_name);

                if let Ok(content) = fs::read_to_string(&src_path) {
                    let (pascal_name, snake_name, kebab_name, constant_name) = get_name_variants(name);
                    let (username, email) = get_user_info();
                    let version = get_version();
                    let current_date = get_date();
                    let new_content = content.replace("{{name}}", name).replace("{{pascalName}}", &pascal_name).replace("{{currentDate}}", &current_date).replace("{{version}}", &version).replace("{{username}}", &username).replace("{{email}}", &email).replace("{{snakeName}}", &snake_name).replace("{{kebabName}}", &kebab_name).replace("{{constantName}}", &constant_name);
                    if let Err(e) = fs::write(&dest_path, new_content) {
                        eprintln!("Failed to write file {}: {}", dest_path.display(), e);
                    }
                } else if let Err(e) = fs::read_to_string(&src_path) {
                    eprintln!("Failed to read file {}: {}", src_path.display(), e);
                }
            }
        }
    }
    println!("Successfully generated files in {}", output_dir.display());
}