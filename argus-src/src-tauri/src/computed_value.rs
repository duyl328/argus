use std::sync::Once;

pub struct ComputedValue<T> {
    value: Option<T>,
    computed: Once,
}

impl<T> ComputedValue<T> {
    pub(crate) fn new() -> Self {
        ComputedValue {
            value: None,
            computed: Once::new(),
        }
    }

    // 通过闭包计算并存储值，只会计算一次
    pub(crate) fn get_or_compute<F>(&mut self, compute_fn: F) -> &T
    where
        F: FnOnce() -> T,
    {
        self.computed.call_once(|| {
            self.value = Some(compute_fn());
        });

        self.value.as_ref().unwrap() // 返回计算后的值
    }
}
