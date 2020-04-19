
use jni::JNIEnv;

use jni::objects::JClass;
use jni::sys::jstring;

#[no_mangle]
pub extern "system" fn Java_com_thecarl_ocwasm_WasmArch_hello(env: JNIEnv,
                                            _class: JClass)
                                            -> jstring {
    let output = env.new_string("New string created in Rust.")
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}