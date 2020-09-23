#[cfg(feature = "blst")]
pub use blstrs::{
    Bls12, Engine, Fp, Fp12, Fp2, FpRepr, G1Affine, G1Compressed, G1Projective, G1Uncompressed,
    G2Affine, G2Compressed, G2Prepared, G2Projective, G2Uncompressed, PairingCurveAffine, Scalar,
    ScalarRepr,
};

#[cfg(feature = "pairing")]
pub use paired::{
    bls12_381::{
        Bls12, Fq as Fp, Fq12 as Fp12, Fq2 as Fp2, FqRepr as FpRepr, Fr as Scalar,
        FrRepr as ScalarRepr, G1Affine, G1Compressed, G1Uncompressed, G2Affine, G2Compressed,
        G2Prepared, G2Uncompressed, G1 as G1Projective, G2 as G2Projective,
    },
    Engine, PairingCurveAffine,
};
