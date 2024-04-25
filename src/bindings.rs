use lazy_static::lazy_static;
use std::ffi::{c_char, CString};

use libc::c_void;
use windows::{
    core::PCSTR,
    Win32::{self, Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
};

pub type PresentCallback = unsafe extern "C" fn(arg1: *mut c_void);

pub type KeyboardHandler =
    unsafe extern "C" fn(arg1: u32, arg2: u16, arg3: u8, arg4: i8, arg5: i8, arg6: i8, arg7: i8);

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(i32)]
pub enum GameVersion {
    VER_1_0_335_2_STEAM,
    VER_1_0_335_2_NOSTEAM,

    VER_1_0_350_1_STEAM,
    VER_1_0_350_2_NOSTEAM,

    VER_1_0_372_2_STEAM,
    VER_1_0_372_2_NOSTEAM,

    VER_1_0_393_2_STEAM,
    VER_1_0_393_2_NOSTEAM,

    VER_1_0_393_4_STEAM,
    VER_1_0_393_4_NOSTEAM,

    VER_1_0_463_1_STEAM,
    VER_1_0_463_1_NOSTEAM,

    VER_1_0_505_2_STEAM,
    VER_1_0_505_2_NOSTEAM,

    VER_1_0_573_1_STEAM,
    VER_1_0_573_1_NOSTEAM,

    VER_1_0_617_1_STEAM,
    VER_1_0_617_1_NOSTEAM,

    VER_SIZE,
    VER_UNK = -1,
}

#[allow(dead_code)]
pub unsafe fn create_texture(tex_file_name: *const c_char) -> i32 {
    (SHV.create_texture)(tex_file_name)
}

#[allow(dead_code)]
pub unsafe fn draw_texture(
    id: i32,
    index: i32,
    level: i32,
    time: i32,
    size_x: f32,
    size_y: f32,
    center_x: f32,
    center_y: f32,
    pos_x: f32,
    pos_y: f32,
    rotation: f32,
    screen_height_scale_factor: f32,
    r: f32,
    g: f32,
    b: f32,
    a: f32,
) {
    (SHV.draw_texture)(
        id,
        index,
        level,
        time,
        size_x,
        size_y,
        center_x,
        center_y,
        pos_x,
        pos_y,
        rotation,
        screen_height_scale_factor,
        r,
        g,
        b,
        a,
    )
}

#[allow(dead_code)]
pub unsafe fn present_callback_register(cb: PresentCallback) {
    (SHV.present_callback_register)(cb)
}

#[allow(dead_code)]
pub unsafe fn present_callback_unregister(cb: PresentCallback) {
    (SHV.present_callback_unregister)(cb)
}

#[allow(dead_code)]
pub unsafe fn keyboard_handler_register(handler: KeyboardHandler) {
    (SHV.keyboard_handler_register)(handler);
}

#[allow(dead_code)]
pub unsafe fn keyboard_handler_unregister(handler: KeyboardHandler) {
    (SHV.keyboard_handler_unregister)(handler);
}

#[allow(dead_code)]
pub unsafe fn script_wait(time: u32) {
    (SHV.script_wait)(time);
}

#[allow(dead_code)]
pub unsafe fn script_register(module: HMODULE, lp_script_main: extern "C" fn()) {
    (SHV.script_register)(module, lp_script_main);
}

#[allow(dead_code)]
pub unsafe fn script_register_additional_thread(module: HMODULE, lp_script_main: extern "C" fn()) {
    (SHV.script_register_additional_thread)(module, lp_script_main);
}

#[allow(dead_code)]
pub unsafe fn script_unregister(module: HMODULE) {
    (SHV.script_unregister)(module);
}

#[allow(dead_code)]
pub unsafe fn native_init(hash: u64) {
    (SHV.native_init)(hash);
}

#[allow(dead_code)]
pub unsafe fn native_push64(val: u64) {
    (SHV.native_push64)(val);
}

#[allow(dead_code)]
pub unsafe fn native_call() -> *mut u64 {
    (SHV.native_call)()
}

#[allow(dead_code)]
pub unsafe fn get_global_ptr(global_id: i32) -> *mut u64 {
    (SHV.get_global_ptr)(global_id)
}

#[allow(dead_code)]
pub unsafe fn world_get_all_vehicles(arr: *mut i32, arr_size: i32) -> i32 {
    (SHV.world_get_all_vehicles)(arr, arr_size)
}

#[allow(dead_code)]
pub unsafe fn world_get_all_peds(arr: *mut i32, arr_size: i32) -> i32 {
    (SHV.world_get_all_peds)(arr, arr_size)
}

#[allow(dead_code)]
pub unsafe fn world_get_all_objects(arr: *mut i32, arr_size: i32) -> i32 {
    (SHV.world_get_all_objects)(arr, arr_size)
}

#[allow(dead_code)]
pub unsafe fn world_get_all_pickups(arr: *mut i32, arr_size: i32) -> i32 {
    (SHV.world_get_all_pickups)(arr, arr_size)
}

#[allow(dead_code)]
pub unsafe fn get_script_handle_base_address(handle: i32) -> *mut u8 {
    (SHV.get_script_handle_base_address)(handle)
}

