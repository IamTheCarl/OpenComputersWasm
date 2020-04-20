
use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JObject;
use jni::objects::JValue;
use jni::signature::JavaType;
use jni::signature::Primitive;

#[allow(dead_code)]
pub mod log {
    use super::*;

    fn get_logger<'a>(env: &JNIEnv<'a>) -> JObject<'a> {
        let arch_class = env.find_class("com/thecarl/ocwasm/WasmArch")
                            .expect("Failed to get WasmArch class.");

        let logger = env.get_static_field(arch_class, "nativeLogger", "Lorg/apache/logging/log4j/Logger;")
            .expect("Failed to obtain Native Side logger.");

        match logger {
            JValue::Object(object) => object,
            _ => panic!("com.thecarl.ocwasm.WasmArch.nativeLogger is not a logger."),
        }
    }

    fn get_logger_class<'a>(env: &JNIEnv<'a>) -> JClass<'a> {
        env.find_class("org/apache/logging/log4j/Logger")
            .expect("Failed to find apache logger class.")
    }

    pub fn info(env: &JNIEnv, message: &str) {
        let logger_class = get_logger_class(&env);
        let logger = get_logger(&env);
    
        let log_method_id = env.get_method_id(logger_class, "info", "(Ljava/lang/String;)V")
           .expect("Could not obtain Java info logger method.");
    
        let output = env.new_string(message)
           .expect(crate::FAIL_CREATE_JAVA_STRING);
    
        env.call_method_unchecked(logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
           .expect("Something went wrong calling the logger info method.");
    }
    
    pub fn debug(env: &JNIEnv, message: &str) {
        let logger_class = get_logger_class(&env);
        let logger = get_logger(&env);
    
        let log_method_id = env.get_method_id(logger_class, "debug", "(Ljava/lang/String;)V")
                            .expect("Could not obtain Java debug logger method.");
    
        let output = env.new_string(message)
           .expect(crate::FAIL_CREATE_JAVA_STRING);
    
        env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
           .expect("Something went wrong calling the logger info method.");
    }
    
    pub fn warn(env: &JNIEnv, message: &str) {
        let logger_class = get_logger_class(&env);
        let logger = get_logger(&env);
    
        let log_method_id = env.get_method_id(logger_class, "warn", "(Ljava/lang/String;)V")
                            .expect("Could not obtain Java warn logger method.");
    
        let output = env.new_string(message)
           .expect(crate::FAIL_CREATE_JAVA_STRING);
    
        env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
           .expect("Something went wrong calling the logger info method.");
    }
    
    pub fn error(env: &JNIEnv, message: &str) {
        let logger_class = get_logger_class(&env);
        let logger = get_logger(&env);
    
        let log_method_id = env.get_method_id(logger_class, "error", "(Ljava/lang/String;)V")
                            .expect("Could not obtain Java error logger method.");
    
        let output = env.new_string(message)
           .expect(crate::FAIL_CREATE_JAVA_STRING);
    
        env.call_method_unchecked(*logger, log_method_id, JavaType::Primitive(Primitive::Void), &[JValue::Object(output.into())])
           .expect("Something went wrong calling the logger info method.");
    }
}

#[allow(dead_code)]
pub enum ExecutionResult {
    Sleep(i32),
    Shutdown(bool),
    SynchronizedCall,
    Error(String)
}

impl ExecutionResult {
    pub fn create_jobject<'a>(&self, env: &JNIEnv<'a>) -> JObject<'a> {
        match self {
            Self::Sleep(time) => {
                let class = env.find_class("li/cil/oc/api/machine/ExecutionResult$Sleep")
                    .expect("Failed to find ExecutionResult.Sleep class.");
    
                env.new_object(
                    class,
                    "(I)V",
                    &[JValue::Int(*time)])
                    .expect("Failed to construct ExecutionResult.Sleep object.") 
            },
            Self::Shutdown(reboot) => {
                let class = env.find_class("li/cil/oc/api/machine/ExecutionResult$Shutdown")
                    .expect("Failed to find ExecutionResult.Shutdown class.");
    
                // Need boolean as integer. Yes, it feels weird.
                let reboot = if *reboot {
                    1
                } else {
                    0
                };

                env.new_object(
                    class,
                    "(Z)V",
                    &[JValue::Bool(reboot)])
                    .expect("Failed to construct ExecutionResult.Shutdown object.") 
            },
            Self::SynchronizedCall => {
                let class = env.find_class("li/cil/oc/api/machine/ExecutionResult$SynchronizedCall")
                    .expect("Failed to find ExecutionResult.SynchronizedCall class.");

                env.new_object(
                    class,
                    "()V",
                    &[])
                    .expect("Failed to construct ExecutionResult.SynchronizedCall object.") 
            },
            Self::Error(message) => {
                let class = env.find_class("li/cil/oc/api/machine/ExecutionResult$Error")
                    .expect("Failed to find ExecutionResult.Error class.");

                let message = env.new_string(message)
                    .expect(crate::FAIL_CREATE_JAVA_STRING);

                env.new_object(
                    class,
                    "(Ljava/lang/String;)V",
                    &[JValue::Object(message.into())])
                    .expect("Failed to construct ExecutionResult.Error object.") 
            }
        }
    }
}