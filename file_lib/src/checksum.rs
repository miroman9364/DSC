// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use digest::Digest;
use hex_string::HexString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use strum_macros::Display;

/// The supported checksum hash algorithms.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display)]
pub enum Algorithm {
    /// The SHA-1 algorithm. (Considered insecure.)
    Sha1,
    /// The SHA-256 algorithm.
    Sha256,
    /// The SHA-512 algorithm.
    Sha512,
}

/// Compute the checksum for a vector of bytes using a given algorithm.
/// 
/// # Arguments
/// 
/// * `bytes` - The vector of bytes to compute the checksum for.
/// * `algorithm` - The algorithm to use to compute the checksum.
/// 
/// # Return value
/// 
/// The checksum as a lowercase string.
///
/// # Examples
///
/// ## Can compute SHA-1 checksum
/// 
/// ```
/// # use file_lib::checksum::{compute_checksum, Algorithm};
/// let checksum = compute_checksum(b"hello world", Algorithm::Sha1);
/// assert_eq!(checksum, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
/// ```
/// 
/// ## Can compute SHA-256 checksum
/// 
/// ```
/// # use file_lib::checksum::{compute_checksum, Algorithm};
/// let checksum = compute_checksum(b"hello world", Algorithm::Sha256);
/// assert_eq!(checksum, "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
/// ```
/// 
/// ## Can compute SHA-512 checksum
/// 
/// ```
/// # use file_lib::checksum::{compute_checksum, Algorithm};
/// let checksum = compute_checksum(b"hello world", Algorithm::Sha512);
/// assert_eq!(checksum, "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f");
/// ```
/// 
/// ## Checksum is lowercase string
/// 
/// ```
/// # use file_lib::checksum::{compute_checksum, Algorithm};
/// let checksum = compute_checksum(b"hello world", Algorithm::Sha512);
/// assert_eq!(checksum, checksum.to_lowercase());
/// ```
pub fn compute_checksum(bytes: &[u8], algorithm: Algorithm) -> String {
    match algorithm {
        Algorithm::Sha1 => {
            let mut hasher = Sha1::new();
            hasher.update(bytes);
            let result = hasher.finalize();

            HexString::from_bytes(&result.to_vec()).as_string().to_lowercase()
        },
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(bytes);
            let result = hasher.finalize();

            HexString::from_bytes(&result.to_vec()).as_string().to_lowercase()
        },
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(bytes);
            let result = hasher.finalize();

            HexString::from_bytes(&result.to_vec()).as_string().to_lowercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::checksum;

    #[test]
    fn can_compute_sha1_checksum() {
        let checksum = checksum::compute_checksum(b"hello world", checksum::Algorithm::Sha1);
        assert_eq!(checksum, "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
    }
}
