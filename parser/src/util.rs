pub(crate) fn concat_nearby<'a, T>(prec: &'a [T], after: &'a [T]) -> Result<&'a [T], ()> {
    unsafe {
        let start = prec.as_ptr();
        // pointer arithmetic in units of T
        if start.add(prec.len()) == after.as_ptr() {
            Ok(std::slice::from_raw_parts(
                prec.as_ptr(),
                prec.len() + after.len(),
            ))
        } else {
            Err(())
        }
    }
}
