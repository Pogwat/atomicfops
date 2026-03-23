use std::fs::rename;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    atomic_write("/home/pog/text.rsssss",".tmp", "hi_rust_world")?;
    atomic_remove_file("/home/pog/text.rsssss",".tmp")?;
    atomic_create_dir("/home/pog/russsstttt",".tmp")?;
    atomic_remove_dir("/home/pog/russsstttt",".tmp")?;
    Ok(())
}


    macro_rules! atomic_construct {
    ($pathref:ident, $tmp_extension_ref:ident, $name:ident($($arg:ident),*)) => {{
        let (path,tmp_extension) = ($pathref.as_ref(),$tmp_extension_ref.as_ref());
        let mut tmp_path = PathBuf::from(path);
        tmp_path.add_extension(tmp_extension);

        $name(&tmp_path, $($arg),*)?;
        rename(&tmp_path, &path)?; 
        Ok(())     
    }};
}

    macro_rules! atomic_destruct {
    ($pathref:expr, $tmp_extension_ref:expr, $name:ident($($arg:expr),*)) => {{
        let (path,tmp_extension) = ($pathref.as_ref(),$tmp_extension_ref.as_ref());
        let mut tmp_path = PathBuf::from(path);
        tmp_path.add_extension(tmp_extension);

        rename(&path, &tmp_path)?; 
        $name(&tmp_path, $($arg),*)?;
        Ok(())
        
    }};
}


use std::fs::write;

fn atomic_write<P:AsRef<Path>, E:AsRef<OsStr>, C: AsRef<[u8]> >(path:P, tmp_extension:E, content:C ) -> std::io::Result<()> {
    atomic_construct!(path,tmp_extension, write(content))
}

//SAME AS BELOW

// fn atomic_write<P:AsRef<Path>, E:AsRef<OsStr>, C: AsRef<[u8]> >(path:P, tmp_extension:E, content:C ) -> std::io::Result<()> {
//     let (path,tmp_extension) = (path.as_ref(),tmp_extension.as_ref());
//     let mut tmp_path = PathBuf::from(path);
//     tmp_path.add_extension(tmp_extension);

//     write(&tmp_path, content)?;
//     rename(&tmp_path, &path)?; //If this fails a tmp_file is left in the directory but path isint modified
//     Ok(())
// }
//fs::symlink("a.txt", "b.txt")?;

use std::fs::remove_file;

fn atomic_remove_file<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
    atomic_destruct!(path,tmp_extension, remove_file())
}

//SAME AS BELOW

// fn atomic_remove_file<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
//     let (path,tmp_extension) = (path.as_ref(),tmp_extension.as_ref());
//     let mut tmp_path = PathBuf::from(path);
//     tmp_path.add_extension(tmp_extension);

//     rename(&path,&tmp_path)?;
//     remove_file(&tmp_path)?;
//     Ok(())
// }

use std::fs::remove_dir;

fn atomic_remove_dir<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
    atomic_destruct!(path,tmp_extension,remove_dir() )
}

use std::fs::create_dir;

fn atomic_create_dir<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
        atomic_construct!(path,tmp_extension, create_dir())
}






