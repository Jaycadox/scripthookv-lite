use crate::{native_call::IntoNativeArg, native_push64};

pub type Void = usize;
pub type Any = usize;
pub type Bool = u32;
pub type Hash = u32;
pub type Entity = i32;

impl IntoNativeArg for i32 {
    fn to_native_arg(&self) {
        let bytes_i32: [u8; 4] = self.to_le_bytes();
        let mut bytes_u64: [u8; 8] = [0; 8];
        bytes_u64[..4].copy_from_slice(&bytes_i32);
        let value_u64: u64 = u64::from_le_bytes(bytes_u64);
        unsafe { native_push64(value_u64) };
    }
}

impl IntoNativeArg for u32 {
    fn to_native_arg(&self) {
        let bytes_i32: [u8; 4] = self.to_le_bytes();
        let mut bytes_u64: [u8; 8] = [0; 8];
        bytes_u64[..4].copy_from_slice(&bytes_i32);
        let value_u64: u64 = u64::from_le_bytes(bytes_u64);
        unsafe { native_push64(value_u64) };
    }
}

impl IntoNativeArg for f32 {
    fn to_native_arg(&self) {
        let bytes_i32: [u8; 4] = self.to_le_bytes();
        let mut bytes_u64: [u8; 8] = [0; 8];
        bytes_u64[..4].copy_from_slice(&bytes_i32);
        let value_u64: u64 = u64::from_le_bytes(bytes_u64);
        unsafe { native_push64(value_u64) };
    }
}

impl IntoNativeArg for bool {
    fn to_native_arg(&self) {
        unsafe { native_push64(if *self { 1 } else { 0 }) };
    }
}

impl IntoNativeArg for usize {
    fn to_native_arg(&self) {
        let ptr = *self as usize;
        let ptr_64 = ptr as u64;
        unsafe { native_push64(ptr_64) };
    }
}

impl<T: Sized> IntoNativeArg for *mut T {
    fn to_native_arg(&self) {
        let ptr = *self as usize;
        let ptr_64 = ptr as u64;
        unsafe { native_push64(ptr_64) };
    }
}

impl<T: Sized> IntoNativeArg for *const T {
    fn to_native_arg(&self) {
        let ptr = *self as usize;
        let ptr_64 = ptr as u64;
        unsafe { native_push64(ptr_64) };
    }
}

pub type Player = i32;
pub type FireId = i32;
pub type Ped = i32;
pub type Vehicle = i32;
pub type Cam = i32;
#[allow(dead_code)]
pub type CarGenerator = i32;
#[allow(dead_code)]
pub type Group = i32;
#[allow(dead_code)]
pub type Train = i32;
pub type Pickup = i32;
pub type Object = i32;
#[allow(dead_code)]
pub type Weapon = i32;
pub type Interior = i32;
pub type Blip = i32;
#[allow(dead_code)]
pub type Texture = i32;
#[allow(dead_code)]
pub type TextureDict = i32;
#[allow(dead_code)]
pub type CoverPoint = i32;
#[allow(dead_code)]
pub type Camera = i32;
#[allow(dead_code)]
pub type TaskSequence = i32;
#[allow(dead_code)]
pub type ColourIndex = i32;
#[allow(dead_code)]
pub type Sphere = i32;
pub type ScrHandle = i32;

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector4 {
    pub x: f32,
    _pad0x04: u32,
    pub y: f32,
    _pad0x08: u32,
    pub z: f32,
    _pad0x10: u32,
    pub w: f32,
    _pad0x18: u32,
}

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f32,
    _pad0x04: u32,
    pub y: f32,
    _pad0x08: u32,
    pub z: f32,
    _pad0x10: u32,
}

#[repr(C, align(1))]
#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f32,
    _pad0x04: u32,
    pub y: f32,
    _pad0x08: u32,
}
