use crate::infra::language::lang_utils;

pub fn get_python_version() -> Option<String> {
    let python_version = run_get_python_version()?;
    let formatted_version = format_python_version(&python_version);

    Some(formatted_version)
}

fn run_get_python_version() -> Option<String> {
    let exec_python = lang_utils::exec_cmd("python", &["--version"]);
    match exec_python {
        Some(output) => {
            if output.stdout.is_empty() {
                Some(output.stderr)
            } else {
                Some(output.stdout)
            }
        }
        None => None,
    }
}

fn format_python_version(python_stdout: &str) -> String {
    format!(
        "v{}",
        python_stdout
            .trim_start_matches("Python ")
            .trim_end_matches(":: Anaconda, Inc.")
            .trim()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_with_output_stdout() {
        let _str = get_python_version();
    }
}