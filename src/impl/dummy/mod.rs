use crate::AbstractDatabase;

pub mod simple;

pub struct DummyDB;
impl AbstractDatabase for DummyDB {}
