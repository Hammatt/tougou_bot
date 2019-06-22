pub enum VNDBResult {
    Single(String),
    MostLikelyAndMore(String, String),
    None,
}
