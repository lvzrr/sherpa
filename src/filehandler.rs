use chrono::{DateTime, Local};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

/* TODO: redo all this shit, it's ass honestly */
/* ALSO READ FILES AS BYTES, ONLY IF DISPLAYED CHANGE TO SRTING */

#[derive(Hash)]
pub struct File {
    pub name: String,
    lastmodtime: String,
}
pub struct Directory {
    pub files: Vec<File>,
    pub children: Vec<Directory>,
    pub hashmap: Vec<u64>,
    pub childrenhashes: Vec<u64>,
    pub path: PathBuf,
    pub name: String,
}
pub struct Database {
    pub children: Vec<Directory>,
    pub file_arr: Vec<File>,
    pub hashmap: Vec<u64>,
    pub childrenhashes: Vec<u64>,
}

fn recursive_analysis(path: &PathBuf, name: &String) -> Directory {
    let mut folder: Directory = Directory {
        path: path.to_owned(),
        files: Vec::new(),
        children: Vec::new(),
        hashmap: Vec::new(),
        childrenhashes: Vec::new(),
        name: name.to_owned(),
    };
    let entries = fs::read_dir(&path).expect("Could not open directory");
    for entry in entries {
        let item = entry.unwrap();
        if !Path::join(&path, Path::new(&item.file_name())).is_dir() {
            let f: File = File {
                name: String::from(item.file_name().to_string_lossy()),
                lastmodtime: (DateTime::from(item.metadata().unwrap().modified().unwrap())
                    as DateTime<Local>)
                    .format("%h:%m:%s %d/%m/%Y")
                    .to_string(),
            };
            let hash_check = calculate_hash(&f.name);
            if !folder.hashmap.contains(&hash_check) {
                println!("\tPushin file: {} | Hash: {}", &f.name, &hash_check);
                folder.hashmap.push(hash_check);
                folder.files.push(f);
            } else {
                println!("\tIgnoring file: {} | Hash: {}", &f.name, &hash_check);
            }
        } else {
            let hash: u64 = calculate_hash(&item.file_name());
            println!(
                "Folder found: {} | Hash {}",
                &item.file_name().into_string().unwrap(),
                hash
            );
            if !folder
                .childrenhashes
                .contains(&calculate_hash(&item.file_name()))
            {
                folder.children.push(recursive_analysis(
                    &Path::join(&path, Path::new(&item.file_name())),
                    &item.file_name().into_string().unwrap(),
                ));
                folder
                    .childrenhashes
                    .push(calculate_hash(&item.file_name()));
            } else {
                println!("Ignoring {}", item.file_name().into_string().unwrap());
            }
        }
    }
    return folder;
}

pub fn show_dir(dir: &Directory) {
    println!("+---[Folder]: {}", dir.name);
    for file in &dir.files {
        println!("[File]: {}", file.name);
    }
    for dir in &dir.children {
        show_dir(dir);
    }
}

