pub use rkyv;

pub type ShakeBuf = [u8; 16];

pub const EOF: u8 = 0x03;
const CLIENT: ShakeBuf = [0x21; 16]; // ! in ascii
const SERVER: ShakeBuf = [0xA1; 16]; // ¡ in ascii

#[must_use]
pub const fn new_shake_buf() -> ShakeBuf {
    [0u8; 16]
}

#[must_use]
pub fn is_valid_server(dat: ShakeBuf) -> bool {
    dat == SERVER
}

#[must_use]
pub fn is_valid_client(dat: ShakeBuf) -> bool {
    dat == CLIENT
}

#[must_use]
pub fn gen_client() -> ShakeBuf {
    CLIENT
}

#[must_use]
pub fn gen_server() -> ShakeBuf {
    SERVER
}

#[cfg(test)]
mod test {
    use crate::camphor::{gen_client, gen_server, is_valid_client, is_valid_server};

    #[test]
    fn test_client_dat() {
        assert!(is_valid_client(gen_client()));
    }

    #[test]
    fn test_server_dat() {
        assert!(is_valid_server(gen_server()));
    }
}
