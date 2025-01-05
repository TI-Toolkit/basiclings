use std::{
    collections::HashMap,
    env,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use deku::prelude::*;
use fancy_regex::Regex;
use serde::Serialize;
use titokens::Tokens;

use crate::{
    lesson::{Test, Variable, VariableData},
    tools::{float_to_tifloat, tokenize, tokenizer},
};

#[derive(Debug)]
pub enum TestError {
    Io(io::Error),
    NoRom,
    TIFileParsing(deku::DekuError),
    CEmuCrashed(ExitStatus),
    Regex(Box<fancy_regex::Error>),
}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::Io(error) => writeln!(f, "Error occurred during tests: {}", error),
            TestError::NoRom => f.write_str("Please ensure there is a working TI84+CE rom file in the current directory. This will be used for testing your submissions.\nThe rom file must end with the file extension \".rom\".\n\nThere are many ways to obtain a rom image if you do not have one. Perhaps the easiest is to use CEmu's rom dump wizard."),
            TestError::TIFileParsing(deku_error) => writeln!(f, "Error parsing 8x file during tests:\n{}", deku_error),
            TestError::CEmuCrashed(exit_status) => writeln!(f, "CEmu crashed during tests: {}", exit_status),
            TestError::Regex(error) => writeln!(f, "Error parsing test regex: {}", error),
        }
    }
}

#[derive(Debug)]
pub enum ProgramTestResult {
    Pass,
    // a little more information about *what* failed
    Fail(String),
}

#[derive(Clone, Serialize)]
struct Program {
    name: String,
    #[serde(rename = "isASM")]
    is_asm: bool,
}

#[derive(Debug, DekuRead, DekuWrite, Eq)]
#[deku(endian = "little")]
pub struct TIEntry {
    #[deku(assert = "*flash_indicator == 0x0b || *flash_indicator == 0x0d")]
    flash_indicator: u16,
    #[deku(update = "self.data.len()")]
    var_data_length: u16,
    file_type: u8,
    pub name: [u8; 8],
    version: u8,
    flags: u8,
    #[deku(update = "self.data.len()")]
    var_data_length_2: u16,
    #[deku(count = "var_data_length_2")]
    data: Vec<u8>,
}

impl TIEntry {
    fn new(name: [u8; 8], file_type: u8, data: Vec<u8>) -> Self {
        TIEntry {
            flash_indicator: 0x0d,
            var_data_length: data.len() as u16,
            file_type,
            name,
            version: 0,
            flags: 0x00,
            var_data_length_2: data.len() as u16,
            data,
        }
    }

    fn size(&self) -> u16 {
        self.to_bytes().unwrap().len() as u16
    }

    fn checksum(&self) -> u16 {
        let bytes = self.to_bytes().unwrap();
        bytes.iter().fold(0, |a, &x| a.wrapping_add(x as u16))
    }
}

impl PartialEq for TIEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.data == other.data
    }
}

impl From<Variable> for TIEntry {
    fn from(value: Variable) -> Self {
        let mut name = [0u8; 8];

        let name_tokens: Vec<u8> = <Tokens as Into<Vec<u8>>>::into(tokenize(&value.name))
            .into_iter()
            .chain(std::iter::repeat(0u8))
            .take(8)
            .collect();
        name[..name_tokens.len()].copy_from_slice(&name_tokens);

        let (file_type, data): (u8, Vec<u8>) = match value.value {
            VariableData::String(token_text) => {
                let mut token_bytes: Vec<u8> = tokenize(&token_text).into();
                let mut len = (token_bytes.len() as u16).to_le_bytes().to_vec();
                len.append(&mut token_bytes);

                (0x04, len)
            }

            VariableData::RealList(list) => (
                0x01,
                (list.len() as u16)
                    .to_le_bytes()
                    .into_iter()
                    .chain(
                        list.into_iter()
                            .flat_map(|element| float_to_tifloat(element).to_raw_bytes()),
                    )
                    .collect::<Vec<u8>>(),
            ),

            VariableData::RealNumber(number) => {
                let data = float_to_tifloat(number).to_raw_bytes();
                (0x00, data.to_vec())
            }
        };

        TIEntry::new(name, file_type, data)
    }
}

impl From<Tokens> for TIEntry {
    fn from(value: Tokens) -> Self {
        let mut token_bytes: Vec<u8> = value.into();
        let mut len = (token_bytes.len() as u16).to_le_bytes().to_vec();
        len.append(&mut token_bytes);

        TIEntry::new(*b"TESTPROG", 0x05, len)
    }
}

