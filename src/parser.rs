use std::collections::BTreeMap;

use markdown::{
    mdast::{Code, Node},
    ParseOptions,
};
use rust_embed::Embed;

use crate::lesson::Lesson;

#[derive(Embed)]
#[folder = "lessons/"]
pub struct LessonData;

pub fn parse_lessons() -> BTreeMap<u16, (Node, Lesson)> {
    let mut data = BTreeMap::new();

    for file_path in LessonData::iter() {
        // Every lesson should be valid UTF8 & markdown- I don't feel so bad making the runtime errors worse than useless.
        let raw_data = LessonData::get(&file_path).unwrap().data;
        let lesson_data = String::from_utf8(raw_data.to_vec()).expect("UTF-8 parsing error");
        let ast =
            markdown::to_mdast(&lesson_data, &ParseOptions::gfm()).expect("Markdown parsing error");

        // extract the yaml metadata (usually at the end)
        let metadata = ast.children().unwrap().iter().find_map(|node| match &node {
            Node::Code(Code {
                lang: Some(lang),
                value,
                ..
            }) if lang == "json" => Some(value),
            _ => None,
        });

        if metadata.is_none() {
            eprintln!("Missing metadata for {}", file_path);
            continue;
        }

        let lesson: Result<Lesson, serde_json::Error> = serde_json::from_str(metadata.unwrap());
        match lesson {
            Ok(lesson) => {
                data.insert(lesson.id, (ast, lesson));
            }
            Err(err) => panic!("Error in lesson {}:\n{}", file_path, err),
        }
    }

    data
}
