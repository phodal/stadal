use crate::infra::language::lang_utils;

pub fn get_golang_version() -> Option<String> {
    let output = lang_utils::exec_cmd("go", &["version"]).unwrap();
    let formatted_version = format_go_version(&output.stdout.as_str());
    formatted_version
}

fn format_go_version(go_stdout: &str) -> Option<String> {
    let version = go_stdout
        .splitn(2, "go version go")
        .nth(1)?
        .split_whitespace()
        .next()?;

    Some(format!("v{}", version))
}
