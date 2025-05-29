use std::{
    process::{Command, Stdio},
    sync::mpsc::Sender,
};

use threadpool::ThreadPool;

use crate::errors::ExecutionError;

pub type ProcSpawnRusult = Result<ProcOutput, ExecutionError>;

pub struct ProcSpawner;

pub struct ProcOutput {
    pub outs: String,
    pub errs: String,
    pub exit_code: i32,
}

impl ProcSpawner {
    pub fn spawn_and_wait(exe: &str, args: &[String]) -> ProcSpawnRusult {
        let output = Command::new(exe).args(args).output()?;

        let outs: String = String::from_utf8_lossy(&output.stdout).into();
        let errs: String = String::from_utf8_lossy(&output.stderr).into();
        let exit_code = output.status.code().unwrap_or(1);

        Ok(ProcOutput {
            outs,
            errs,
            exit_code,
        })
    }

    pub fn spawn_and_wait_owned(exe: String, args: Vec<String>) -> ProcSpawnRusult {
        let output = Command::new(exe).args(args).output()?;

        let outs: String = String::from_utf8_lossy(&output.stdout).into();
        let errs: String = String::from_utf8_lossy(&output.stderr).into();
        let exit_code = output.status.code().unwrap_or(1);

        Ok(ProcOutput {
            outs,
            errs,
            exit_code,
        })
    }

    pub fn spawn_into_pool(
        exe: String,
        args: Vec<String>,
        tp: &ThreadPool,
        tx: Sender<ProcSpawnRusult>,
    ) {
        tp.execute(move || {
            let res = Self::spawn_and_wait_owned(exe, args);
            tx.send(res).unwrap();
        });
    }

    /// Spawn process and inherit all stdio streams from parent. Returns exit_code.
    pub fn spawn_into_parent(exe: &str, args: &[String]) -> Result<i32, ExecutionError> {
        let mut handle = Command::new(exe)
            .args(args)
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .stdout(Stdio::inherit())
            .spawn()?;

        let code = handle.wait()?.code().unwrap_or(1);
        Ok(code)
    }
}
