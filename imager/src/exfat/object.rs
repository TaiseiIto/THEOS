use std::{
    ffi,
    fmt,
    fs,
    io::{
        BufReader,
        Read,
    },
    os::raw,
    path,
};

#[link(name="libstat")]
extern "C" {
    fn get_access_time(path: *const raw::c_char) -> u32;
    fn get_change_time(path: *const raw::c_char) -> u32;
    fn get_modification_time(path: *const raw::c_char) -> u32;
}

#[derive(Debug)]
pub struct Object {
    path: path::PathBuf,
    name: String,
    access_time: u32,
    change_time: u32,
    modification_time: u32,
    content: FileOrDirectory,
    children: Vec<Object>,
}

impl Object {
    pub fn new(path: path::PathBuf) -> Self {
        if !path.exists() {
            panic!("\"{}\" is not found.", path.display());
        }
        Self {
            path: path.to_path_buf(),
            name: match path.file_name() {
                Some(name) => match name.to_os_string().into_string() {
                    Ok(name) => name,
                    _ => String::from(""),
                },
                None => String::from(""),
            },
            access_time: {
                let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
                let path = ffi::CString::new(path).expect("Can't create CString.");
                let path: *const raw::c_char = path.as_ptr();
                unsafe {
                    get_access_time(path)
                }
            },
            change_time: {
                let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
                let path = ffi::CString::new(path).expect("Can't create CString.");
                let path: *const raw::c_char = path.as_ptr();
                unsafe {
                    get_change_time(path)
                }
            },
            modification_time: {
                let path: &str = path.to_str().expect("Can't convert PathBuf to &str");
                let path = ffi::CString::new(path).expect("Can't create CString.");
                let path: *const raw::c_char = path.as_ptr();
                unsafe {
                    get_modification_time(path)
                }
            },
            content: if path.is_file() {
                let file = fs::File::open(&path).expect(&format!("\"{}\" is not found.", path.display()));
                let mut file = BufReader::new(file);
                let mut bytes = Vec::<u8>::new();
                file.read_to_end(&mut bytes).expect(&format!("Can't read \"{}\".", path.display()));
                FileOrDirectory::File {
                    bytes,
                }
            } else if path.is_dir() {
                FileOrDirectory::Directory
            } else {
                panic!("\"{}\" is not a file or directory.", path.display());
            },
            children: {
                match fs::read_dir(path) {
                    Ok(dir) => dir
                        .into_iter()
                        .filter_map(|dir| dir.ok())
                        .map(|dir| Self::new(dir.path()))
                        .collect(),
                    _ => vec![],
                }
            },
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object.path = {}\n", self.path.display())?;
        write!(f, "object.name = {}\n", self.name)?;
        write!(f, "object.access_time = {}\n", self.access_time)?;
        write!(f, "object.change_time = {}\n", self.change_time)?;
        write!(f, "object.modification_time = {}\n", self.modification_time)?;
        write!(f, "object.content = {}\n", self.content)?;
        for (i, child) in self.children.iter().enumerate() {
            let child = format!("{}", child)
                .replace("object", &format!("object.child[{}]", i));
            write!(f, "{}\n", child)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
enum FileOrDirectory {
    File {
        bytes: Vec<u8>,
    },
    Directory,
}

impl fmt::Display for FileOrDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileOrDirectory::File {
                bytes,
            } => write!(f, "File\n{:x?}", bytes),
            FileOrDirectory::Directory => write!(f, "Directory"),
        }
    }
}

