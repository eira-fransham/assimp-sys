#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

extern crate libz_sys;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for aiVector3D {
    fn default() -> Self {
        Self {
            x: 0.0, y: 0.0, z: 0.0
        }
    }
}

impl Default for aiString {
    fn default() -> Self {
        aiString {
            length: 0,
            data: [0; MAXLEN as usize],
        }
    }
}

impl AsRef<str> for aiString {
    fn as_ref(&self) -> &str {
        let len = self.length as usize;
        unsafe {
            let bytes = std::slice::from_raw_parts(self.data.as_ptr() as *const u8, len);
            std::str::from_utf8_unchecked(bytes)
        }
    }
}

impl From<&str> for aiString {
    fn from(s: &str) -> Self {
        let mut aistr = Self::default();
        let len = s.len();
        let bytes = unsafe { std::slice::from_raw_parts(s.as_bytes().as_ptr() as *const i8, len) };
        aistr.data[0..len].copy_from_slice(bytes);
        aistr.length = len as u32;
        aistr
    }
}

impl std::fmt::Debug for aiString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s: &str = self.as_ref();
        write!(f, "{:?}", s)
    }
}
impl PartialEq for aiString {
    fn eq(&self, other: &aiString) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for aiString {}


pub const aiPostProcessSteps_EMPTY: aiPostProcessSteps = 0;
