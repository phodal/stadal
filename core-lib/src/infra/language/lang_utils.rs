use std::process::Command;

#[derive(Debug)]
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
}

impl PartialEq for CommandOutput {
    fn eq(&self, other: &Self) -> bool {
        self.stdout == other.stdout && self.stderr == other.stderr
    }
}

pub fn exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    internal_exec_cmd(&cmd, &args)
}

fn internal_exec_cmd(cmd: &str, args: &[&str]) -> Option<CommandOutput> {
    log::trace!("Executing command '{:?}' with args '{:?}'", cmd, args);
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            let stdout_string = String::from_utf8(output.stdout).unwrap();
            let stderr_string = String::from_utf8(output.stderr).unwrap();

            if !output.status.success() {
                log::trace!("Non-zero exit code '{:?}'", output.status.code());
                log::trace!("stdout: {}", stdout_string);
                log::trace!("stderr: {}", stderr_string);
                return None;
            }

            Some(CommandOutput {
                stdout: stdout_string,
                stderr: stderr_string,
            })
        }
        Err(_) => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_with_output_stdout() {
        let result = internal_exec_cmd("/bin/echo", &["-n", "hello"]);
        let expected = Some(CommandOutput {
            stdout: String::from("hello"),
            stderr: String::from(""),
        });

        assert_eq!(result, expected)
    }
}