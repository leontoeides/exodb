#[derive(Clone, Debug)]
pub struct Metadata {
    /// Signals that error correction (such as Reed-Solomon) was to be used to recover corrupted
    /// data.
    ///
    /// This flag is used to determine when values ought to be re-written or re-committed to a
    /// `redb` database table.
    recovered: bool,
}