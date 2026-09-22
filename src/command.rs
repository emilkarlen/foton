pub trait ExecutableCmd {
    fn execute(&self) -> std::process::ExitCode;
}
