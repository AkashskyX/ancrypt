// dir_walk.rs

use std::fs;
use std::path::Path;
use std::io;
use walkdir::WalkDir;
use super::encrypt::xor_encrypt;


pub fn walk_dir_decrypt(source: &str , target: &str,  key: &[u8]) ->  io::Result<()> {


    if !Path::new(target).exists(){
        fs::create_dir(target)?;
    }

    for entry in WalkDir::new(source){
        let entry = entry.unwrap();
        //println!("{:?}" , entry) ;
        let entry_path = entry.path() ;
        if entry_path.is_file(){
            println!("Decrypting file : {}" , entry_path.display());

           let contents =  fs::read(entry_path)?;
          

           let encrypted = xor_encrypt(&contents, &key);

            
            let relative_path = match entry_path.strip_prefix(source){
                Ok(path) => path,
                Err(_) => continue ,
            };
            let new_path = Path::new(target).join(relative_path);

            fs::write(&new_path , encrypted)?;





            
        }
       
    }

    Ok(())

}



pub fn walk_dir_encrypt(source: &str , target: &str , key: &[u8]) ->  io::Result<()> {

    for entry in WalkDir::new(source){
        let entry = entry.unwrap();
        //println!("{:?}" , entry) ;
        let entry_path = entry.path() ;
        if entry_path.is_file(){
            println!("Encrypting file : {}" , entry_path.display());

           let contents =  fs::read(entry_path)?;
           //let key = b"xvideo52.com";

           let encrypted = xor_encrypt(&contents, &key);

            
            let relative_path = match entry_path.strip_prefix(source){
                Ok(path) => path,
                Err(_) => continue ,
            };
            let new_path = Path::new(target).join(relative_path);

            fs::write(&new_path , encrypted)?;





            
        }
       
    }

    Ok(())

}
