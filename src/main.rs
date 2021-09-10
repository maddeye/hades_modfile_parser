use std::fs;

use hades_modmanager::{Parser, Statement};
fn main() -> std::io::Result<()> {
    /*==============================================
        File Reading
    ================================================*/
    let mod_dir_path = "./Hades_Example/Content/Mods/";

    let mod_dir = fs::read_dir(mod_dir_path)?;

    let mut stm: Vec<Statement> = Vec::new();
    for dir in mod_dir {
        let path = &dir?.path();
        if !path.join("modfile.txt").exists() {
            continue;
        }

        let path = path.clone().into_os_string().into_string().unwrap();

        let content = fs::read_to_string(path.to_string() + "/modfile.txt")?;

        let mut parser = Parser::new(content.as_str(), path);

        stm.extend(parser.parse());
    }

    stm.sort_by_key(|k| (k.target.to_string(), k.priority));

    Ok(())
}
