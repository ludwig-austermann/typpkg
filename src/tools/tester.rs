use std::path::PathBuf;

use anyhow::Result;
use wax::Glob;

pub fn compile_tests(in_dir: &PathBuf) -> Result<()> {
    let test_dir = in_dir.join("tests");
    let temp_dir = std::env::temp_dir();
    log::trace!(
        "Starting tests with {} as the temp result directory.",
        temp_dir.display()
    );
    let phasher = image_hasher::HasherConfig::new().to_hasher();

    for entry in Glob::new("**/*.typ").unwrap().walk(&test_dir) {
        let entry = entry?;
        let old_image_path = entry.path().with_extension("png");
        if !old_image_path.exists() {
            log::warn!(
                "A compiled version of {} does not exist for {}.",
                old_image_path.display(),
                entry.path().display()
            );
            continue;
        }

        let new_image_path = temp_dir
            .join(entry.path().strip_prefix(&test_dir).unwrap())
            .with_extension("png");
        std::fs::create_dir_all(&*new_image_path.parent().unwrap())?;

        std::process::Command::new("typst")
            .arg("c")
            .arg(entry.path())
            .arg(&new_image_path)
            .current_dir(in_dir)
            .status()
            .unwrap_or_else(|e| {
                log::error!("Error compiling {}. {}", entry.path().display(), e);
                std::process::exit(0)
            });
        log::info!(
            "compiled {} to {}.",
            entry.path().display(),
            &new_image_path.display()
        );

        let img_old = image::open(&old_image_path)?;
        let phash_old = phasher.hash_image(&img_old);

        let img_new = image::open(&new_image_path)?;
        let phash_new = phasher.hash_image(&img_new);

        if phash_old.dist(&phash_new) > 0 {
            log::error!(
                "Test Faliure:
            old png: {}, phash: {}
            new png: {}, phash: {}
            diff: {}
            You can use a tool as odiff to compare the pictures.",
                &old_image_path.display(),
                phash_old.to_base64(),
                &new_image_path.display(),
                phash_new.to_base64(),
                phash_old.dist(&phash_new)
            );
        }
    }

    Ok(())
}