impl Database {
    pub fn generate() -> Database {
        println!("\nInitializing database...\n");

        let mut database: Database = Database {
            file_arr: Vec::new(),
            hashmap: Vec::new(),
            children: Vec::new(),
            childrenhashes: Vec::new(),
        };

        let entries = fs::read_dir(".").expect("Could not open directory");
        for entry in entries {
            let item = entry.unwrap();
            if !Path::new(&item.file_name()).is_dir() {
                let f: File = File {
                    name: String::from(item.file_name().to_string_lossy()),
                    lastmodtime: (DateTime::from(item.metadata().unwrap().modified().unwrap())
                        as DateTime<Local>)
                        .format("%h:%m:%s %d/%m/%Y")
                        .to_string(),
                };
                let hash_check = calculate_hash(&f.name);
                if !database.hashmap.contains(&hash_check) {
                    println!("Pushin file: {} | Hash: {}", &f.name, &hash_check);
                    database.hashmap.push(hash_check);
                    database.file_arr.push(f);
                } else {
                    println!("Ignoring file: {} | Hash: {}", &f.name, &hash_check);
                }
            } else {
                let hash: u64 = calculate_hash(&item.file_name());
                if !database.childrenhashes.contains(&hash) {
                    println!(
                        "Folder found: {} | Hash {}",
                        &item.file_name().into_string().unwrap(),
                        hash
                    );
                    let childpath = Path::join(Path::new("."), Path::new(&item.file_name()));
                    database.children.push(recursive_analysis(
                        &childpath,
                        &item.file_name().into_string().unwrap(),
                    ));
                    database.childrenhashes.push(hash);
                } else {
                    println!(
                        "Ignoring Folder: {} | Hash: {} ",
                        &item.file_name().into_string().unwrap(),
                        hash
                    )
                }
            }
        }
        return database;
    }
    pub fn analyze(&mut self) {
        println!("\nUpdating database...\n");
        let entries = fs::read_dir(".").expect("Could not open directory");
        for entry in entries {
            let item = entry.unwrap();
            if !Path::new(&item.file_name()).is_dir() {
                let f: File = File {
                    name: String::from(item.file_name().to_string_lossy()),
                    lastmodtime: (DateTime::from(item.metadata().unwrap().modified().unwrap())
                        as DateTime<Local>)
                        .format("%d/%m/%Y %T")
                        .to_string(),
                };
                let hash_check = calculate_hash(&f.name);
                if !self.hashmap.contains(&hash_check) {
                    println!("Pushin file: {} | Hash: {}", &f.name, &hash_check);
                    self.hashmap.push(hash_check);
                    self.file_arr.push(f);
                } else {
                    println!("Ignoring {}", f.name);
                }
            } else {
                let hash: u64 = calculate_hash(&item.file_name());
                if !self.childrenhashes.contains(&hash) {
                    println!(
                        "Folder found: {} | Hash {}",
                        &item.file_name().into_string().unwrap(),
                        hash
                    );
                    self.children.push(recursive_analysis(
                        &Path::join(Path::new("."), Path::new(&item.file_name())),
                        &item.file_name().into_string().unwrap(),
                    ));
                    self.childrenhashes.push(hash);
                } else {
                    println!(
                        "Ignoring Folder: {} | Hash: {} ",
                        &item.file_name().into_string().unwrap(),
                        hash
                    )
                }
            }
        }
    }
    pub fn show(&self) {
        for file in &self.file_arr {
            println!("[File]: {}", file.name);
        }
        for dir in &self.children {
            show_dir(dir);
        }
    }
}

pub fn calculate_hash<T: Hash>(f: &T) -> u64 {
    let mut obj = DefaultHasher::new();
    f.hash(&mut obj);
    obj.finish()
}

pub fn lookup(filename: String, db: &Database) -> PathBuf {
    let reference: u64 = calculate_hash(&filename);
    let mut cont: i32 = 0;
    for hash in &db.hashmap {
        if hash == &reference {
            return Path::join(Path::new("."), Path::new(&db.file_arr[cont as usize].name));
        }
        cont += 1;
    }
    cont = 0;
    for dir in &db.children {
        return dirlookup(
            &reference,
            &dir,
            &Path::join(Path::new("."), Path::new(&dir.name)),
        );
    }
    return Path::join(Path::new(""), Path::new("error"));
}

pub fn get_file_contents(path: PathBuf) -> String {
    return fs::read_to_string(&path).unwrap();
}

fn dirlookup(hash: &u64, folder: &Directory, path: &PathBuf) -> PathBuf {
    let mut cont = 0;
    for h in &folder.hashmap {
        if h == hash {
            return Path::join(
                folder.path.as_path(),
                Path::new(&folder.files[cont as usize].name),
            );
        }
        cont += 1;
    }
    for dir in &folder.children {
        return dirlookup(
            &hash,
            &dir,
            &Path::join(path.as_path(), Path::new(&dir.name)),
        );
    }
    return Path::join(Path::new(""), Path::new("error"));
}

pub fn overwrite_file_contents(path: PathBuf, contents: String) -> () {
    fs::remove_file(&path).unwrap();
    return fs::write(&path, contents).unwrap();
}

pub fn delete_file(path: PathBuf) -> () {
    fs::remove_file(&path).unwrap();
}
