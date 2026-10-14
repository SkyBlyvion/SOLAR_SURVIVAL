//! Aventures indépendantes : un répertoire nommé et une session par profil.
use super::*;

pub fn validate_name(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty()
        || name.chars().count() > 40
        || !name
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, ' ' | '_' | '-'))
    {
        return Err("Nom : 1 à 40 lettres, chiffres, espaces, tirets ou _".into());
    }
    let upper = name.to_uppercase();
    if ["CON", "PRN", "AUX", "NUL"].contains(&upper.as_str())
        || (1..=9).any(|n| upper == format!("COM{n}") || upper == format!("LPT{n}"))
    {
        return Err("Ce nom est réservé par le système ; choisissez un autre nom".into());
    }
    Ok(name.to_owned())
}
pub fn list(root: &Path) -> Result<Vec<String>> {
    let entries = match fs::read_dir(root) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("Lire les aventures : {e}")),
    };
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if validate_name(&name).as_deref() != Ok(name.as_str()) {
            continue;
        }
        if entry.path().join("session.solar").is_file()
            || entry.path().join("session.solar.bak").is_file()
        {
            names.push(name);
        }
    }
    names.sort_by_key(|s| s.to_lowercase());
    Ok(names)
}
fn path(root: &Path, name: &str) -> Result<PathBuf> {
    let name = validate_name(name)?;
    let dir = root.join(name);
    let metadata =
        fs::symlink_metadata(&dir).map_err(|e| format!("Répertoire d'aventure : {e}"))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err("Le profil doit être un répertoire local".into());
    }
    Ok(dir.join("session.solar"))
}
pub fn refresh(world: &mut World) -> Result<()> {
    let root = world.resource::<SaveControl>().root.clone();
    let names = list(&root)?;
    let mut c = world.resource_mut::<SaveControl>();
    c.profiles = names;
    c.selected = c.selected.min(c.profiles.len().saturating_sub(1));
    Ok(())
}
pub(super) fn save_current(world: &World) -> Result<()> {
    let c = world.resource::<SaveControl>();
    if let Some(name) = &c.active_profile {
        save(world, &path(&c.root, name)?)?;
    }
    Ok(())
}
/// Le nom est validé avant création ; un profil existant n'est jamais écrasé.
pub fn create(world: &mut World, name: &str) -> Result<()> {
    let name = validate_name(name)?;
    let root = world.resource::<SaveControl>().root.clone();
    if list(&root)?
        .iter()
        .any(|n| n.to_lowercase() == name.to_lowercase())
        || root.join(&name).exists()
    {
        return Err("Cette aventure existe déjà ; choisissez un autre nom".into());
    }
    let fresh = Session::fresh(world)?;
    save_current(world)?;
    fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    let dir = root.join(&name);
    fs::create_dir(&dir).map_err(|e| format!("Créer l'aventure : {e}"))?;
    let target = dir.join("session.solar");
    if let Err(e) = write_session(&fresh, &target) {
        let _ = fs::remove_dir(&dir);
        return Err(e);
    }
    let names = list(&root)?;
    fresh.restore(world)?;
    {
        let mut c = world.resource_mut::<SaveControl>();
        c.path = target;
        c.active_profile = Some(name.clone());
        c.draft.clear();
        c.selected = names.iter().position(|n| n == &name).unwrap_or(0);
        c.profiles = names;
    }
    if let Some(mut menu) = world.get_resource_mut::<MenuState>() {
        menu.play();
    }
    Ok(())
}
pub fn open(world: &mut World, name: &str) -> Result<bool> {
    let name = validate_name(name)?;
    let c = world.resource::<SaveControl>();
    let target = path(&c.root, &name)?;
    if c.active_profile
        .as_deref()
        .is_some_and(|active| active != name)
    {
        save_current(world)?;
    }
    let backup = load(world, &target)?;
    let mut c = world.resource_mut::<SaveControl>();
    c.path = target;
    c.selected = c.profiles.iter().position(|n| n == &name).unwrap_or(0);
    c.active_profile = Some(name);
    Ok(backup)
}
