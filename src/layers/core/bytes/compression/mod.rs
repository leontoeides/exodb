#[cfg(not(feature = "compress-dictionaries"))]
mod standard;

#[cfg(feature = "compress-dictionaries")]
mod dictionary;