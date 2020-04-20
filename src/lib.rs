
#[macro_use]
extern crate lazy_static;

use slotmap::KeyData;
use jni::JNIEnv;

use jni::objects::JClass;
use jni::objects::JObject;
use jni::signature::JavaType;
use jni::signature::Primitive;
use jni::objects::JValue;
use jni::sys::jlong;

use slotmap::SlotMap;
use slotmap::new_key_type;
use std::sync::RwLock;

#[derive(Copy, Clone)]
struct WasmMachine {

}

const POISON_MESSAGE: &str = "Wasm VM list has been poisoned.";
const FAIL_FIND_APACHE_LOGGER_CLASS: &str = "Failed to find apache logger class.";
const FAIL_CREATE_JAVA_STRING: &str = "Failed to create Java string.";

new_key_type! { struct MachineKey; }
lazy_static! {
    static ref MACHINE_LIST: RwLock<SlotMap<MachineKey, WasmMachine>> = RwLock::new(SlotMap::with_key());
}

pub fn log_info(env: &JNIEnv, logger: &JObject, message: &str) {
    let logger_class = env.find_class("org/apache/logging/log4j/Logger")
                        .expect(FAIL_FIND_APACHE_LOGGER_CLASS);

    let log_method_id = env.get_method_id(logger_class, "info", "(Ljava/lang/String;)V")
       .expect("Could not obtain Java info logger method.");

    let output = env.new_string(message)
       .expect(FAIL_CREATE_JAVA_STRING);

    env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
       .expect("Something went wrong calling the logger info method.");
}

pub fn log_debug(env: &JNIEnv, logger: &JObject, message: &str) {
    let logger_class = env.find_class("org/apache/logging/log4j/Logger")
                        .expect(FAIL_FIND_APACHE_LOGGER_CLASS);

    let log_method_id = env.get_method_id(logger_class, "debug", "(Ljava/lang/String;)V")
                        .expect("Could not obtain Java debug logger method.");

    let output = env.new_string(message)
       .expect(FAIL_CREATE_JAVA_STRING);

    env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
       .expect("Something went wrong calling the logger info method.");
}

pub fn log_warn(env: &JNIEnv, logger: &JObject, message: &str) {
    let logger_class = env.find_class("org/apache/logging/log4j/Logger")
                        .expect(FAIL_FIND_APACHE_LOGGER_CLASS);

    let log_method_id = env.get_method_id(logger_class, "warn", "(Ljava/lang/String;)V")
                        .expect("Could not obtain Java warn logger method.");

    let output = env.new_string(message)
       .expect(FAIL_CREATE_JAVA_STRING);

    env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
       .expect("Something went wrong calling the logger info method.");
}

pub fn log_error(env: &JNIEnv, logger: &JObject, message: &str) {
    let logger_class = env.find_class("org/apache/logging/log4j/Logger")
                        .expect(FAIL_FIND_APACHE_LOGGER_CLASS);

    let log_method_id = env.get_method_id(logger_class, "error", "(Ljava/lang/String;)V")
                        .expect("Could not obtain Java error logger method.");

    let output = env.new_string(message)
       .expect(FAIL_CREATE_JAVA_STRING);

    env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
       .expect("Something went wrong calling the logger info method.");
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_setup(env: JNIEnv, _class: JClass, logger: JObject) {
    log_info(&env, &logger, "Rust side of the WASM interpreter has been setup.");
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_createWasmInstance(_env: JNIEnv, _class: JClass) -> jlong {

    // Get a write lock, because we actually need to write to this list.
    let mut machines = MACHINE_LIST.write().expect(POISON_MESSAGE);
    let key = machines.insert(WasmMachine{});
    let key_data: KeyData = key.into();
    let id = key_data.as_ffi() as i64;

    return id;
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_destoryWasmInstance(_env: JNIEnv, _class: JClass, id: jlong) {
    let key_data = KeyData::from_ffi(id as u64);
    let key = MachineKey::from(key_data);

    // Get a write lock, because we actually need to write to this list.
    let mut machines = MACHINE_LIST.write().expect(POISON_MESSAGE);
    machines.remove(key);
}