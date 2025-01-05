pub struct Writers<'a> {
    pub stdout: &'a mut dyn std::io::Write,
    pub stderr: &'a mut dyn std::io::Write,
}

impl Writers<'_> {
    pub fn stdout(&mut self) -> &mut dyn std::io::Write {
        self.stdout
    }

    pub fn stderr(&mut self) -> &mut dyn std::io::Write {
        self.stderr
    }
}
