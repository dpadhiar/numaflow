use crate::message::{Message, Offset};

/// [User-Defined Source] extends Numaflow to add custom sources supported outside the builtins.
///
/// [User-Defined Source]: https://numaflow.numaproj.io/user-guide/sources/user-defined-sources/
pub(crate) mod user_defined;

/// Set of items that has to be implemented to become a Source.
pub(crate) trait Source {
    #[allow(dead_code)]
    /// Name of the source.
    fn name(&self) -> &'static str;

    async fn read(&mut self) -> crate::Result<Vec<Message>>;

    /// acknowledge an offset. The implementor might choose to do it in an asynchronous way.
    async fn ack(&mut self, _: Vec<Offset>) -> crate::Result<()>;

    #[allow(dead_code)]
    /// number of partitions processed by this source.
    fn partitions(&self) -> Vec<u16>;
}