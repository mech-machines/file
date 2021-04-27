extern crate crossbeam_channel;
use mech_core::{hash_string, TableIndex, Table, Value, ValueType, ValueMethods, Transaction, Change, TableId, Register};
use mech_utilities::{Machine, MachineRegistrar, RunLoopMessage};
//use std::sync::mpsc::{self, Sender};
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;

lazy_static! {
  static ref FILE_WRITE: u64 = hash_string("file/write");
}

export_machine!(file_write, file_write_reg);

extern "C" fn file_write_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Read{outgoing}));
  "#file/write = [|path contents|]".to_string()
}

#[derive(Debug)]
pub struct Read {
  outgoing: Sender<RunLoopMessage>,
}

impl Machine for Read {

  fn name(&self) -> String {
    "file/write".to_string()
  }

  fn id(&self) -> u64 {
    Register{table_id: TableId::Global(*FILE_WRITE), row: TableIndex::All, column: TableIndex::All}.hash()
  }

  fn on_change(&mut self, table: &Table) -> Result<(), String> {

    Ok(())
  }
}