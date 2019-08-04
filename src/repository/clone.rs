use url::{Url, ParseError};
extern crate custom_error;
use custom_error::custom_error;
use reqwest;
use crate::repository::build::FileSystemEntity;
custom_error! {pub CloneError
    FolderNotFound = "Folder not found!",
    NodeAppendError = "Node could not be appended",
    SQLError{source: rusqlite::Error} = "SQL Error",
    WalkDirError{source: walkdir::Error} = "Walkdir Error",
    CryptoError{source: easy_xxhash64::file_hash::CryptoError} = "Crypto Error",
    SerdeError{source: serde_json::Error} = "Serde Error",
    SystemTimeErr{source: std::time::SystemTimeError} = "System Time Error",
    IOError{source: std::io::Error} = "IO Error",
    ParseError{source: url::ParseError} = "Parse Error",
    RequestError{source: reqwest::Error} = "Request Error"
}


/// Clone an remote repository.
/// * `path` : Path where the data will be stored (can be relative or absolute)
/// * `url` : URL to the a3mo folder
pub fn clone(path: &str, url: &str) -> Result<(), CloneError>{
    Url::parse(url)?;

    let sync_url = url.to_owned() + "/sync.json";
    let mut sync_json_resp = reqwest::get(&sync_url)?;
    let jstring = sync_json_resp.text()?;

    println!("{:?}",&jstring);
    //let f: Node<FileSystemEntity> = serde_json::from_str(jstring.as_str())?;

    //println!("{:?}",&f);


    Ok(())
}