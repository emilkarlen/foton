use std::path::Path;

pub fn is_existing_dir(p: &Path) -> Result<(), String>
{
    if !p.is_dir() {
        let s = format!("not a dir: {}\n", p.display());
        Err(s)
    } else {
        Ok(())
    }
}
