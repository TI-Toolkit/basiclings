use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
};

use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use inquire::{
    validator::{StringValidator, Validation},
    Autocomplete, Confirm, CustomUserError, Editor, Select, Text,
};
use markdown::mdast::{Code, Node};
use serde::{Deserialize, Serialize};

use crate::{
    cemu::{ProgramTestResult, TestRunner},
    lesson::Lesson,
    parser::parse_lessons,
    tools::{byte_count, process_submission},
};

const SAVE_PATH: &str = "basiclings_save.json";
const RECOVERY_PATH: &str = "basiclings_recovery.json";

const COMMANDS: [&str; 7] = [
    "help", "select", "next", "retry", "quit", "progress", "review",
];

pub struct UserInterface {
    lessons: BTreeMap<u16, (Node, Lesson)>,
    save: Save,
    test_runner: TestRunner,

    last_attempt: Option<u16>,
}

impl UserInterface {
    pub fn new() -> io::Result<Self> {
        let mut interface = UserInterface {
            lessons: parse_lessons(),
            save: Save::load()?,
            test_runner: TestRunner::new(),

            last_attempt: None,
        };

        interface.save();

        Ok(interface)
    }

    pub fn run(&mut self) {
        self.show_progress_report();

        loop {
            let command = Text::new("")
                .with_validator(MainPrompt)
                .with_autocomplete(MainPrompt)
                .prompt()
                .unwrap();

            match command.as_str() {
                "help" => Self::show_help(),

                "select" => {
                    if let Some(next_lesson_id) = self.select_lesson(&self.save.unlocked_lessons) {
                        self.execute_lesson(next_lesson_id);
                    } else {
                        eprintln!("Operation failed.")
                    }
                }
                "next" => {
                    if let Some(&next_lesson_id) = self.save.unlocked_lessons.first() {
                        self.execute_lesson(next_lesson_id);
                    } else {
                        println!("No lessons remaining!");
                    }
                }
                "retry" => {
                    if let Some(last_lesson_id) = self.last_attempt {
                        self.execute_lesson(last_lesson_id);
                    } else {
                        println!("No lesson to retry.")
                    }
                }

                "quit" => break,
                "progress" => self.show_progress_report(),
                "review" => {
                    if let Some(next_lesson_id) = self.select_lesson(&self.save.completed_lessons) {
                        todo!()
                    } else {
                        eprintln!("Operation failed.")
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    fn lesson_markdown(&self, current_lesson: u16) -> &Node {
        &self.lessons.get(&current_lesson).unwrap().0
    }

    fn lesson_data(&self, current_lesson: u16) -> &Lesson {
        &self.lessons.get(&current_lesson).unwrap().1
    }

    fn save(&mut self) {
        self.save.save().unwrap()
    }

    fn show_progress_report(&self) {
        let total = self.lessons.len();
        let unlocked = self.save.unlocked_lessons.len();
        let completed = self.save.completed_lessons.len();

        println!(
            "{:.2}% complete: {} completed / {} unlocked / {} unseen",
            100.0 * completed as f64 / total as f64,
            completed,
            unlocked,
            total - completed - unlocked,
        );
    }

    fn show_help() {
        println!("Welcome to BASIClings.");
    }

    fn print_lesson(&self, lesson_id: u16) {
        UserInterface::print_node(self.lesson_markdown(lesson_id));
    }

    fn print_node(node: &Node) {
        // does not support nested styling tags... we shouldn't need it?
        match node {
            Node::Text(text) => print!("{}", text.value),
            Node::InlineCode(inline_code) => print!("{}", inline_code.value),

            Node::Code(Code {
                lang: Some(lang), ..
            }) if lang == "json" => return,
            _ => {}
        }

        if let Some(children) = node.children() {
            children.iter().for_each(UserInterface::print_node);
        }

        if matches!(node, Node::Heading(_) | Node::Paragraph(_)) {
            println!("\n")
        }
    }

    fn execute_lesson(&mut self, lesson_id: u16) {
        self.print_lesson(lesson_id);

        let lesson_data = &self.lessons.get(&lesson_id).unwrap().1;

        let savings_message = format!(
            "Save {} byte{} to proceed. (target: {} bytes)",
            lesson_data.required_savings,
            if lesson_data.required_savings > 1 {
                "s"
            } else {
                ""
            },
            lesson_data.byte_threshold()
        );

        let boilerplate = self
            .save
            .attempts
            .get(&lesson_id)
            .cloned()
            .unwrap_or_else(|| {
                let objective = lesson_data
                    .brief_description
                    .clone()
                    .map_or("".to_owned(), |objective| {
                        "\n// Objective: ".to_owned() + &objective.replace("\n", "\n// ")
                    });
                "// Blank lines and lines starting with two forward slashes are ignored.\n"
                    .to_owned()
                    + &objective
                    + "\n// Original Program:\n// "
                    + &lesson_data.starting_program.replace("\n", "\n// ")
                    + "\n\n// "
                    + &savings_message
                    + "\n\n"
                    + &lesson_data.starting_program
            });

        let result = Editor::new(&savings_message)
            .with_predefined_text(&boilerplate)
            .with_file_extension(".8xp.txt")
            .prompt();

        if let Ok(raw_text) = result {
            if raw_text.trim() == "" {
                return;
            }

            self.save.attempts.insert(lesson_id, raw_text.clone());

            let tokens_struct = process_submission(raw_text);
            let tokens = tokens_struct.clone().collect::<Vec<_>>();
            let byte_count: usize = byte_count(&tokens);

            println!("{} tokens, {} bytes.", tokens.len(), byte_count);

            let byte_threshold = lesson_data.byte_threshold();
            if byte_count > byte_threshold {
                println!("Too large: target is {} bytes", byte_threshold);
                self.last_attempt = Some(lesson_id);
            } else {
                println!("Testing...");
                match self
                    .test_runner
                    .run_tests(tokens_struct, &lesson_data.tests)
                {
                    Err(test_error) => {
                        eprintln!("{}", test_error);
                        std::process::exit(1)
                    }

                    Ok(ProgramTestResult::Fail(reason)) => {
                        println!("{}", reason);
                        self.last_attempt = Some(lesson_id);
                    }
                    Ok(ProgramTestResult::Pass) => {
                        self.complete_lesson(lesson_id);
                        self.last_attempt = None;
                    }
                }
            }

            self.save();
        }
    }

    fn complete_lesson(&mut self, lesson_id: u16) {
        self.save.unlocked_lessons.remove(&lesson_id);
        self.save.completed_lessons.insert(lesson_id);

        // we could precompute this but it's not really necessary
        // when there are only a couple hundred lessons at most.
        let new_lessons = self
            .lessons
            .iter()
            .filter_map(|(&k, (_, v))| {
                if !self.save.completed_lessons.contains(&k)
                    && !self.save.unlocked_lessons.contains(&k)
                    && !v
                        .requirements
                        .iter()
                        .any(|x| !self.save.completed_lessons.contains(x))
                {
                    Some(k)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !new_lessons.is_empty() {
            println!(
                "Congratulations! You unlocked {} new lesson{}:",
                new_lessons.len(),
                if new_lessons.len() > 1 { "s" } else { "" }
            );

            new_lessons
                .iter()
                .for_each(|&lesson_id| println!(" - {}", self.lesson_data(lesson_id).name));
        } else {
            println!("Congratulations!")
        }

        self.save.unlocked_lessons.extend(new_lessons.iter());
        self.show_progress_report();
    }

    fn select_lesson(&self, set: &BTreeSet<u16>) -> Option<u16> {
        if set.len() == 1 {
            let lesson_id = *set.first().unwrap();
            let proceed = Confirm::new(&format!(
                "Only one lesson available: {}. Would you like to select it?",
                self.lesson_data(lesson_id).name
            ))
            .with_default(true)
            .prompt()
            .ok()?;

            if proceed {
                return Some(lesson_id);
            } else {
                return None;
            }
        } else {
            println!("{} lessons available.", set.len())
        }

        let selection = Select::new(
            "Select a lesson:",
            set.iter()
                .map(|&x| self.lesson_data(x).name.clone())
                .collect(),
        )
        .prompt()
        .ok()?;

        set.iter()
            .find(|&&x| self.lesson_data(x).name == selection)
            .copied()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub unlocked_lessons: BTreeSet<u16>,
    pub completed_lessons: BTreeSet<u16>,

    pub attempts: BTreeMap<u16, String>,
}

impl Save {
    pub fn load() -> Result<Save, std::io::Error> {
        if !fs::exists(SAVE_PATH)? {
            return Ok(Save::default());
        }

        let data = fs::read_to_string(SAVE_PATH)?;

        let save: Save = serde_json::from_str(&data).unwrap_or_else(|_err| {
            fs::write(RECOVERY_PATH, data).unwrap();
            eprintln!("There was an error recovering your save. A copy of the malformed save file was written to {}, and a new save was created.", RECOVERY_PATH);

            Save::default()
        });

        Ok(save)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        fs::write(SAVE_PATH, serde_json::to_string(&self).unwrap())
    }
}

impl Default for Save {
    fn default() -> Self {
        Save {
            unlocked_lessons: BTreeSet::from([0]),
            completed_lessons: BTreeSet::new(),

            attempts: BTreeMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct MainPrompt;

impl Autocomplete for MainPrompt {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, inquire::CustomUserError> {
        let mut matches: Vec<(String, i64)> = COMMANDS
            .into_iter()
            .filter_map(|command| {
                SkimMatcherV2::default()
                    .ignore_case()
                    .fuzzy_match(command, input)
                    .map(|score| (command.to_string(), score))
            })
            .collect();

        matches.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(matches.into_iter().map(|x| x.0).collect())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<inquire::autocompletion::Replacement, inquire::CustomUserError> {
        let first_suggestion = self
            .get_suggestions(input)
            .ok()
            .and_then(|x| x.first().cloned());

        Ok(highlighted_suggestion.or(first_suggestion))
    }
}

impl StringValidator for MainPrompt {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if COMMANDS.contains(&input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Expected a valid command".into()))
        }
    }
}
