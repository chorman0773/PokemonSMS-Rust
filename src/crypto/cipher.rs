pub enum Mode{
    Encrypt,
    Decrypt
}

pub trait Cipher{
    const BLCK_SIZE: usize;
    fn update(&mut self,in_buf: &[u8],out_buf: &mut [u8])->usize;
    fn do_final(&mut self,in_buf: &[u8],out_buf: &mut [u8])->usize;
    fn mode(&self) -> Mode;
}