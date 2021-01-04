// NOTE: the actual Command API does not use owned Strings;
// this is a simplified version.

#[derive(Debug)]
pub struct Command {
    program: String,
    args: Vec<String>,
    cwd: Option<String>,
    // etc
}

impl Command {
    pub fn new(program: String) -> Command {
        Command {
            program: program,
            args: Vec::new(),
            cwd: None,
        }
    }

    /// Add an argument to pass to the program.
    pub fn arg(&mut self, arg: String) -> &mut Command {
        self.args.push(arg);
        self
    }

    /// Add multiple arguments to pass to the program.
    pub fn args(&mut self, args: &[String]) -> &mut Command {
        self.args.extend_from_slice(args);
        self
    }

    /// Set the working directory for the child process.
    pub fn current_dir(&mut self, dir: String) -> &mut Command {
        self.cwd = Some(dir);
        self
    }

    /// Executes the command as a child process, which is returned.
    pub fn spawn(&self) -> () {
        println!("{:?}", self);
        /* ... */
    }
}

fn main() {
    println!("Non-consuming builders");
    // One-liners
    Command::new("/bin/cat".to_string()).arg("file.txt".to_string()).spawn();

    // Complex configuration
    let mut cmd = Command::new("/bin/ls".to_string());
    cmd.arg(".".to_string());
    let size_sorted = true;
    if size_sorted {
        cmd.arg("-S".to_string());
    }
    cmd.spawn();
}
