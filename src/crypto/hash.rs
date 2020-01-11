
pub trait Hash{
    const SIZE: usize;
    fn init() -> Self;
    fn update(&mut self,input:&[u8]);
    fn do_final(&mut self,output:&mut [u8;Self::SIZE]);

    fn hash(input: &[u8],output: &mut [u8;Self::SIZE]){
        let mut algorithm = Self::init();
        algorithm.update(input);
        alogrithm.do_final(out);
    }
}