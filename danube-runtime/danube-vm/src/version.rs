pub const MAJOR: u32 = 0;
pub const MINOR: u32 = 1;
pub const PATCH: u8 = 0;
pub const RELEASE_LEVEL: &'static str = "nightly";
pub const ADDITIONAL_INFO: Option<&'static str> = None;

#[inline]
pub fn version_number() -> String {
    format!("{}.{}.{}", MAJOR, MINOR, PATCH)
}

pub fn version() -> String {
    if let Some(additional_info) = ADDITIONAL_INFO {
        format!("{}-{}-{}", version_number(), RELEASE_LEVEL, additional_info)
    } else {
        format!("{}-{}", version_number(), RELEASE_LEVEL)
    }
}
