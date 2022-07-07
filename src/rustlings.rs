use indoc::{formatdoc, indoc};
use trane::{
    course_builder::{AssetBuilder, ExerciseBuilder, LessonBuilder},
    data::{ExerciseAsset, ExerciseManifestBuilder, ExerciseType},
};

/// Represents a lesson containing rustlings exercises. A lesson can contain one or more exercises.
struct RustlingsLesson {
    /// A UID for the lesson.
    uid: usize,

    /// The dependencies of this lesson, also written as UIDs.
    dependencies: Vec<String>,

    /// The name of the rustlings exercises contained by this lesson.
    exercises: Vec<String>,
}

impl RustlingsLesson {
    fn new(uid: usize, dependencies: &[&str], exercises: &[&str]) -> Self {
        Self {
            uid,
            dependencies: dependencies.iter().map(|&id| id.to_string()).collect(),
            exercises: exercises.iter().map(|&id| id.to_string()).collect(),
        }
    }

    /// Generates a LessonBuilder based on this object.
    fn lesson_builder(&self, course_id: &str) -> LessonBuilder {
        let lesson_name = format!("Lesson {}", self.uid);
        let lesson_id = format!("{}::lesson_{}", course_id, self.uid);
        let dependencies: Vec<String> = self
            .dependencies
            .iter()
            .map(|id| format!("{}::{}", course_id, id))
            .collect();

        let mut exercises = vec![];
        for exercise in &self.exercises {
            let exercise_name = exercise.to_string();
            let exercise_id = format! {"{}::{}", lesson_id, exercise};

            exercises.push(ExerciseBuilder {
                directory_name: "exercise".to_string(),
                manifest_closure: Box::new(move |m| {
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
        let name_clone = lesson_name.clone();
        LessonBuilder {
            directory_name: format!("lesson_{}", self.uid),
            asset_builders: vec![],
            manifest_closure: Box::new(move |m| {
                m.clone()
                    .id(lesson_id_clone.clone())
                    .name(name_clone.clone())
                    .dependencies(dependencies.clone())
                    .clone()
            }),
            exercise_manifest_template: ExerciseManifestBuilder::default()
                .lesson_id(lesson_id.clone())
                .course_id(course_id.to_string())
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
