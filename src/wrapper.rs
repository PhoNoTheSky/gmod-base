use super::bindings::*;
use std::os::raw::{c_void, c_char};
use std::ffi::{CString, CStr};
use std::str::Utf8Error;

pub struct LuaWrapper {
    pub state: *mut lua_State
}

impl LuaWrapper {
    pub unsafe fn at_panic(&self, func: lua_CFunction) -> lua_CFunction {
        lua_atpanic(self.state, func)
    }

    pub unsafe fn call(&self, args: i32, results: i32) {
        lua_call(self.state, args, results)
    }

    pub unsafe fn check_stack(&self, extra: i32) -> i32 {
        lua_checkstack(self.state, extra)
    }

    pub unsafe fn close(self) {
        lua_close(self.state);
        drop(self)
    }

    pub unsafe fn concat(&self, n: i32) {
        lua_concat(self.state, n)
    }

    pub unsafe fn cpcall(&self, func: lua_CFunction, ud: *mut c_void) -> i32 {
        lua_cpcall(self.state, func, ud)
    }

    pub unsafe fn create_table(&self, arr: i32, rec: i32) {
        lua_createtable(self.state, arr, rec)
    }

    pub unsafe fn dump(&self, writer: lua_Writer, data: *mut c_void) -> i32 {
        lua_dump(self.state, writer, data)
    }

    pub unsafe fn equal(&self, first_index: i32, second_index: i32) -> bool {
        lua_equal(self.state, first_index, second_index) == 1
    }

    pub unsafe fn error(&self) -> i32 {
        lua_error(self.state)
    }

    pub unsafe fn gc(&self, what: i32, data: i32) -> i32 {
        lua_gc(self.state, what, data)
    }

    pub unsafe fn get_allocf(&self, ud: *mut *mut c_void) -> lua_Alloc {
        lua_getallocf(self.state, ud)
    }

    pub unsafe fn get_fenv(&self, index: i32) {
        lua_getfenv(self.state, index)
    }

    pub unsafe fn get_field<T>(&self, index: i32, k: T)
        where T: ToString {
        let k = k.to_string();
        let k = CString::new(k).unwrap();
        lua_getfield(self.state, index, k.as_ptr())
    }

    pub unsafe fn get_global<T>(&self, name: T)
        where T: ToString {
        self.get_field(LUA_GLOBALSINDEX, name)
    }

    pub unsafe fn get_metatable(&self, index: i32) -> i32 {
        lua_getmetatable(self.state, index)
    }

    pub unsafe fn get_table(&self, index: i32) {
        lua_gettable(self.state, index)
    }

    pub unsafe fn get_top(&self) -> i32 {
        lua_gettop(self.state)
    }

    pub unsafe fn insert(&self, index: i32) {
        lua_insert(self.state, index)
    }

    pub unsafe fn is_boolean(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TBOOLEAN as i32
    }

    pub unsafe fn is_cfunction(&self, index: i32) -> bool {
        lua_iscfunction(self.state, index) == 1
    }

    pub unsafe fn is_function(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TFUNCTION as i32
    }

    pub unsafe fn is_light_userdata(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TLIGHTUSERDATA as i32
    }

    pub unsafe fn is_nil(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TNIL as i32
    }

    pub unsafe fn is_none(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TNONE as i32
    }

    pub unsafe fn is_none_or_nil(&self, index: i32) -> bool {
        self.is_none(index) || self.is_nil(index)
    }

    pub unsafe fn is_number(&self, index: i32) -> bool {
        lua_isnumber(self.state, index) == 1
    }

    pub unsafe fn is_string(&self, index: i32) -> bool {
        lua_isstring(self.state, index) == 1
    }

    pub unsafe fn is_table(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TTABLE as i32
    }

    pub unsafe fn is_thread(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TTHREAD as i32
    }

    pub unsafe fn is_userdata(&self, index: i32) -> bool {
        lua_type(self.state, index) == LUA_TUSERDATA as i32
    }

    pub unsafe fn less_than(&self, first_index: i32, second_index: i32) -> bool {
        lua_lessthan(self.state, first_index, second_index) == 1
    }

