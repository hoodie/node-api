use std;
use std::boxed::Box;
use std::ffi::{CStr, CString, NulError};

use node_api_sys::*;

use napi_args::FromNapiArgs;

pub type NapiValue = napi_value;
pub type NapiEnv = napi_env;
pub type Result<T> = std::result::Result<T, NapiError>;

#[derive(Debug, Clone)]
pub struct NapiModule {
    pub version: i32,
    pub flags: u32,
    pub filename: String,
    pub register_func: napi_addon_register_func,
    pub modname: String,
}

#[derive(Debug, Clone)]
pub struct NapiError {
    pub error_message: String,
    pub engine_error_code: u32,
    pub error_code: NapiErrorType,
}

impl From<napi_extended_error_info> for NapiError {
    fn from(error: napi_extended_error_info) -> Self {
        unsafe {
            Self {
                error_message: CStr::from_ptr(error.error_message)
                    .to_string_lossy()
                    .into_owned(),
                engine_error_code: error.engine_error_code,
                error_code: NapiErrorType::from(error.error_code),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NapiErrorType {
    InvalidArg,
    ObjectExpected,
    StringExpected,
    NameExpected,
    FunctionExpected,
    NumberExpected,
    BooleanExpected,
    ArrayExpected,
    GenericFailure,
    PendingException,
    Cancelled,
    StatusLast,
}

impl From<napi_status> for NapiErrorType {
    fn from(s: napi_status) -> Self {
        match s {
            napi_status::napi_invalid_arg => NapiErrorType::InvalidArg,
            napi_status::napi_object_expected => NapiErrorType::ObjectExpected,
            napi_status::napi_string_expected => NapiErrorType::StringExpected,
            napi_status::napi_name_expected => NapiErrorType::NameExpected,
            napi_status::napi_function_expected => NapiErrorType::FunctionExpected,
            napi_status::napi_number_expected => NapiErrorType::NumberExpected,
            napi_status::napi_boolean_expected => NapiErrorType::BooleanExpected,
            napi_status::napi_array_expected => NapiErrorType::ArrayExpected,
            napi_status::napi_generic_failure => NapiErrorType::GenericFailure,
            napi_status::napi_pending_exception => NapiErrorType::PendingException,
            napi_status::napi_cancelled => NapiErrorType::Cancelled,
            napi_status::napi_status_last => NapiErrorType::StatusLast,
            _ => NapiErrorType::GenericFailure,
        }
    }
}

fn napi_either<T>(env: NapiEnv, status: napi_status, val: T) -> Result<T> {
    match status {
        napi_status::napi_ok => Ok(val),
        err => Err(get_last_napi_error(env).expect("error fetching last napi error")),
    }
}

fn get_last_error_info(env: napi_env)
                       -> std::result::Result<napi_extended_error_info, napi_status> {
    unsafe {
        let mut info: *const napi_extended_error_info =
            Box::into_raw(Box::new(std::mem::uninitialized()));
        let status = napi_get_last_error_info(env, &mut info);
        match status {
            napi_status::napi_ok => Ok(*info),
            _ => Err(status),
        }
    }
}

fn get_last_napi_error(env: NapiEnv) -> std::result::Result<NapiError, NapiErrorType> {
    get_last_error_info(env)
        .map(|res| NapiError::from(res))
        .map_err(|err| NapiErrorType::from(err))
}

pub fn module_register(mod_: NapiModule) -> std::result::Result<(), NulError> {
    let module = &mut napi_module {
                          nm_version: mod_.version,
                          nm_flags: mod_.flags,
                          nm_filename: CString::new(mod_.filename)?.as_ptr(),
                          nm_register_func: Some(mod_.register_func.unwrap()),
                          nm_modname: try!(CString::new(mod_.modname)).as_ptr(),
                          nm_priv: std::ptr::null_mut(),
                          reserved: [std::ptr::null_mut(),
                                     std::ptr::null_mut(),
                                     std::ptr::null_mut(),
                                     std::ptr::null_mut()],
                      };
    unsafe {
        napi_module_register(module);
    }
    Ok(())
}

pub fn get_undefined(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_get_undefined(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_null(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_get_null(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_global(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_get_global(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn get_boolean(env: NapiEnv, value: bool) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_get_boolean(env, value, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_object(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_create_object(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_array(env: NapiEnv) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_create_array(env, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn array_with_length(env: NapiEnv, size: usize) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_create_array_with_length(env, size, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}

pub fn create_number(env: NapiEnv, value: f64) -> Result<NapiValue> {
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_create_number(env, value, &mut napi_val);
        napi_either(env, status, napi_val)
    }
}


//     pub fn napi_create_number(env: napi_env, value: f64,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_create_string_latin1(env: napi_env,
//                                      str: *const ::std::os::raw::c_char,
//                                      length: usize, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_string_utf8(env: napi_env,
//                                    str: *const ::std::os::raw::c_char,
//                                    length: usize, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_string_utf16(env: napi_env, str: *const char16_t,
//                                     length: usize, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_symbol(env: napi_env, description: napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_create_function(env: napi_env,
//                                 utf8name: *const ::std::os::raw::c_char,
//                                 cb: napi_callback,
//                                 data: *mut ::std::os::raw::c_void,
//                                 result: *mut napi_value) -> napi_status;

pub fn create_function<F, T: FromNapiArgs>(env: NapiEnv, utf8name: &str, f: F) -> Result<NapiValue>
    where F: Fn(NapiEnv, T),
          T: FromNapiArgs
{
    let user_data = &f as *const _ as *mut std::os::raw::c_void;
    unsafe extern "C" fn wrapper<F, T>(env: NapiEnv, cbinfo: napi_callback_info) -> NapiValue
        where F: Fn(NapiEnv, T),
              T: FromNapiArgs
    {
        let mut argc: usize = 16;
        let mut argv: [NapiValue; 16] = std::mem::uninitialized();
        let mut callback: Option<F> = None;

        napi_get_cb_info(env,
                         cbinfo,
                         &mut argc,
                         argv.as_mut_ptr(),
                         std::ptr::null_mut(),
                         &mut std::mem::transmute::<&mut Option<F>,
                                                    *mut ::std::os::raw::c_void>(&mut callback));

        let args = T::from_napi_args(&argv[0..argc]).unwrap();
        match callback {
            Some(cb) => cb(env, args),
            None => ()
        }
        get_undefined(env).unwrap()
    }
    unsafe {
        let mut napi_val: NapiValue = std::mem::uninitialized();
        let status = napi_create_function(env,
                                          CString::new(utf8name).unwrap().into_raw(),
                                          Some(wrapper::<F, T>),
                                          user_data,
                                          &mut napi_val);
        napi_either(env, status, napi_val)
    }
}


//     pub fn napi_create_error(env: napi_env, msg: napi_value,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_create_type_error(env: napi_env, msg: napi_value,
//                                   result: *mut napi_value) -> napi_status;


//     pub fn napi_create_range_error(env: napi_env, msg: napi_value,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_typeof(env: napi_env, value: napi_value,
//                        result: *mut napi_valuetype) -> napi_status;


//     pub fn napi_get_value_double(env: napi_env, value: napi_value,
//                                  result: *mut f64) -> napi_status;


//     pub fn napi_get_value_int32(env: napi_env, value: napi_value,
//                                 result: *mut i32) -> napi_status;


//     pub fn napi_get_value_uint32(env: napi_env, value: napi_value,
//                                  result: *mut u32) -> napi_status;


//     pub fn napi_get_value_int64(env: napi_env, value: napi_value,
//                                 result: *mut i64) -> napi_status;


//     pub fn napi_get_value_bool(env: napi_env, value: napi_value,
//                                result: *mut bool) -> napi_status;


//     pub fn napi_get_value_string_latin1(env: napi_env, value: napi_value,
//                                         buf: *mut ::std::os::raw::c_char,
//                                         bufsize: usize, result: *mut usize)
//      -> napi_status;


//     pub fn napi_get_value_string_utf8(env: napi_env, value: napi_value,
//                                       buf: *mut ::std::os::raw::c_char,
//                                       bufsize: usize, result: *mut usize)
//      -> napi_status;


//     pub fn napi_get_value_string_utf16(env: napi_env, value: napi_value,
//                                        buf: *mut char16_t, bufsize: usize,
//                                        result: *mut usize) -> napi_status;


//     pub fn napi_coerce_to_bool(env: napi_env, value: napi_value,
//                                result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_number(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_object(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_coerce_to_string(env: napi_env, value: napi_value,
//                                  result: *mut napi_value) -> napi_status;


//     pub fn napi_get_prototype(env: napi_env, object: napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_get_property_names(env: napi_env, object: napi_value,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_set_property(env: napi_env, object: napi_value,
//                              key: napi_value, value: napi_value)
//      -> napi_status;


//     pub fn napi_has_property(env: napi_env, object: napi_value,
//                              key: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_property(env: napi_env, object: napi_value,
//                              key: napi_value, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_set_named_property(env: napi_env, object: napi_value,
//                                    utf8name: *const ::std::os::raw::c_char,
//                                    value: napi_value) -> napi_status;


//     pub fn napi_has_named_property(env: napi_env, object: napi_value,
//                                    utf8name: *const ::std::os::raw::c_char,
//                                    result: *mut bool) -> napi_status;


//     pub fn napi_get_named_property(env: napi_env, object: napi_value,
//                                    utf8name: *const ::std::os::raw::c_char,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_set_element(env: napi_env, object: napi_value, index: u32,
//                             value: napi_value) -> napi_status;


//     pub fn napi_has_element(env: napi_env, object: napi_value, index: u32,
//                             result: *mut bool) -> napi_status;


//     pub fn napi_get_element(env: napi_env, object: napi_value, index: u32,
//                             result: *mut napi_value) -> napi_status;


//     pub fn napi_define_properties(env: napi_env, object: napi_value,
//                                   property_count: usize,
//                                   properties: *const napi_property_descriptor)
//      -> napi_status;


//     pub fn napi_is_array(env: napi_env, value: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_array_length(env: napi_env, value: napi_value,
//                                  result: *mut u32) -> napi_status;


//     pub fn napi_strict_equals(env: napi_env, lhs: napi_value, rhs: napi_value,
//                               result: *mut bool) -> napi_status;


//     pub fn napi_call_function(env: napi_env, recv: napi_value,
//                               func: napi_value, argc: usize,
//                               argv: *const napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_new_instance(env: napi_env, constructor: napi_value,
//                              argc: usize, argv: *const napi_value,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_instanceof(env: napi_env, object: napi_value,
//                            constructor: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_make_callback(env: napi_env, recv: napi_value,
//                               func: napi_value, argc: usize,
//                               argv: *const napi_value,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_get_cb_info(env: napi_env, cbinfo: napi_callback_info,
//                             argc: *mut usize, argv: *mut napi_value,
//                             this_arg: *mut napi_value,
//                             data: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;


//     pub fn napi_is_construct_call(env: napi_env, cbinfo: napi_callback_info,
//                                   result: *mut bool) -> napi_status;


//     pub fn napi_define_class(env: napi_env,
//                              utf8name: *const ::std::os::raw::c_char,
//                              constructor: napi_callback,
//                              data: *mut ::std::os::raw::c_void,
//                              property_count: usize,
//                              properties: *const napi_property_descriptor,
//                              result: *mut napi_value) -> napi_status;


//     pub fn napi_wrap(env: napi_env, js_object: napi_value,
//                      native_object: *mut ::std::os::raw::c_void,
//                      finalize_cb: napi_finalize,
//                      finalize_hint: *mut ::std::os::raw::c_void,
//                      result: *mut napi_ref) -> napi_status;


//     pub fn napi_unwrap(env: napi_env, js_object: napi_value,
//                        result: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;


//     pub fn napi_create_external(env: napi_env,
//                                 data: *mut ::std::os::raw::c_void,
//                                 finalize_cb: napi_finalize,
//                                 finalize_hint: *mut ::std::os::raw::c_void,
//                                 result: *mut napi_value) -> napi_status;


//     pub fn napi_get_value_external(env: napi_env, value: napi_value,
//                                    result: *mut *mut ::std::os::raw::c_void)
//      -> napi_status;


//     pub fn napi_create_reference(env: napi_env, value: napi_value,
//                                  initial_refcount: u32, result: *mut napi_ref)
//      -> napi_status;


//     pub fn napi_delete_reference(env: napi_env, ref_: napi_ref)
//      -> napi_status;


//     pub fn napi_reference_ref(env: napi_env, ref_: napi_ref, result: *mut u32)
//      -> napi_status;


//     pub fn napi_reference_unref(env: napi_env, ref_: napi_ref,
//                                 result: *mut u32) -> napi_status;


//     pub fn napi_get_reference_value(env: napi_env, ref_: napi_ref,
//                                     result: *mut napi_value) -> napi_status;


//     pub fn napi_open_handle_scope(env: napi_env,
//                                   result: *mut napi_handle_scope)
//      -> napi_status;


//     pub fn napi_close_handle_scope(env: napi_env, scope: napi_handle_scope)
//      -> napi_status;


//     pub fn napi_open_escapable_handle_scope(env: napi_env,
//                                             result:
//                                                 *mut napi_escapable_handle_scope)
//      -> napi_status;


//     pub fn napi_close_escapable_handle_scope(env: napi_env,
//                                              scope:
//                                                  napi_escapable_handle_scope)
//      -> napi_status;


//     pub fn napi_escape_handle(env: napi_env,
//                               scope: napi_escapable_handle_scope,
//                               escapee: napi_value, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_throw(env: napi_env, error: napi_value) -> napi_status;


//     pub fn napi_throw_error(env: napi_env, msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_throw_type_error(env: napi_env,
//                                  msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_throw_range_error(env: napi_env,
//                                   msg: *const ::std::os::raw::c_char)
//      -> napi_status;


//     pub fn napi_is_error(env: napi_env, value: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_is_exception_pending(env: napi_env, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_and_clear_last_exception(env: napi_env,
//                                              result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_buffer(env: napi_env, length: usize,
//                               data: *mut *mut ::std::os::raw::c_void,
//                               result: *mut napi_value) -> napi_status;


//     pub fn napi_create_external_buffer(env: napi_env, length: usize,
//                                        data: *mut ::std::os::raw::c_void,
//                                        finalize_cb: napi_finalize,
//                                        finalize_hint:
//                                            *mut ::std::os::raw::c_void,
//                                        result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_create_buffer_copy(env: napi_env, length: usize,
//                                    data: *const ::std::os::raw::c_void,
//                                    result_data:
//                                        *mut *mut ::std::os::raw::c_void,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_is_buffer(env: napi_env, value: napi_value, result: *mut bool)
//      -> napi_status;


//     pub fn napi_get_buffer_info(env: napi_env, value: napi_value,
//                                 data: *mut *mut ::std::os::raw::c_void,
//                                 length: *mut usize) -> napi_status;


//     pub fn napi_is_arraybuffer(env: napi_env, value: napi_value,
//                                result: *mut bool) -> napi_status;


//     pub fn napi_create_arraybuffer(env: napi_env, byte_length: usize,
//                                    data: *mut *mut ::std::os::raw::c_void,
//                                    result: *mut napi_value) -> napi_status;


//     pub fn napi_create_external_arraybuffer(env: napi_env,
//                                             external_data:
//                                                 *mut ::std::os::raw::c_void,
//                                             byte_length: usize,
//                                             finalize_cb: napi_finalize,
//                                             finalize_hint:
//                                                 *mut ::std::os::raw::c_void,
//                                             result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_get_arraybuffer_info(env: napi_env, arraybuffer: napi_value,
//                                      data: *mut *mut ::std::os::raw::c_void,
//                                      byte_length: *mut usize) -> napi_status;


//     pub fn napi_is_typedarray(env: napi_env, value: napi_value,
//                               result: *mut bool) -> napi_status;


//     pub fn napi_create_typedarray(env: napi_env, type_: napi_typedarray_type,
//                                   length: usize, arraybuffer: napi_value,
//                                   byte_offset: usize, result: *mut napi_value)
//      -> napi_status;


//     pub fn napi_get_typedarray_info(env: napi_env, typedarray: napi_value,
//                                     type_: *mut napi_typedarray_type,
//                                     length: *mut usize,
//                                     data: *mut *mut ::std::os::raw::c_void,
//                                     arraybuffer: *mut napi_value,
//                                     byte_offset: *mut usize) -> napi_status;


//     pub fn napi_create_async_work(env: napi_env,
//                                   execute: napi_async_execute_callback,
//                                   complete: napi_async_complete_callback,
//                                   data: *mut ::std::os::raw::c_void,
//                                   result: *mut napi_async_work)
//      -> napi_status;


//     pub fn napi_delete_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;


//     pub fn napi_queue_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;


//     pub fn napi_cancel_async_work(env: napi_env, work: napi_async_work)
//      -> napi_status;
