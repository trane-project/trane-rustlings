use anyhow::Result;
use trane::course_builder::CourseBuilder;

static AUTHORS: &str = "The Trane Project";

fn main() -> Result<()> {
    let curr_dir = std::env::current_dir()?;

    let course_builders: Vec<CourseBuilder> = vec![

    ];

    for course_builder in course_builders {
        course_builder.build(&curr_dir)?;
        println!("Built {} course", course_builder.course_manifest.name);
    }
    Ok(())
}
