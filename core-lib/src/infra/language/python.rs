use crate::infra::language::lang_utils;

pub fn get_python_version() -> Option<String> {
    let exec_python = lang_utils::exec_cmd("python", &["--version"]);
    match exec_python {
        Some(output) => {
            if output.stdout.is_empty() {
                Some(output.stdout)
            } else {
                let formatted_version = format_python_version(&output.stdout);
                Some(formatted_version)
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
        let str = get_python_version();
        println!(".... {}", str.unwrap())
    }
}