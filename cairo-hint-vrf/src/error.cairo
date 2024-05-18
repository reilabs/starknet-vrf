#[derive(Drop)]
pub enum Error {
    ProofVerificationError,
    PointAtInfinity,
}