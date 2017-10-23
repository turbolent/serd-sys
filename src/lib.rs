
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


extern crate libc;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};


    #[test]
    fn test_read() {

        extern "C" fn process(handle: *mut ::std::os::raw::c_void,
                              _flags: SerdStatementFlags,
                              _graph: *const SerdNode,
                              subject: *const SerdNode,
                              predicate: *const SerdNode,
                              object: *const SerdNode,
                              object_datatype: *const SerdNode,
                              object_lang: *const SerdNode) -> SerdStatus
        {
            assert!(!subject.is_null());
            assert!(!predicate.is_null());
            assert!(!object.is_null());
            assert!(object_datatype.is_null());
            assert!(object_lang.is_null());

            let s = unsafe { *subject };
            let p = unsafe { *predicate };
            let o = unsafe { *object };

            assert_eq!(s.type_, SerdType::SERD_URI);
            assert_eq!(p.type_, SerdType::SERD_URI);
            assert_eq!(o.type_, SerdType::SERD_URI);

            assert_eq!(unsafe { CStr::from_ptr(s.buf as *const _ )}.to_str().unwrap(), "urn:foo");
            assert_eq!(unsafe { CStr::from_ptr(p.buf as *const _ )}.to_str().unwrap(), "urn:bar");
            assert_eq!(unsafe { CStr::from_ptr(o.buf as *const _ )}.to_str().unwrap(), "urn:baz");

            let count: &mut i32 = unsafe { &mut *(handle as *mut i32) };
            *count += 1;

            SerdStatus::SERD_SUCCESS
        }

        unsafe {
            let path = "test.nt";
            let c_path = CString::new(path).unwrap();

            let mode = CString::new("rb").unwrap();
            let fd = libc::fopen(c_path.as_ptr(), mode.as_ptr());
            if fd.is_null() {
                panic!("Can't open test file: {}", path);
            }

            let mut count = 0;

            let reader = serd_reader_new(SerdSyntax::SERD_NTRIPLES,
                                         &mut count as *mut _ as *mut ::std::os::raw::c_void,
                                         None,
                                         None,
                                         None,
                                         Some(process),
                                         None);

            serd_reader_read_file_handle(reader, fd as *mut FILE,
                                         c_path.as_ptr() as *const _);

            assert_eq!(count, 1);

            libc::fclose(fd);
        }
    }
}
