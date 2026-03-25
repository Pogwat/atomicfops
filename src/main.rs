use std::fs::rename;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    atomic_write("/home/pog/text.rsssss",".tmp", "hi_rust_world")?;
    atomic_remove_file("/home/pog/text.rsssss",".tmp")?;
    atomic_create_dir("/home/pog/russsstttt",".tmp")?;
    atomic_remove_dir("/home/pog/russsstttt",".tmp")?;
    atomic_write("/home/pog/text.rsssss1",".tmp", "hi_rust_world")?;
    atomic_write("/home/pog/text.rsssss2",".tmp", "I_AM_DIFFRENT")?;
    atomic_remove_file("/home/pog/text.rsssss1",".tmp")?;
    
    atomic_symlink("/home/pog/text.rsssss1",".tmp","/home/pog/text.rsssss2" )?;

    Ok(())
}


    macro_rules! atomic_construct {
    ($path:expr, $tmp_extension:expr, $name:ident($($arg:ident),*)) => {{
        let tmp_path = add_extension_to_path($path,$tmp_extension);
        $name(&tmp_path, $($arg),*)?;
        rename(&tmp_path, $path)?; 
        Ok(())     
    }};
}

    macro_rules! atomic_destruct {
    ($path:expr, $tmp_extension:expr, $name:ident($($arg:expr),*)) => {{
        let tmp_path = add_extension_to_path($path,$tmp_extension);
        rename($path, &tmp_path)?; 
        $name(&tmp_path, $($arg),*)?;
        Ok(())
        
    }};
}


fn add_extension_to_path<P:AsRef<Path>,E:AsRef<OsStr>>(path:&P, extension:&E) -> PathBuf {
    let (path,extension) = (path.as_ref(),extension.as_ref());
    let mut new_path = PathBuf::from(path);
    new_path.add_extension(extension);
    new_path
}

use std::fs::write;

fn atomic_write<P:AsRef<Path>, E:AsRef<OsStr>, C: AsRef<[u8]> >(path:P, tmp_extension:E, content:C ) -> std::io::Result<()> {
    atomic_construct!(&path,&tmp_extension, write(content))
}

use std::fs::remove_file;

fn atomic_remove_file<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
    atomic_destruct!(&path,&tmp_extension, remove_file())
}

use std::fs::remove_dir;

fn atomic_remove_dir<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {
    atomic_destruct!(&path,&tmp_extension,remove_dir() )
}

use std::fs::create_dir;

fn atomic_create_dir<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E) -> std::io::Result<()> {    
    atomic_construct!(&path,&tmp_extension, create_dir())
}

use std::os::unix::fs::symlink;

fn atomic_symlink<P:AsRef<Path>, E:AsRef<OsStr>>(path:P, tmp_extension:E, source:P) -> std::io::Result<()> {
    let tmp_path = add_extension_to_path(&path, &tmp_extension);

    symlink(source, &tmp_path)?; //ORDER IS FLIPPED
    rename(&tmp_path, &path)?; 
    Ok(())     
}