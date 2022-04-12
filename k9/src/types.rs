pub type FilePath = String;

#[derive(Copy, Clone)]
pub enum UpdateMode {
    NoUpdate,
    InPlace,
    Corrected,
}
