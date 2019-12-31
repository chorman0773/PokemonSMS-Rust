
extern crate openssl;
use crate::io::InputStream;
use crate::cipher::Cipher;


pub struct CipherInputStream<'a,'b> {
    wrapped: &'a dyn InputStream,
    cipher: &'b dyn Cipher
}
