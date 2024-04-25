use crate::bindings;

pub type NativeHash = u64;

/// Start a native call
#[inline]
pub fn native_init(hash: NativeHash) {
    unsafe { bindings::native_init(hash) }
}

pub trait IntoNativeArg {
    fn to_native_arg(&self);
}

#[inline]
pub unsafe fn native_push<T: IntoNativeArg>(value: &T) {
    value.to_native_arg()
}

#[inline]
pub unsafe fn native_end_call<T: Copy>() -> T {
    let result_pointer = bindings::native_call();
    *(result_pointer as *mut T)
}

#[macro_export]
macro_rules! call_native {
  ($type:ty, $hash:literal $(, $args:expr)*) => {{
    native_init($hash);
    $(
      native_push(&$args);
    )*
    native_end_call::<$type>()
  }}
}
