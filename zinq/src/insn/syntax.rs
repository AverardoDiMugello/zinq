pub trait Decodable<W>: std::fmt::Debug + Clone {
    const FIXEDBITS: W;
    const FIXEDMASK: W;
}
