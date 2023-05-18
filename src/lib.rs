#[no_mangle]
extern "C" {
    pub fn kvpair_set(x: u64);
    pub fn kvpair_get() -> u64;
    pub fn kvpair_setroot(x: u64);
    pub fn kvpair_getroot() -> u64;
    pub fn kvpair_address(x: u64);
}

extern "C" {
    pub fn wasm_input(index: u32) -> i64;
}


const TREE_ID: u64 = 100;
const ADDRESS: u64 = 1234;
const KEY: u32 = 11111;

#[no_mangle]
pub fn zkmain() {
    // let count = unsafe { wasm_input(1) };
    // assert_eq!(3, count)

    unsafe {
        kvpair_setroot(TREE_ID);
        kvpair_address(ADDRESS);
        kvpair_set(123);

        let tree_id = kvpair_getroot();
        assert_eq!(TREE_ID, tree_id);
        kvpair_address(ADDRESS);
        assert_eq!(123, kvpair_get());
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
