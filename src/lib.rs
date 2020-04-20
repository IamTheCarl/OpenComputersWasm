
#[macro_use]
extern crate lazy_static;

use slotmap::KeyData;

use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JObject;
use jni::sys::jlong;

use slotmap::SlotMap;
use slotmap::new_key_type;
use std::sync::RwLock;

mod java_encapsulation;
use java_encapsulation as je;

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

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_runInstance<'a>(env: JNIEnv<'a>, _class: JClass, _id: jlong, _machine: JObject) -> JObject<'a> {
    // Signature for machine: Lli/cil/oc/api/machine/Machine;

    je::ExecutionResult::Sleep(10).create_jobject(&env)
    // je::ExecutionResult::Shutdown(false).create_jobject(&env)
    // je::ExecutionResult::SynchronizedCall.create_jobject(&env)
    // je::ExecutionResult::Error(format!("Nothing actually went wrong.")).create_jobject(&env)
}

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_runSynchronized<'a>(env: JNIEnv<'a>, _class: JClass, _id: jlong, _machine: JObject) {
    // Signature for machine: Lli/cil/oc/api/machine/Machine;

    je::log::debug(&env, "Synchronized run.");
}