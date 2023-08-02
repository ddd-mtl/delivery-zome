
pub const REMOTE_ENDPOINT: &'static str = "receive_delivery_dm";

pub const DIRECT_SEND_TIMEOUT_MS: usize = 1000;
pub const DIRECT_SEND_CHUNK_TIMEOUT_MS: usize = 10000;

// const CHUNK_MAX_SIZE: usize = 1 * 1024 * 1024;
pub const CHUNK_MAX_SIZE: usize = 200 * 1024;
pub const PARCEL_MAX_SIZE: usize = 10 * 1024 * 1024;
pub const NAME_MIN_LENGTH: usize = 2;

/// Listing all Holochain Path used in this DNA
pub const DIRECTORY_PATH: &'static str = "directory";


/// WARNING Must use these names in the dna definition file
pub const DELIVERY_ZOME_NAME: &'static str = "zDelivery";
pub const DELIVERY_INTERGRITY_ZOME_NAME: &'static str = "zDeliveryIntegrity";