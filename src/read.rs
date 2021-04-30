extern crate crossbeam_channel;
use mech_core::{hash_string, TableIndex, Table, Value, ValueType, ValueMethods, Transaction, Change, TableId, Register};
use mech_utilities::{Machine, MachineRegistrar, RunLoopMessage};
//use std::sync::mpsc::{self, Sender};
use std::thread::{self};
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::fs;

lazy_static! {
  static ref FILE_READ: u64 = hash_string("file/read");
  static ref PATH: u64 = hash_string("path");
  static ref CONTENTS: u64 = hash_string("contents");
}

export_machine!(file_read, file_read_reg);

extern "C" fn file_read_reg(registrar: &mut dyn MachineRegistrar, outgoing: Sender<RunLoopMessage>) -> String {
  registrar.register_machine(Box::new(Read{outgoing}));
  "#file/read = [|path contents|]".to_string()
}

#[derive(Debug)]
pub struct Read {
  outgoing: Sender<RunLoopMessage>,
}

impl Machine for Read {

  fn name(&self) -> String {
    "file/read".to_string()
  }

  fn id(&self) -> u64 {
    Register{table_id: TableId::Global(*FILE_READ), row: TableIndex::All, column: TableIndex::All}.hash()
  }

  fn on_change(&mut self, table: &Table) -> Result<(), String> {
    for i in 1..=table.rows {
      let filename = table.get_string(&TableIndex::Index(i), &TableIndex::Alias(*PATH));
      match filename {
        Some(filename) => {
          let path = filename.clone();
          let outgoing = self.outgoing.clone();
          let row = TableIndex::Index(i);
          let reader_handle = thread::spawn(move || {
            let contents = fs::read_to_string(path).expect("Error reading file"); // TODO Send error
            outgoing.send(RunLoopMessage::Transaction(Transaction{changes: vec![
              Change::Set{table_id: *FILE_READ, values: vec![(row, TableIndex::Alias(*CONTENTS), Value::from_string(&contents))]},
              Change::InternString{string: contents},
            ]}));
          });
        }
        None => (), // TODO Send error
      }
    }
    Ok(())
  }
}