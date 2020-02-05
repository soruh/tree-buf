use crate::prelude::*;

// TODO: impl Writable for () {

#[derive(Default)]
pub struct BoxWriterArray<T> {
    inner: T,
}

impl<'a, T: Writable<'a>> Writable<'a> for Box<T> {
    type WriterArray = BoxWriterArray<T::WriterArray>;
    fn write_root<'b: 'a>(value: &'b Self, bytes: &mut Vec<u8>, lens: &mut Vec<usize>) -> RootTypeId {
        T::write_root(&value, bytes, lens)
    }
}

pub struct BoxReaderArray<T> {
    inner: T,
}

impl<T: Readable> Readable for Box<T> {
    type ReaderArray = BoxReaderArray<T::ReaderArray>;
    fn read(sticks: DynRootBranch<'_>) -> ReadResult<Self> {
        Ok(Box::new(T::read(sticks)?))
    }
}

impl<'a, T: WriterArray<'a>> WriterArray<'a> for BoxWriterArray<T> {
    type Write = Box<T::Write>;
    fn buffer<'b: 'a>(&mut self, value: &'b Self::Write) {
        self.inner.buffer(&value)
    }
    fn flush(self, bytes: &mut Vec<u8>, lens: &mut Vec<usize>) -> ArrayTypeId {
        self.inner.flush(bytes, lens)
    }
}

impl<T: ReaderArray> ReaderArray for BoxReaderArray<T> {
    type Read = Box<T::Read>;
    fn new(sticks: DynArrayBranch<'_>) -> ReadResult<Self> {
        Ok(BoxReaderArray { inner: T::new(sticks)? })
    }
    fn read_next(&mut self) -> ReadResult<Self::Read> {
        Ok(Box::new(self.inner.read_next()?))
    }
}