use crate::memory_reader::memory_reader_trait::IMemoryReader;
use squalr_engine_api::structures::structs::valued_struct::ValuedStruct;
use squalr_engine_processes::process_info::OpenedProcessInfo;
use std::os::raw::c_void;

pub struct MacosMemoryReader;

impl MacosMemoryReader {
    pub fn new() -> Self {
        MacosMemoryReader
    }
}

impl IMemoryReader for MacosMemoryReader {
    fn read_struct(
        &self,
        process_info: &OpenedProcessInfo,
        address: u64,
        valued_struct: &mut ValuedStruct,
    ) -> bool {
        false
    }
}
