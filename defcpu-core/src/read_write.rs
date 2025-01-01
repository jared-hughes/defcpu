pub(crate) struct Writers<'a> {
    pub(crate) stdout: &'a mut dyn std::io::Write,
    pub(crate) stderr: &'a mut dyn std::io::Write,
}

impl Writers<'_> {
    pub(crate) fn stdout(&mut self) -> &mut dyn std::io::Write {
        self.stdout
    }

    pub(crate) fn stderr(&mut self) -> &mut dyn std::io::Write {
        self.stderr
    }
}
