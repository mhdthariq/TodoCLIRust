use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use crate::task::TaskList;
use crate::error::Result;

const FILE_PATH: &str = "tasks.json";

pub fn load_tasks() -> Result<TaskList> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)?;

    let reader = BufReader::new(file);
    match serde_json::from_reader(reader) {
        Ok(tasks) => Ok(tasks),
        Err(_) => Ok(TaskList::default()),
    }
}

pub fn save_tasks(tasks: &TaskList) -> Result<()> {
    let file = File::create(FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks)?;
    Ok(())
}
