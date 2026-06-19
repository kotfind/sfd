/// Why a path was skipped during indexing.
#[derive(Debug)]
pub enum SkipReason {
    Ignored,
    Pattern,
    NoLang,
}
