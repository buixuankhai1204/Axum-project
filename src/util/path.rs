use std::path::{Component, Path, PathBuf};

pub fn normalize<P: AsRef<Path>>(path: &P) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::CurDir | Component::RootDir => {},
            Component::ParentDir => {
                ret.pop();
            },
            Component::Normal(c) => {
                ret.push(c);
            },
        }
    }
    ret
}