// this is good enough for exactly my use here but is by no means good enough for general-purpose TI-File parsing.
#[derive(Debug, DekuRead, DekuWrite)]
#[deku(magic = b"**TI83F*\x1A\x0A")]
pub struct TIFile {
    product_id: u8,
    comment: [u8; 42],
    #[deku(update = "self.entry.size()")]
    data_length: u16,
    pub entry: TIEntry,
    #[deku(update = "self.entry.checksum()")]
    checksum: u16,
}

impl From<TIEntry> for TIFile {
    fn from(value: TIEntry) -> Self {
        Self {
            product_id: 0x00,
            comment: *b"Generated for BASIClings automated testing",
            data_length: value.size(),
            checksum: value.checksum(),

            entry: value,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct AutotesterConfig {
    rom: String,
    target: Program,
    sequence: Vec<String>,
    transfer_files: Vec<String>,
    hashes: HashMap<String, ()>,
}

impl AutotesterConfig {
    pub fn with_rom(rom_path: String) -> Self {
        Self {
            rom: rom_path,
            target: Program {
                name: "TESTPROG".to_owned(),
                is_asm: false,
            },
            sequence: vec![
                "action|launch".to_owned(),
                "delay|2000".to_owned(),
                "key|on".to_owned(),
            ],
            transfer_files: vec![],
            hashes: HashMap::new(),
        }
    }

    pub fn add_import(&mut self, path: String) {
        self.transfer_files.push(path)
    }

    pub fn add_export(&mut self, var_name: &str) {
        self.sequence
            .push(format!("saveVar|{}", translate_variable_name(var_name)));
    }
}

/// translate from the token sheets' accessible name into CEmu's preferred name for the variable.
fn translate_variable_name(var_name: &str) -> &str {
    match var_name {
        "L1" => "L\u{2081}",
        "L2" => "L\u{2082}",
        "L3" => "L\u{2083}",
        "L4" => "L\u{2084}",
        "L5" => "L\u{2085}",
        "L6" => "L\u{2086}",

        "theta" => "\u{03B8}",

        name => name,
    }
}

pub struct TestRunner {
    rom_path: Option<PathBuf>,
}

impl TestRunner {
    pub fn new() -> Self {
        TestRunner { rom_path: None }
    }

    pub fn find_rom(&mut self) -> Result<String, TestError> {
        if let Some(pathbuf) = &self.rom_path {
            if !fs::exists(pathbuf).map_err(TestError::Io)? {
                self.rom_path = None;
            } else {
                return Ok(pathbuf.to_str().unwrap().to_owned());
            }
        }

        let paths =
            fs::read_dir(env::current_dir().map_err(TestError::Io)?).map_err(TestError::Io)?;

        for entry in paths {
            let entry = entry.map_err(TestError::Io)?;
            if entry.file_name().to_string_lossy().ends_with(".rom") {
                self.rom_path = Some(entry.path().canonicalize().map_err(TestError::Io)?);
                return Ok(self.rom_path.as_ref().unwrap().to_str().unwrap().to_owned());
            }
        }

        Err(TestError::NoRom)
    }

    fn run_cemu_test(
        &mut self,
        program: Tokens,
        inputs: &Vec<Variable>,
        outputs: &Vec<Variable>,
    ) -> Result<ProgramTestResult, TestError> {
        let folder = tempfile::tempdir().map_err(TestError::Io)?;
        let folder_path = folder.path();

        let autotester_config_path = self.initialize_cemu_test(
            &folder_path.canonicalize().map_err(TestError::Io)?,
            program,
            inputs,
            outputs,
        )?;

        let autotester_path = if cfg!(debug_assertions) {
            env::current_dir()
        } else {
            env::current_exe()
        }
            .map_err(TestError::Io)?
            .join("autotester");
        let cemu_status = Command::new(autotester_path)
            .arg(&autotester_config_path)
            .current_dir(folder_path)
            .stdout(Stdio::null())
            .spawn()
            .expect("Failed to start CEmu process.")
            .wait()
            .map_err(TestError::Io)?;

        self.validate_cemu_test_state(
            cemu_status,
            &folder_path.canonicalize().map_err(TestError::Io)?,
            outputs,
        )
    }

    /// Sets up variables, autotester config, etc
    fn initialize_cemu_test(
        &mut self,
        folder: &Path,
        program: Tokens,
        inputs: &Vec<Variable>,
        outputs: &Vec<Variable>,
    ) -> Result<PathBuf, TestError> {
        let autotester_config_path = folder.join("autotester.json");
        let mut autotester_config = AutotesterConfig::with_rom(self.find_rom()?);
        for input in inputs {
            let destination_path =
                folder.join(input.name.clone() + "." + input.value.file_extension());

            autotester_config.add_import(destination_path.to_str().unwrap().to_owned());

            let entry: TIEntry = (*input).clone().into();
            let file: TIFile = entry.into();

            fs::write(
                destination_path,
                file.to_bytes().map_err(TestError::TIFileParsing)?,
            )
            .map_err(TestError::Io)?;
        }

        for output in outputs {
            autotester_config.add_export(&output.name);
        }

        let program_path = folder.join("TESTPROG.8xp");
        autotester_config.add_import(program_path.to_str().unwrap().to_owned());
        let program_file = fs::File::create(program_path).map_err(TestError::Io)?;
        let entry: TIEntry = program.into();
        let file: TIFile = entry.into();
        file.to_writer(&mut Writer::new(program_file), ())
            .map_err(TestError::TIFileParsing)?;

        serde_json::to_writer(
            fs::File::create(&autotester_config_path).map_err(TestError::Io)?,
            &autotester_config,
        )
        .unwrap();

        Ok(autotester_config_path)
    }

    fn validate_cemu_test_state(
        &self,
        cemu_status: ExitStatus,
        folder: &Path,
        outputs: &Vec<Variable>,
    ) -> Result<ProgramTestResult, TestError> {
        if !cemu_status.success() {
            Err(TestError::CEmuCrashed(cemu_status))
        } else {
            for output in outputs {
                let variable_name = output.name.clone();

                let actual_path = folder.join(format!(
                    "{}.{}",
                    translate_variable_name(&variable_name),
                    output.value.file_extension()
                ));

                if !fs::exists(&actual_path).map_err(TestError::Io)? {
                    if output.name == "Ans" {
                        return Ok(ProgramTestResult::Fail("Cannot find Ans; perhaps Ans is the wrong type at the end of your program.".to_owned()));
                    }

                    return Ok(ProgramTestResult::Fail(format!(
                        "Cannot find variable {}.",
                        variable_name
                    )));
                }

                let actual = TIFile::from_reader((
                    &mut fs::File::options()
                        .read(true)
                        .open(actual_path)
                        .map_err(TestError::Io)?,
                    0,
                ))
                .map_err(TestError::TIFileParsing)?
                .1
                .entry;

                let expected: TIEntry = output.clone().into();

                if expected.data != actual.data {
                    return Ok(ProgramTestResult::Fail(format!(
                        "Incorrect value for variable {}.",
                        variable_name
                    )));
                }
            }

            Ok(ProgramTestResult::Pass)
        }
    }

    /// Test program.
    ///
    /// If any of the root tests pass, the program passes.
    /// All of the tests in a test group must pass for the whole group to pass.
    pub fn run_tests(
        &mut self,
        program: Tokens,
        tests: &Vec<Test>,
    ) -> Result<ProgramTestResult, TestError> {
        let mut last_result = ProgramTestResult::Fail("Tests failed.".to_owned());
        for test in tests {
            last_result = self.run_test(program.clone(), test)?;
            if let ProgramTestResult::Pass = last_result {
                return Ok(ProgramTestResult::Pass);
            }
        }

        Ok(last_result)
    }

    fn run_test(&mut self, program: Tokens, test: &Test) -> Result<ProgramTestResult, TestError> {
        match test {
            Test::CEmu { input, output } => self.run_cemu_test(program, input, output),
            Test::FulltextMatch { regex } => {
                let regex_result = Regex::new(&("^".to_owned() + regex + "$"))
                    .unwrap()
                    .is_match(&program.to_string(tokenizer()))
                    .map_err(|err| TestError::Regex(Box::new(err)))?;

                if regex_result {
                    Ok(ProgramTestResult::Pass)
                } else {
                    Ok(ProgramTestResult::Fail("Tests failed.".to_owned()))
                }
            }
            Test::Group(group) => {
                for test in group {
                    if let ProgramTestResult::Fail(reason) = self.run_test(program.clone(), test)? {
                        return Ok(ProgramTestResult::Fail(reason));
                    }
                }

                Ok(ProgramTestResult::Pass)
            }
        }
    }
}