#[allow(dead_code)]
pub unsafe fn get_game_version() -> GameVersion {
    (SHV.get_game_version)()
}

pub struct ScriptHookVFunctions {
    pub create_texture: extern "C" fn(*const c_char) -> i32,
    pub draw_texture: extern "C" fn(
        i32,
        i32,
        i32,
        i32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
        f32,
    ),
    pub present_callback_register: extern "C" fn(PresentCallback),
    pub present_callback_unregister: extern "C" fn(PresentCallback),
    pub keyboard_handler_register: extern "C" fn(KeyboardHandler),
    pub keyboard_handler_unregister: extern "C" fn(KeyboardHandler),
    pub script_wait: extern "C" fn(u32),
    pub script_register: extern "C" fn(HMODULE, extern "C" fn()),
    pub script_register_additional_thread: extern "C" fn(HMODULE, extern "C" fn()),
    pub script_unregister: extern "C" fn(HMODULE),
    pub native_init: extern "C" fn(u64),
    pub native_push64: extern "C" fn(u64),
    pub native_call: extern "C" fn() -> *mut u64,
    pub get_global_ptr: extern "C" fn(i32) -> *mut u64,
    pub world_get_all_vehicles: extern "C" fn(*mut i32, i32) -> i32,
    pub world_get_all_peds: extern "C" fn(*mut i32, i32) -> i32,
    pub world_get_all_objects: extern "C" fn(*mut i32, i32) -> i32,
    pub world_get_all_pickups: extern "C" fn(*mut i32, i32) -> i32,
    pub get_script_handle_base_address: extern "C" fn(i32) -> *mut u8,
    pub get_game_version: extern "C" fn() -> GameVersion,
}

fn init() -> Option<ScriptHookVFunctions> {
    Some(ScriptHookVFunctions {
        create_texture: load_function("?createTexture@@YAHPEBD@Z")?,
        draw_texture: load_function("?drawTexture@@YAXHHHHMMMMMMMMMMMM@Z")?,
        present_callback_register: load_function("?presentCallbackRegister@@YAXP6AXPEAX@Z@Z")?,
        present_callback_unregister: load_function("?presentCallbackUnregister@@YAXP6AXPEAX@Z@Z")?,
        keyboard_handler_register: load_function("?keyboardHandlerRegister@@YAXP6AXKGEHHHH@Z@Z")?,
        keyboard_handler_unregister: load_function(
            "?keyboardHandlerUnregister@@YAXP6AXKGEHHHH@Z@Z",
        )?,
        script_wait: load_function("?scriptWait@@YAXK@Z")?,
        script_register: load_function("?scriptRegister@@YAXPEAUHINSTANCE__@@P6AXXZ@Z")?,
        script_register_additional_thread: load_function(
            "?scriptRegisterAdditionalThread@@YAXPEAUHINSTANCE__@@P6AXXZ@Z",
        )?,
        script_unregister: load_function("?scriptUnregister@@YAXPEAUHINSTANCE__@@@Z")?,
        native_init: load_function("?nativeInit@@YAX_K@Z")?,
        native_push64: load_function("?nativePush64@@YAX_K@Z")?,
        native_call: load_function("?nativeCall@@YAPEA_KXZ")?,
        get_global_ptr: load_function("?getGlobalPtr@@YAPEA_KH@Z")?,
        world_get_all_vehicles: load_function("?worldGetAllVehicles@@YAHPEAHH@Z")?,
        world_get_all_peds: load_function("?worldGetAllPeds@@YAHPEAHH@Z")?,
        world_get_all_objects: load_function("?worldGetAllObjects@@YAHPEAHH@Z")?,
        world_get_all_pickups: load_function("?worldGetAllPickups@@YAHPEAHH@Z")?,
        get_script_handle_base_address: load_function("?getScriptHandleBaseAddress@@YAPEAEH@Z")?,
        get_game_version: load_function("?getGameVersion@@YA?AW4eGameVersion@@XZ")?,
    })
}

lazy_static! {
    static ref SHV: ScriptHookVFunctions = init().unwrap();
}

pub fn load_function<T>(name: &str) -> Option<T> {
    let proc = unsafe { Win32::System::Threading::GetCurrentProcess() };
    let mut module_handles: [HMODULE; 1024] = [HMODULE::default(); 1024];
    let mut bytes_needed = 0;
    if unsafe {
        Win32::System::ProcessStatus::EnumProcessModules(
            proc,
            module_handles.as_mut_ptr(),
            std::mem::size_of::<HMODULE>() as u32 * module_handles.len() as u32,
            &mut bytes_needed,
        )
    }
    .is_err()
    {
        return None;
    };

    let num_modules = bytes_needed as usize / std::mem::size_of::<HMODULE>();
    let mod_str = CString::new(name).unwrap();
    for i in 0..num_modules {
        let proc =
            unsafe { GetProcAddress(module_handles[i], PCSTR(mod_str.as_ptr() as *const u8)) };
        if let Some(proc) = proc {
            return Some(unsafe { std::mem::transmute_copy(&proc) });
        }
    }
    None
}