    pub unsafe fn load<T>(&self, reader: lua_Reader, data: *mut c_void, chunk_name: T) -> i32
        where T: ToString {
        let chunk_name = chunk_name.to_string();
        let chunk_name = CString::new(chunk_name).unwrap();
        lua_load(self.state, reader, data, chunk_name.as_ptr())
    }

    pub unsafe fn new_state(f: lua_Alloc, ud: *mut c_void) -> LuaWrapper {
        LuaWrapper {
            state: lua_newstate(f, ud),
        }
    }

    pub unsafe fn new_table(&self) {
        self.create_table(0, 0)
    }

    pub unsafe fn new_thread(&self) -> LuaWrapper {
        LuaWrapper {
            state: lua_newthread(self.state)
        }
    }

    pub unsafe fn new_userdata(&self, size: size_t) -> *mut c_void {
        lua_newuserdata(self.state, size)
    }

    pub unsafe fn next(&self, index: i32) -> i32 {
        lua_next(self.state, index)
    }

    pub unsafe fn obj_len(&self, index: i32) -> size_t {
        lua_objlen(self.state, index)
    }

    pub unsafe fn pcall(&self, args: i32, results: i32, err_func: i32) -> i32 {
        lua_pcall(self.state, args, results, err_func)
    }

    pub unsafe fn pop(&self, index: i32) {
        self.set_top(-index - 1)
    }

    pub unsafe fn push_boolean(&self, value: bool) {
        lua_pushboolean(self.state, value as _)
    }

    pub unsafe fn push_cclosure(&self, func: lua_CFunction, n: i32) {
        lua_pushcclosure(self.state, func, n)
    }

    pub unsafe fn push_cfunction(&self, func: lua_CFunction) {
        self.push_cclosure(func, 0)
    }

    // TODO: implement LuaWrapper::push_fstring
    pub unsafe fn push_fstring<T>(&self, _fmt: T /*, ...*/) -> *const c_char
        where T: ToString {
        unimplemented!()
    }

    pub unsafe fn push_integer(&self, n: lua_Integer) {
        lua_pushinteger(self.state, n)
    }

    pub unsafe fn push_light_userdata(&self, p: *mut c_void) {
        lua_pushlightuserdata(self.state, p)
    }

    pub unsafe fn push_literal<T>(&self, s: T)
        where T: ToString {
        let s = s.to_string();
        let len = s.len();
        self.push_lstring(s, len as size_t)
    }

    pub unsafe fn push_lstring<T>(&self, s: T, len: size_t)
        where T: ToString {
        let s = s.to_string();
        let s = CString::new(s).unwrap();
        lua_pushlstring(self.state, s.as_ptr(), len)
    }

    pub unsafe fn push_nil(&self) {
        lua_pushnil(self.state)
    }

    pub unsafe fn push_number(&self, n: lua_Number) {
        lua_pushnumber(self.state, n)
    }

    pub unsafe fn push_string<T>(&self, s: T)
        where T: ToString {
        let s = s.to_string();
        let s = CString::new(s).unwrap();
        lua_pushstring(self.state, s.as_ptr())
    }

    pub unsafe fn push_thread(&self) -> i32 {
        lua_pushthread(self.state)
    }

    pub unsafe fn push_value(&self, index: i32) {
        lua_pushvalue(self.state, index)
    }

    pub unsafe fn push_vfstring<T>(&self, fmt: T, args: va_list) -> *const c_char
        where T: ToString {
        let fmt = fmt.to_string();
        let fmt = CString::new(fmt).unwrap();
        lua_pushvfstring(self.state, fmt.as_ptr(), args)
    }

    pub unsafe fn raw_equal(&self, first_index: i32, second_index: i32) -> bool {
        lua_rawequal(self.state, first_index, second_index) == 1
    }

    pub unsafe fn raw_get(&self, index: i32) {
        lua_rawget(self.state, index)
    }

    pub unsafe fn raw_geti(&self, index: i32, n: i32) {
        lua_rawgeti(self.state, index, n)
    }

    pub unsafe fn raw_set(&self, index: i32) {
        lua_rawset(self.state, index)
    }

    pub unsafe fn raw_seti(&self, index: i32, n: i32) {
        lua_rawseti(self.state, index, n)
    }

