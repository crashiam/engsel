use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;
use zip::result::ZipError;

pub struct XlsxArchive {
    archive: ZipArchive<File>,
}

impl XlsxArchive {
    pub fn new_from_path<P: AsRef<Path>>(path: P) -> Result<Self, ZipError> {
        let file = File::open(path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self { archive })
    }

    pub fn get_file_content(&mut self, file_name: &str) -> Result<Vec<u8>, ZipError> {
        let mut file = self.archive.by_name(file_name)?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn has_file(&mut self, file_name: &str) -> bool {
        self.archive.by_name(file_name).is_ok()
    }
}
