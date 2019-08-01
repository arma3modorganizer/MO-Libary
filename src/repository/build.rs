use crate::sql::sqlite;
use std::path::{Path};
use walkdir::{WalkDir, DirEntry};
use indextree::{Arena, NodeId};
use std::time::{SystemTime, UNIX_EPOCH};

extern crate rayon;
use rayon::prelude::*;

extern crate rusqlite;

extern crate custom_error;
use custom_error::custom_error;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json;
use serde_indextree::Node;
use crate::sql::sqlite::Repository;
use std::fs::{File, DirBuilder};
use self::rusqlite::Error;

custom_error! {pub BuildRepoError
    FolderNotFound = "Folder not found!",
    SQLError{source: rusqlite::Error} = "SQL Error",
    WalkDirError{source: walkdir::Error} = "Walkdir Error",
    CryptoError{source: easy_xxhash64::file_hash::CryptoError} = "Crypto Error",
    SerdeError{source: serde_json::Error} = "Serde Error",
    SystemTimeErr{source: std::time::SystemTimeError} = "System Time Error"
}

#[derive(Debug, Serialize)]
pub struct FileSystemEntity {
    pub name: String,
    pub is_folder: bool,
    pub hash: u64,
}
impl FileSystemEntity {
    pub fn new(name: &str, repo_path: &str) -> Result<FileSystemEntity, BuildRepoError> {
        let is_directory = Path::is_dir(name.as_ref());
        let mut xhash: u64 = 0;
        if !is_directory {
            xhash = easy_xxhash64::file_hash::hash_path(name.as_ref())?
        }
        println!("{:?}:\t{:?}", &name, &xhash);
        Ok(FileSystemEntity {
            name: String::from(name.replace(repo_path, "").trim_start_matches("\\")),
            is_folder: is_directory,
            hash: xhash,
        })
    }
}

fn build_tree(name: &str, arena: &mut Arena<FileSystemEntity>, repo_path: String, rayon: bool) -> Result<NodeId, BuildRepoError> {
    let mut node_map: HashMap<String, NodeId> = HashMap::new();

    let mut root_node: Option<NodeId> = None;

    //Insert nodes

    if rayon {
        let entries: Vec<std::result::Result<walkdir::DirEntry, walkdir::Error>> = WalkDir::new(&repo_path).into_iter().collect();
        let fsxe_s: Vec<FileSystemEntity> = entries.par_iter().map(|p| {
            let fa : &std::result::Result<walkdir::DirEntry, walkdir::Error> = p;
            let f = fa.as_ref().unwrap();

            let fname = f.path().to_str().unwrap();
            let fse = FileSystemEntity::new(fname, &repo_path.as_str()).unwrap();
            fse
        }).collect();

        for fse in fsxe_s{
            let fse_s = String::from(&fse.name);

            let node_id = arena.new_node(fse);

            if root_node.is_none() {
                root_node = Some(node_id);
            }
            node_map.insert(fse_s, node_id);
        }

    }else {
        for entry in WalkDir::new(&repo_path) {
            let f = entry?;

            let fname = f.path().to_str().unwrap();

            let fse = FileSystemEntity::new(fname, &repo_path.as_str())?;

            let fse_s = String::from(&fse.name);

            let node_id = arena.new_node(fse);

            if root_node.is_none() {
                root_node = Some(node_id);
            }

            node_map.insert(fse_s, node_id);
        }
    }

    for n in &node_map {
        let node_name = n.0;
        let node_index = n.1;

        //TODO Fix for linux
        let xf = match node_name.rfind("\\") {
            Some(v) => {v}
            None => {0}
        };
        let parent_name = node_name.split_at(xf).0;

        let parent = node_map.get(parent_name);
        match parent {
            Some(v) => {
                v.append(*node_index, arena);
            }
            None => {
                continue
            }
        }

    }

    Ok(root_node.unwrap())
}


/// (Re)build a repository
/// [name: &str] : Repository name (Has to be created using new command)
/// [fmt_json: bool] : Output formatted json
/// [rayon: bool] : Parallelize building using rayon (requires multiple cores/threads)
pub fn build(name: &str, fmt_json: bool, rayon: bool) -> Result<(), BuildRepoError> {
    let repo = sqlite::get_repository(name)?;

    if Path::exists(repo.path.as_ref()) == false {
        return Err(BuildRepoError::FolderNotFound);
    };

    println!("Building repository {:?}", &name);
    let start = SystemTime::now();

    let sync_folder_path : String = String::clone(&repo.path) + "\\.a3mo";
    //Delete sync folder
    std::fs::remove_dir_all(&sync_folder_path);

    let arena = &mut Arena::new();
    let root_node = build_tree(name, arena, String::clone(&repo.path), rayon)?;
    let sn = Node::new(root_node, arena);

    let json = if !fmt_json {
        serde_json::to_string(&sn)?
    }else{
        serde_json::to_string_pretty(&sn)?
    };

    //Create sync folder
    std::fs::create_dir(&sync_folder_path);

    //Save json at repo
    std::fs::write(sync_folder_path + "\\sync.json", json);

    println!("Finished building. {:?} sec", start.elapsed()?);

    Ok(())
}
