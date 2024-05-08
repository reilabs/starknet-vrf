use sha2::digest::DynDigest;

#[derive(Clone, Default)]
pub struct PedersenHash;

impl DynDigest for PedersenHash {
    fn update(&mut self, data: &[u8]) {
        todo!()
    }

    fn finalize_into(self, buf: &mut [u8]) -> Result<(), sha2::digest::InvalidBufferSize> {
        todo!()
    }

    fn finalize_into_reset(
        &mut self,
        out: &mut [u8],
    ) -> Result<(), sha2::digest::InvalidBufferSize> {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn output_size(&self) -> usize {
        todo!()
    }

    fn box_clone(&self) -> Box<dyn DynDigest> {
        todo!()
    }
}