    pub unsafe fn register<T>(&self, name: T, func: lua_CFunction)
        where T: ToString {
        self.push_cfunction(func);
        self.set_global(name)
    }

    pub unsafe fn remove(&self, index: i32) {
        lua_remove(self.state, index)
    }

    pub unsafe fn replace(&self, index: i32) {
        lua_replace(self.state, index)
    }

    // TODO: fix lua_resume not linking
    // pub unsafe fn resume(&self, args: i32) -> i32 {
    //     lua_resume(self.state, args)
    // }

    pub unsafe fn set_allocf(&self, f: lua_Alloc, ud: *mut c_void) {
        lua_setallocf(self.state, f, ud)
    }

    pub unsafe fn set_fenv(&self, index: i32) -> i32 {
        lua_setfenv(self.state, index)
    }

    pub unsafe fn set_field<T>(&self, index: i32, k: T)
        where T: ToString {
        let k = k.to_string();
        let k = CString::new(k).unwrap();
        lua_setfield(self.state, index, k.as_ptr())
    }

    pub unsafe fn set_global<T>(&self, name: T)
        where T: ToString {
        self.set_field(LUA_GLOBALSINDEX, name)
    }

    pub unsafe fn set_metatable(&self, index: i32) -> i32 {
        lua_setmetatable(self.state, index)
    }

    pub unsafe fn set_table(&self, index: i32) {
        lua_settable(self.state, index)
    }

    pub unsafe fn set_top(&self, index: i32) {
        lua_settop(self.state, index)
    }

    pub unsafe fn status(&self) -> i32 {
        lua_status(self.state)
    }

    pub unsafe fn to_boolean(&self, index: i32) -> i32 {
        lua_toboolean(self.state, index)
    }

    pub unsafe fn to_cfunction(&self, index: i32) -> lua_CFunction {
        lua_tocfunction(self.state, index)
    }

    pub unsafe fn to_integer(&self, index: i32) -> lua_Integer {
        lua_tointeger(self.state, index)
    }

    pub unsafe fn to_lstring(&self, index: i32, len: *mut size_t) -> Result<&str, Utf8Error> {
        let s = lua_tolstring(self.state, index, len);
        CStr::from_ptr(s).to_str()
    }

    pub unsafe fn to_number(&self, index: i32) -> lua_Number {
        lua_tonumber(self.state, index)
    }

    pub unsafe fn to_pointer(&self, index: i32) -> *const c_void {
        lua_topointer(self.state, index)
    }

    pub unsafe fn to_string(&self, index: i32) -> Result<&str, Utf8Error> {
        self.to_lstring(index, std::ptr::null_mut())
    }

    pub unsafe fn to_thread(&self, index: i32) -> LuaWrapper {
        LuaWrapper {
            state: lua_tothread(self.state, index)
        }
    }

    pub unsafe fn to_userdata(&self, index: i32) -> *mut c_void {
        lua_touserdata(self.state, index)
    }

