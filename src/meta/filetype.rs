use color::{ColoredString, Colors, Elem};
use std::fs::Metadata;
use std::os::unix::fs::FileTypeExt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum FileType {
    BlockDevice,
    CharDevice,
    Directory,
    File,
    SymLink,
    Pipe,
    Socket,
    Special,
}

impl<'a> From<&'a Metadata> for FileType {
    fn from(meta: &'a Metadata) -> Self {
        let file_type = meta.file_type();

        if file_type.is_file() {
            FileType::File
        } else if file_type.is_dir() {
            FileType::Directory
        } else if file_type.is_fifo() {
            FileType::Pipe
        } else if file_type.is_symlink() {
            FileType::SymLink
        } else if file_type.is_char_device() {
            FileType::CharDevice
        } else if file_type.is_block_device() {
            FileType::BlockDevice
        } else if file_type.is_socket() {
            FileType::Socket
        } else {
            FileType::Special
        }
    }
}

impl FileType {
    pub fn render(self, colors: &Colors) -> ColoredString {
        match self {
            FileType::File => colors.colorize(String::from("."), &Elem::File),
            FileType::Directory => colors.colorize(String::from("d"), &Elem::Dir),
            FileType::Pipe => colors.colorize(String::from("|"), &Elem::Pipe),
            FileType::SymLink => colors.colorize(String::from("l"), &Elem::SymLink),
            FileType::BlockDevice => colors.colorize(String::from("b"), &Elem::BlockDevice),
            FileType::CharDevice => colors.colorize(String::from("c"), &Elem::CharDevice),
            FileType::Socket => colors.colorize(String::from("s"), &Elem::Socket),
            FileType::Special => colors.colorize(String::from("?"), &Elem::Special),
        }
    }
}
