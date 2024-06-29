
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {

    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_store(kstack_ptr: usize) -> Self {
        extern "C" {
            fn _restore();
        }

        Self {
            ra: _restore  as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
