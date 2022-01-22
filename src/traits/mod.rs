mod simple;

pub use simple::AbstractSimpleModel;

pub trait AbstractDatabase: Sync + Send + AbstractSimpleModel {}
