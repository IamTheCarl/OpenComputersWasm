
#[macro_use]
extern crate lazy_static;

use jni::sys::jboolean;
use slotmap::KeyData;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JObject;
use jni::sys::jlong;
use jni::errors::Result as JNIResult;

use slotmap::SlotMap;
use slotmap::new_key_type;
use std::sync::RwLock;

mod java_encapsulation;
use java_encapsulation as je;
use je::open_computers as oc;

#[derive(Copy, Clone)]
struct WasmMachine {

}

const POISON_MESSAGE: &str = "Wasm VM list has been poisoned.";
const FAIL_CREATE_JAVA_STRING: &str = "Failed to create Java string.";

new_key_type! { struct MachineKey; }
lazy_static! {
    static ref MACHINE_LIST: RwLock<SlotMap<MachineKey, WasmMachine>> = RwLock::new(SlotMap::with_key());
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_setup<'a>(env: JNIEnv<'a>, _class: JClass) {
    je::log::info(&env, "Rust side of the WASM interpreter has been setup.");
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_createWasmInstance<'a>(env: JNIEnv<'a>, _class: JClass) -> jlong {

    // Get a write lock, because we actually need to write to this list.
    let mut machines = MACHINE_LIST.write().expect(POISON_MESSAGE);
    let key = machines.insert(WasmMachine{});
    let key_data: KeyData = key.into();
    let id = key_data.as_ffi() as i64;

    je::log::debug(&env, &format!("Create WASM VM: {}", id));

    return id;
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_destoryWasmInstance<'a>(env: JNIEnv<'a>, _class: JClass, id: jlong) {
    let key_data = KeyData::from_ffi(id as u64);
    let key = MachineKey::from(key_data);

    // Get a write lock, because we actually need to write to this list.
    let mut machines = MACHINE_LIST.write().expect(POISON_MESSAGE);
    machines.remove(key);

    je::log::debug(&env, &format!("Destroy WASM VM: {}", id));
}

fn run_instance<'a>(env: JNIEnv<'a>, _class: JClass, id: jlong, last_run_was_synchronus: jboolean, machine: JObject) -> JNIResult<JObject<'a>> {
    // First get a reference to the in-world machine.
    let machine = oc::Machine::new(&env, machine)?;

    // Then we grab a reference to our VM.

    let key_data = KeyData::from_ffi(id as u64);
    let key = MachineKey::from(key_data);
    let machine_lock = MACHINE_LIST.read().expect(POISON_MESSAGE);
    let vm = machine_lock.get(key);

    let components = machine.get_components()?;
    for component in components.iter()? {
        let left: String = env.get_string(component.0.into()).unwrap().into();
        let right: String = env.get_string(component.1.into()).unwrap().into();

        je::log::info(&env, &format!("Component: {}:{}", left, right));
    }

    if let Some(_vm) = vm {
        Ok(oc::ExecutionResult::Sleep(10).create_jobject(&env))
        // Ok(oc::ExecutionResult::Shutdown(false).create_jobject(&env))
        // Ok(oc::ExecutionResult::SynchronizedCall.create_jobject(&env))
    } else {
        Ok(oc::ExecutionResult::Error(format!("Failed to find machine {} in WASM VM list.", id)).create_jobject(&env))
    }
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_runInstance<'a>(
    env: JNIEnv<'a>, class: JClass, id: jlong, last_run_was_synchronus: jboolean, machine: JObject) -> JObject<'a> {

    let result = run_instance(env, class, id, last_run_was_synchronus, machine);

    match result {
        Err(error) => {
            match error.kind() {
                jni::errors::ErrorKind::JavaException => {
                    JObject::null()
                },
                _ => {
                    panic!("Panic in the native code: {}", error);
                }
            }
        },
        Ok(object) => {
            object
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_runSynchronized<'a>(env: JNIEnv<'a>, _class: JClass, _id: jlong, _machine: JObject) {

    je::log::debug(&env, "Synchronized run.");
}