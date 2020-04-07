use crate::prelude::*;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ignore;

#[cfg(feature = "write")]
impl<'a> Writable<'a> for Ignore {
    type WriterArray = Ignore;
    fn write_root<'b: 'a>(&'b self, _stream: &mut impl WriterStream) -> RootTypeId {
        RootTypeId::Void
    }
}

#[cfg(feature = "read")]
impl Readable for Ignore {
    type ReaderArray = Ignore;
    fn read(_sticks: DynRootBranch<'_>, _options: &impl DecodeOptions) -> ReadResult<Self> {
        Ok(Self)
    }
}

#[cfg(feature = "write")]
impl<'a> WriterArray<'a> for Ignore {
    type Write = Ignore;
    fn buffer<'b: 'a>(&mut self, _value: &'b Self::Write) {}
    fn flush(self, _stream: &mut impl WriterStream) -> ArrayTypeId {
        ArrayTypeId::Void
    }
}

#[cfg(feature = "read")]
impl ReaderArray for Ignore {
    type Read = Ignore;
    fn new(_sticks: DynArrayBranch<'_>, _options: &impl DecodeOptions) -> ReadResult<Self> {
        Ok(Ignore)
    }
    fn read_next(&mut self) -> Self::Read {
        Ignore
    }
}