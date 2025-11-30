use crate::crypto::{sha1, sha256};
use crate::mtproto::Side;
use crate::utils::SliceExt;

#[derive(Clone)]
pub struct AuthKey {
    data: [u8; 256],
    hash: [u8; 20],
}

impl AuthKey {
    pub fn new(data: [u8; 256]) -> Self {
        let hash = sha1!(&data);

        Self { data, hash }
    }

    #[inline]
    pub fn data(&self) -> &[u8; 256] {
        &self.data
    }

    #[inline]
    pub fn id(&self) -> &[u8; 8] {
        self.hash[12..20].arr()
    }

    /// Compute msg_key.
    pub(super) fn msg_key(&self, buffer: &[u8], padding: &[u8], side: Side) -> [u8; 16] {
        let x = side.x();

        // SHA256(substr(auth_key, 88 + x, 32) + plaintext + random_padding);
        let msg_key_large = sha256!(&self.data[88 + x..88 + x + 32], buffer.as_ref(), padding);

        // msg_key = substr(msg_key_large, 8, 16);
        let msg_key = *msg_key_large[8..24].arr();

        msg_key
    }

    /// Compute aes_key, aes_iv.
    pub(super) fn compute(&self, msg_key: &[u8; 16], side: Side) -> ([u8; 32], [u8; 32]) {
        let x = side.x();

        // sha256_a = SHA256(msg_key + substr(auth_key, x, 36));
        let sha256_a = sha256!(msg_key, &self.data[x..x + 36]);

        // sha256_b = SHA256(substr(auth_key, 40 + x, 36) + msg_key);
        let sha256_b = sha256!(&self.data[40 + x..40 + x + 36], msg_key);

        // aes_key = substr(sha256_a, 0, 8) + substr(sha256_b, 8, 16) + substr(sha256_a, 24, 8);
        let mut aes_key = [0u8; 32];

        aes_key[0..8].copy_from_slice(&sha256_a[0..8]);
        aes_key[8..24].copy_from_slice(&sha256_b[8..24]);
        aes_key[24..32].copy_from_slice(&sha256_a[24..32]);

        // aes_iv = substr(sha256_b, 0, 8) + substr(sha256_a, 8, 16) + substr(sha256_b, 24, 8);
        let mut aes_iv = [0; 32];

        aes_iv[0..8].copy_from_slice(&sha256_b[0..8]);
        aes_iv[8..24].copy_from_slice(&sha256_a[8..24]);
        aes_iv[24..32].copy_from_slice(&sha256_b[24..32]);

        (aes_key, aes_iv)
    }
}
