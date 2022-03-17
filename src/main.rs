fn main() {
    static mut STASH: &i32 = &10;
    static WORTH_POINT_AT: i32 = 1000;
    f(&WORTH_POINT_AT);

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
}