    pub unsafe fn r#type(&self, index: i32) -> i32 {
        lua_type(self.state, index)
    }

    pub unsafe fn type_name(&self, tp: i32) -> Result<&str, Utf8Error> {
        let s = lua_typename(self.state, tp);
        CStr::from_ptr(s).to_str()
    }

    pub unsafe fn xmove(&self, to: *mut lua_State, n: i32) {
        lua_xmove(self.state, to, n)
    }

    pub unsafe fn copy(&self, from_index: i32, to_index: i32) {
        lua_copy(self.state, from_index, to_index)
    }

    pub unsafe fn r#yield(&self, n: i32) -> i32 {
        lua_yield(self.state, n)
    }

    pub unsafe fn is_yieldable(&self) -> bool {
        lua_isyieldable(self.state) == 1
    }

    pub unsafe fn get_hook(&self) -> lua_Hook {
        lua_gethook(self.state)
    }

    pub unsafe fn get_hook_count(&self) -> i32 {
        lua_gethookcount(self.state)
    }

    pub unsafe fn get_hook_mask(&self) -> i32 {
        lua_gethookmask(self.state)
    }

    pub unsafe fn get_info<T>(&self, what: T, ar: *mut lua_Debug) -> i32
        where T: ToString {
        let what = what.to_string();
        let what = CString::new(what).unwrap();
        lua_getinfo(self.state, what.as_ptr(), ar)
    }

    pub unsafe fn get_stack(&self, level: i32, ar: *mut lua_Debug) -> i32 {
        lua_getstack(self.state, level, ar)
    }

    pub unsafe fn get_up_value(&self, func_index: i32, n: i32) -> Result<&str, Utf8Error> {
        let string = lua_getupvalue(self.state, func_index, n);
        CStr::from_ptr(string).to_str()
    }

    pub unsafe fn set_up_value(&self, func_index: i32, n: i32) -> Result<&str, Utf8Error> {
        let string = lua_setupvalue(self.state, func_index, n);
        CStr::from_ptr(string).to_str()
    }

    pub unsafe fn set_hook(&self, f: lua_Hook, mask: i32, count: i32) -> i32 {
        lua_sethook(self.state, f, mask, count)
    }

    pub unsafe fn get_local(&self, ar: *mut lua_Debug, n: i32) -> Result<&str, Utf8Error> {
        let string = lua_getlocal(self.state, ar, n);
        CStr::from_ptr(string).to_str()
    }

    pub unsafe fn set_local(&self, ar: *mut lua_Debug, n: i32) -> Result<&str, Utf8Error> {
        let string = lua_setlocal(self.state, ar, n);
        CStr::from_ptr(string).to_str()
    }

    pub unsafe fn up_valid_id(&self, fidx: i32, n: i32) -> *mut c_void {
        lua_upvalueid(self.state, fidx, n)
    }

    pub unsafe fn up_value_join(&self, first_fidx: i32, first_n: i32,
                                second_fidx: i32, second_n: i32) {
        lua_upvaluejoin(self.state, first_fidx, first_n, second_fidx, second_n)
    }

    pub fn get_auxiliary_wrapper(&self) -> LuaAuxiliaryWrapper {
        LuaAuxiliaryWrapper {
            state: self.state
        }
    }
}

pub struct LuaAuxiliaryWrapper {
    pub state: *mut lua_State,
}

impl LuaAuxiliaryWrapper {
    pub fn get_standard_wrapper(&self) -> LuaWrapper {
        LuaWrapper {
            state: self.state,
        }
    }

    pub unsafe fn new_state(&self) -> LuaAuxiliaryWrapper {
        let state = luaL_newstate();
        luaL_openlibs(state);
        LuaAuxiliaryWrapper {
            state,
        }
    }
    pub unsafe fn buff_init(&self) -> LuaBuffer {
        let buffer = LuaBuffer {
            buffer: std::mem::zeroed()
        };
        luaL_buffinit(self.state, buffer.buffer);
        buffer
    }

    pub unsafe fn open_lib<T>(&self, lib: T, reg: *mut luaL_Reg, nup: i32)
        where T: ToString {
        let lib = lib.to_string();
        let lib = CString::new(lib).unwrap();
        luaL_openlib(self.state, lib.as_ptr(), reg, nup)
    }

    pub unsafe fn register<T>(&self, lib: T, reg: *mut luaL_Reg)
        where T: ToString {
        let lib = lib.to_string();
        let lib = CString::new(lib).unwrap();
        luaL_register(self.state, lib.as_ptr(), reg)
    }

    // TODO: finish rest of luaL_* functions
}

pub struct LuaBuffer {
    pub buffer: *mut luaL_Buffer,
}

impl LuaBuffer {
    pub unsafe fn prep_buffer(&self) -> *mut i8 {
        luaL_prepbuffer(self.buffer)
    }

    pub unsafe fn push_result(&self) {
        luaL_pushresult(self.buffer)
    }

    pub unsafe fn add_lstring<T>(&self, s: T)
        where T: ToString {
        let s = s.to_string();
        let s = CString::new(s).unwrap();
        let len = s.as_bytes().len();
        luaL_addlstring(self.buffer, s.as_ptr(), len as size_t);
    }

    pub unsafe fn add_string<T>(&self, s: T)
        where T: ToString {
        let s = s.to_string();
        let s = CString::new(s).unwrap();
        luaL_addstring(self.buffer, s.as_ptr())
    }

    pub unsafe fn add_value(&self) {
        luaL_addvalue(self.buffer)
    }

    // TODO: finished missing buffer functions
}
