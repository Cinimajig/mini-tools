use ::std::fmt;

#[derive(Default, Debug)]
pub struct WideString {
    inner: Vec<u16>,
}

impl WideString {
    pub fn from_str(text: &str) -> Self {
        use ::std::ffi::OsStr;
        use ::std::os::windows::ffi::OsStrExt;

        Self {
            inner: OsStr::new(text)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect(),
        }
    }

    pub fn ptr(&self) -> *const u16 {
        self.inner.as_ptr()
    }

    pub fn mut_ptr(&mut self) -> *mut u16 {
        self.inner.as_mut_ptr()
    }
}

impl fmt::Display for WideString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = String::from_utf16_lossy(&self.inner);
        write!(f, "{}", string)
    }
}
