use crate::tasks::Task;
use anyhow::{Context, Result};
use core::panic;
use std::{env, fmt::Display, fs::File, io::Read, path::PathBuf};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum FilesType {
    Task,
    Example,
    Custom(&'static str),
}

impl Display for FilesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Task => write!(f, ""),
            Self::Example => write!(f, "example"),
            Self::Custom(name) => write!(f, "example_{name}"),
        }
    }
}

#[allow(dead_code)]
enum FileIO {
    In,
    Out,
}

impl Display for FileIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::In => write!(f, "in"),
            Self::Out => write!(f, "out"),
        }
    }
}

pub struct Files {
    dir: PathBuf,
}

impl Files {
    pub fn new(dir: PathBuf) -> Self {
        if !dir.exists() {
            panic!("Directory {dir:?} must exists.");
        }
        if !dir.is_dir() {
            panic!("{dir:?} must be direcotry.");
        }
        Self { dir }
    }

    fn get_file(&self, file_type: FilesType, io: FileIO, task: &Task) -> PathBuf {
        let name = match file_type {
            FilesType::Task => format!("{io}.txt"),
            FilesType::Example | FilesType::Custom(_) => format!("{file_type}_{io}.txt"),
        };
        let dir = self
            .dir
            .join(task.year.to_string())
            .join(format!("{:0>2}", task.task_number));
        let file_path = dir.join(format!("{}_{name}", task.task_type));
        if file_path.exists() {
            file_path
        } else {
            dir.join(name)
        }
    }

    pub fn get_output(&self, file_type: FilesType, task: &Task) -> String {
        let path = self.get_file(file_type, FileIO::Out, task);
        let mut file = File::open(&path)
            .unwrap_or_else(|e| panic!("Failed to open file {:?}. Error: {e}", path));
        let mut ret = String::new();
        file.read_to_string(&mut ret)
            .unwrap_or_else(|e| panic!("Failed to read file {:?}. Error: {e}", path));
        ret.trim().to_string()
    }

    pub fn get_input(&self, file_type: FilesType, task: &Task) -> Result<String> {
        let path = self.get_file(file_type, FileIO::In, task);
        let mut file =
            File::open(&path).with_context(|| format!("Failed to open file {:?}", path))?;
        let mut ret = String::new();
        file.read_to_string(&mut ret)
            .with_context(|| format!("Failed to read fiel {:?}", path))?;
        Ok(ret)
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self::new(PathBuf::from(
            env::var("AOC_DATA").context("Env variable AOC_DATA must be set.")?,
        )))
    }
}
