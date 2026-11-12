use std::path::Path;
use std::path::PathBuf;


fn main() -> std::io::Result<()>{

    let valutFile = getVaultFilePath();

    if valutFile.exists(){
        let password = rpassword::prompt_password("Master password: ")?;
    } else {
        let (mut password, mut repeatPassword) = getNewPassword()?;

        while password != repeatPassword {
            println!("Passwords do not match");
            (password, repeatPassword) = getNewPassword()?;
        }

        std::fs::write(valutFile, "")?;
        println!("Vault createed");

    }

    Ok(())
}

fn getNewPassword() -> std::io::Result<(String,String)>{
    let password = rpassword::prompt_password("Create master password: ")?;
    let repeatPassword = rpassword::prompt_password("Reapeat password: ")?;
    Ok((password, repeatPassword))
}

fn getVaultFilePath() -> PathBuf {
    let home = dirs::home_dir().expect("no home dir found");
    home.join(".vault.oxl")
}
