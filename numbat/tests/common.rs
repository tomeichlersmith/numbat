use std::path::Path;

use numbat::{module_importer::FileSystemImporter, resolver::CodeSource, Context};
use once_cell::sync::Lazy;

pub fn get_test_context_without_prelude() -> Context {
    let module_path = Path::new(
        &std::env::var_os("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR variable should be set when calling 'cargo test'"),
    )
    .join("modules");

    let mut importer = FileSystemImporter::default();
    importer.add_path(module_path);

    Context::new(importer)
}

pub fn get_test_context() -> Context {
    static CONTEXT: Lazy<Context> = Lazy::new(|| {
        let mut context = get_test_context_without_prelude();

        let _ = context
            .interpret("use prelude", CodeSource::Internal)
            .expect("Error while running prelude");
        context
    });

    CONTEXT.clone()
}
