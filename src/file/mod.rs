#[allow(dead_code, unused_imports)]
pub mod project_file_generated;

pub use project_file_generated::*;

#[cfg(test)]
mod tests {

    use chrono::{DateTime, Utc};
    use std::array;

    use crate::file::ContentArgs;
    // import the generated code
    #[allow(dead_code, unused_imports)]
    #[allow(clippy::all)]
    use crate::file::project_file_generated::*;

    #[test]
    fn it_works() {
        let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

        // Category
        let set = builder.create_string("Set");
        let objs = builder.create_vector(&[0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let arrows = builder.create_vector(&[0_u64, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // Content
        let s = Content::create(
            &mut builder,
            &ContentArgs {
                category: Some(set),
                objects: Some(objs),
                arrows: Some(arrows),
            },
        );
        let monoid = builder.create_string("Monoid");
        let objs = builder.create_vector(&[0_u64]);
        let arrows = builder.create_vector(&[0_u64]);
        // Content
        let m = Content::create(
            &mut builder,
            &ContentArgs {
                category: Some(monoid),
                objects: Some(objs),
                arrows: Some(arrows),
            },
        );
        // Author
        let name = builder.create_string("Bob");
        let bob = Author::create(
            &mut builder,
            &AuthorArgs {
                id: 1,
                name: Some(name),
            },
        );
        // Operation
        let op = Operation::create(
            &mut builder,
            &OperationArgs {
                author: Some(bob),
                timestamp: Utc::now().timestamp(),
                content: Some(s),
            },
        );

        let operations = builder.create_vector(&[op]);

        let orc = ProjectFile::create(
            &mut builder,
            &ProjectFileArgs {
                version: Version::Zero,
                operations: Some(operations),
            },
        );

        builder.finish(orc, None);

        let buf = builder.finished_data();

        let project_file = flatbuffers::root::<ProjectFile>(buf).unwrap();

        let version = project_file.version();
        let ops = project_file.operations();

        dbg!(project_file);
    }
}
