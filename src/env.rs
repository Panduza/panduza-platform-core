use std::env::consts::OS;
use std::path::PathBuf;
use std::str::FromStr;

static DEFAULT_DIR_UNIX: &str = "/etc/panduza";
static DEFAULT_DIR_WINDOWS: &str = r"C:\ProgramData\Panduza";

///
/// Provide the default directory where to put platform configs (tree.json, connection.json...)
///
pub fn system_default_config_dir() -> Result<PathBuf, std::io::Error> {
    match OS {
        "linux" => Ok(PathBuf::from_str(DEFAULT_DIR_UNIX).unwrap()),
        "windows" => Ok(PathBuf::from(dirs::public_dir().unwrap()).join("panduza")),
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported OS",
            ))
        }
    }
}

///
///
///
pub fn system_default_device_tree_file() -> Result<PathBuf, std::io::Error> {
    let tree_path = system_default_config_dir().unwrap().join("tree.json");
    Ok(tree_path)
}

///
/// Provide a list of all the possible location for plugins
///
pub fn system_plugins_dir_paths() -> Vec<PathBuf> {
    let mut res = Vec::new();
    // res.push(value);
    // a coté du binaire
    // si windows c:/
    let path = std::env::current_exe().unwrap();
    let parent = path.parent().unwrap();
    let ppp = parent.join("plugins");
    // println!("The current directory is {}", ppp.display()); // cd/plugins

    res.push(ppp);

    // main and alternate

    let windows_path = PathBuf::from(dirs::public_dir().unwrap())
        .join("panduza")
        .join("plugins");
    // println!("The current directory is {}", windows_path.display()); // cd/plugins

    res.push(windows_path);

    return res;
}
