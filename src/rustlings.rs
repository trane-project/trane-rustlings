use std::{collections::BTreeMap, vec};

use indoc::{formatdoc, indoc};
use trane::{
    course_builder::{AssetBuilder, CourseBuilder, ExerciseBuilder, LessonBuilder},
    data::{
        BasicAsset, CourseManifest, ExerciseAsset, ExerciseManifestBuilder, ExerciseType,
        LessonManifestBuilder,
    },
};

use crate::AUTHORS;

static COURSE_ID: &str = "trane::programming::rust::rustlings";

/// Represents a lesson containing rustlings exercises. A lesson can contain one or more exercises.
struct RustlingsLesson {
    /// A UID for the lesson.
    uid: usize,

    /// The dependencies of this lesson, also written as UIDs.
    dependencies: Vec<usize>,

    /// The name of the rustlings exercises contained by this lesson.
    exercises: Vec<String>,
}

impl RustlingsLesson {
    fn new(uid: usize, dependencies: &[usize], exercises: &[&str]) -> Self {
        Self {
            uid,
            dependencies: dependencies.to_vec(),
            exercises: exercises.iter().map(|&id| id.to_string()).collect(),
        }
    }

    /// Generates a LessonBuilder based on this object.
    fn lesson_builder(&self) -> LessonBuilder {
        let lesson_name = format!("Lesson {}", self.uid);
        let lesson_id = format!("{}::lesson_{}", COURSE_ID, self.uid);
        let dependencies: Vec<String> = self
            .dependencies
            .iter()
            .map(|id| format!("{}::lesson_{}", COURSE_ID, id))
            .collect();

        let mut exercises = vec![];
        for exercise in &self.exercises {
            let exercise_name = exercise.to_string();
            let exercise_id = format! {"{}::{}", lesson_id, exercise};

            exercises.push(ExerciseBuilder {
                directory_name: exercise_name.clone(),
                manifest_closure: Box::new(move |m| {
                    #[allow(clippy::redundant_clone)]
                    m.clone()
                        .id(exercise_id.clone())
                        .name(exercise_name.clone())
                        .clone()
                }),
                asset_builders: vec![
                    AssetBuilder {
                        file_name: "front.md".to_string(),
                        contents: formatdoc! {"
                            Solve Rustlings exercise {}.
                        ", exercise}
                        .to_string(),
                    },
                    AssetBuilder {
                        file_name: "back.md".to_string(),
                        contents: indoc! {"
                            Check your work via Rustlings and score this exercise accordingly.
                        "}
                        .to_string(),
                    },
                ],
            });
        }

        let lesson_id_clone = lesson_id.clone();
        LessonBuilder {
            directory_name: format!("lesson_{}", self.uid),
            asset_builders: vec![],
            manifest_closure: Box::new(move |m| {
                #[allow(clippy::redundant_clone)]
                m.clone()
                    .id(lesson_id_clone.clone())
                    .name(lesson_name.clone())
                    .dependencies(dependencies.clone())
                    .clone()
            }),
            exercise_manifest_template: ExerciseManifestBuilder::default()
                .lesson_id(lesson_id)
                .course_id(COURSE_ID.to_string())
                .exercise_type(ExerciseType::Procedural)
                .exercise_asset(ExerciseAsset::FlashcardAsset {
                    front_path: "front.md".to_string(),
                    back_path: "back.md".to_string(),
                })
                .clone(),
            exercise_builders: exercises,
        }
    }
}

struct RustlingsCourse {
    lessons: Vec<RustlingsLesson>,
}

impl RustlingsCourse {
    fn course_builder(&self) -> CourseBuilder {
        CourseBuilder {
            directory_name: "rustlings".to_string(),
            course_manifest: CourseManifest {
                id: COURSE_ID.to_string(),
                name: "Rustlings".to_string(),
                dependencies: vec![],
                description: Some("Learn Rust with Rustlings and Trane".to_string()),
                authors: Some(vec![AUTHORS.to_string()]),
                metadata: Some(BTreeMap::from([(
                    "programming_language".to_string(),
                    vec!["rust".to_string()],
                )])),
                course_instructions: Some(BasicAsset::MarkdownAsset {
                    path: "instructions.md".to_string(),
                }),
                course_material: None,
            },
            asset_builders: vec![AssetBuilder {
                file_name: "instructions.md".to_string(),
                contents: indoc! {"
                    This course contains exercises for learning Rust with Rustlings.
    
                    Perform the exercise indicated in the exercise by running the command below
                    in the rustlings directory in another terminal window.
                    
                    ```
                    rustlings run <EXERCISE_NAME>
                    ```

                    Undo your changes after completing the exercise to avoid seeing the answer
                    the next time you are asked to complete the same exercise.
                "}
                .to_string(),
            }],
            lesson_builders: self
                .lessons
                .iter()
                .map(|lesson| lesson.lesson_builder())
                .collect(),
            lesson_manifest_template: LessonManifestBuilder::default()
                .course_id(COURSE_ID.to_string())
                .clone(),
        }
    }
}

pub fn course_builder() -> CourseBuilder {
    let course = RustlingsCourse {
        lessons: vec![
            RustlingsLesson::new(
                1,
                &[],
                &["variables1", "variables2", "variables3", "variables4"],
            ),
            RustlingsLesson::new(2, &[1], &["variables5", "variables6"]),
            RustlingsLesson::new(
                3,
                &[1],
                &[
                    "functions1",
                    "functions2",
                    "functions3",
                    "functions4",
                    "functions5",
                ],
            ),
            RustlingsLesson::new(4, &[1], &["if1", "if2"]),
            RustlingsLesson::new(5, &[2, 3, 4], &["quiz1"]),
            RustlingsLesson::new(6, &[5], &["move_semantics1", "move_semantics2"]),
            RustlingsLesson::new(
                7,
                &[6],
                &[
                    "move_semantics3",
                    "move_semantics4",
                    "move_semantics5",
                    "move_semantics6",
                ],
            ),
            RustlingsLesson::new(
                8,
                &[5],
                &["primitive_types1", "primitive_types2", "primitive_types3"],
            ),
            RustlingsLesson::new(
                9,
                &[8],
                &["primitive_types4", "primitive_types5", "primitive_types6"],
            ),
            RustlingsLesson::new(10, &[5], &["structs1", "structs2", "structs3"]),
            RustlingsLesson::new(11, &[5], &["enums1", "enums2", "enums3"]),
            RustlingsLesson::new(12, &[5], &["modules1", "modules2", "modules3"]),
            RustlingsLesson::new(13, &[5], &["modules1", "modules2", "modules3"]),
            RustlingsLesson::new(14, &[5], &["vec1", "vec2", "hashmap1", "hashmap2"]),
            RustlingsLesson::new(15, &[5], &["strings1", "strings2"]),
            RustlingsLesson::new(16, &[7, 9, 10, 11, 12, 13, 14, 15], &["quiz2"]),
            RustlingsLesson::new(17, &[16], &["errors1", "errors2", "errors3", "errors4"]),
            RustlingsLesson::new(18, &[17], &["errors5", "errors6"]),
            RustlingsLesson::new(19, &[16], &["generics1", "generics2", "generics3"]),
            RustlingsLesson::new(20, &[16], &["options1", "options2", "options3"]),
            RustlingsLesson::new(21, &[16], &["traits1", "traits2"]),
            RustlingsLesson::new(22, &[16], &["tests1", "tests2"]),
            RustlingsLesson::new(23, &[18, 19, 20, 21, 22], &["quiz3"]),
            RustlingsLesson::new(24, &[23], &["box1", "arc1"]),
            RustlingsLesson::new(25, &[24], &["threads1"]),
            RustlingsLesson::new(
                26,
                &[23],
                &[
                    "iterators1",
                    "iterators2",
                    "iterators3",
                    "iterators4",
                    "iterators5",
                ],
            ),
            RustlingsLesson::new(27, &[23], &["macros1", "macros2", "macros3", "macros4"]),
            RustlingsLesson::new(28, &[25, 26, 27], &["quiz4"]),
            RustlingsLesson::new(29, &[28], &["clippy1", "clippy2"]),
            RustlingsLesson::new(
                30,
                &[28],
                &[
                    "using_as",
                    "from_into",
                    "from_str",
                    "try_from_into",
                    "as_ref_mut",
                ],
            ),
            RustlingsLesson::new(31, &[28], &["advanced_errs1", "advanced_errs2"]),
        ],
    };
    course.course_builder()
}
