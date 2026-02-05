use std::path::{Path, PathBuf};

pub fn relativize_path(base: &Path, target: &Path) -> PathBuf {
    let mut result = PathBuf::new();
    let base_abs = base.canonicalize().unwrap_or(base.to_path_buf());
    let target_abs = target.canonicalize().unwrap_or(target.to_path_buf());

    println!("base_abs: {:?}", base_abs);
    println!("target_abs: {:?}", target_abs);

    let mut base_iter = base_abs.components();
    let mut target_iter = target_abs.components();

    let mut remaining_base = Vec::new();
    let mut remaining_target = Vec::new();

    loop {
        match (base_iter.next(), target_iter.next()) {
            (Some(b), Some(t)) if b == t => continue,
            (b, t) => {
                if let Some(comp) = b {
                    remaining_base.push(comp);
                }
                for comp in base_iter {
                    remaining_base.push(comp);
                }
                if let Some(comp) = t {
                    remaining_target.push(comp);
                }
                for comp in target_iter {
                    remaining_target.push(comp);
                }
                break;
            }
        }
    }

    println!("remaining_base: {:?}", remaining_base);
    println!("remaining_target: {:?}", remaining_target);

    for _ in &remaining_target {
        result.push("..");
    }
    for comp in remaining_base {
        result.push(comp);
    }

    if result.as_os_str().is_empty() {
        result.push(".");
    }

    result
}

fn main() {
    let base = Path::new("/Users/devel0per/Code/ohosdemo/gen-ohos");
    let target = Path::new("/Users/devel0per/Code/ohosdemo/gen-ohos/entry");
    let rel = relativize_path(base, target);
    println!("Result: {:?}", rel);

    let base2 = Path::new("/Users/devel0per/Code/ohosdemo");
    let target2 = Path::new("/Users/devel0per/Code/ohosdemo/gen-ohos/entry");
    let rel2 = relativize_path(base2, target2);
    println!("Result2: {:?}", rel2);
}
