use crate::error::SchemeResult;
use crate::generator::Randomizer;
use crate::Scheme;

pub trait GeneratedValueWriter<W: std::io::Write> {
    fn from_writer(writer: W) -> Self;
    fn into_inner(self) -> W;
    fn flush(&mut self) -> SchemeResult<()>;
    fn write_after_all_generated<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        scheme: &Scheme<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemeResult<()>;
    fn write_with_generate<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        scheme: &Scheme<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemeResult<()>;
}
