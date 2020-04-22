
use std::iter::Map;
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

pub mod open_computers {

    use jni::objects::JString;
    use jni::objects::JMap;
    use jni::objects::JMethodID;
    use jni::errors::Result as JNIResult;
    use super::*;

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

    pub struct Signal {

    }
    
    pub struct Callback {
        
    }

    pub struct Value {

    }

    pub struct Environment {

    }

    pub enum MethodValue {
        Byte(i8),
        Short(i16),
        Int(i32),
        Long(i64),
        Float(f32),
        Double(f64),
        Str(String),
        Array(Vec<MethodValue>),
    }

    pub enum ValEnviron {
        Value(Value),
        Environment(Environment),
    }
    
    pub struct Machine<'a> {
        env: &'a JNIEnv<'a>,
        machine: JObject<'a>,
        get_components_method_id: JMethodID<'a>,
        component_count_id: JMethodID<'a>,
        max_component_count_id: JMethodID<'a>,
        get_cost_per_tick_id: JMethodID<'a>,
        set_cost_per_tick_id: JMethodID<'a>,
        get_tmp_address_id: JMethodID<'a>,
        get_last_error_id: JMethodID<'a>,
        get_world_time_id: JMethodID<'a>,
        get_uptime_id: JMethodID<'a>,
        get_cpu_time_id: JMethodID<'a>,
        beep_id: JMethodID<'a>,
        crash_id: JMethodID<'a>,
        pop_signal_id: JMethodID<'a>,
        methods_id: JMethodID<'a>,
        invoke_id: JMethodID<'a>,
        get_users_id: JMethodID<'a>,
        add_user_id: JMethodID<'a>,
        remove_user_id: JMethodID<'a>,
    }
    
    #[allow(dead_code)]
    impl<'a> Machine<'a> {

        pub fn new(env: &'a JNIEnv<'a>, machine: JObject<'a>) -> JNIResult<Machine<'a>> {
            let class = env.auto_local(env.find_class("li/cil/oc/api/machine/Machine")?);

            let get_components_method_id = env.get_method_id(&class, "components", "()Ljava/util/Map;")?;
            let component_count_id = env.get_method_id(&class, "componentCount", "()I")?;
            let max_component_count_id = env.get_method_id(&class, "maxComponents", "()I")?;
            let get_cost_per_tick_id = env.get_method_id(&class, "getCostPerTick", "()D")?;
            let set_cost_per_tick_id = env.get_method_id(&class, "setCostPerTick", "(D)V")?;
            let get_tmp_address_id = env.get_method_id(&class, "tmpAddress", "()Ljava/lang/String;")?;
            let get_last_error_id = env.get_method_id(&class, "lastError", "()Ljava/lang/String;")?;
            let get_world_time_id = env.get_method_id(&class, "worldTime", "()J")?;
            let get_uptime_id = env.get_method_id(&class, "upTime", "()D")?;
            let get_cpu_time_id = env.get_method_id(&class, "cpuTime", "()D")?;
            let beep_id = env.get_method_id(&class, "beep", "(SS)V")?;
            let crash_id = env.get_method_id(&class, "crash", "(Ljava/lang/String;)Z")?;
            let pop_signal_id = env.get_method_id(&class, "popSignal", "()Lli/cil/oc/api/machine/Signal;")?;
            let methods_id = env.get_method_id(&class, "methods", "(Ljava/lang/Object;)Ljava/util/Map;")?;
            let invoke_id = env.get_method_id(&class, "invoke", "(Ljava/lang/String;Ljava/lang/String;[Ljava/lang/Object;)[Ljava/lang/Object;")?;
            let get_users_id = env.get_method_id(&class, "users", "()[Ljava/lang/String;")?;
            let add_user_id = env.get_method_id(&class, "addUser", "(Ljava/lang/String;)V")?;
            let remove_user_id = env.get_method_id(&class, "removeUser", "(Ljava/lang/String;)Z")?;

            Ok(Machine {
                env,
                machine,
                get_components_method_id,
                component_count_id,
                max_component_count_id,
                get_cost_per_tick_id,
                set_cost_per_tick_id,
                get_tmp_address_id,
                get_last_error_id,
                get_world_time_id,
                get_uptime_id,
                get_cpu_time_id,
                beep_id,
                crash_id,
                pop_signal_id,
                methods_id,
                invoke_id,
                get_users_id,
                add_user_id,
                remove_user_id,
            })
        }
    
        pub fn get_components(&self) -> JNIResult<JMap> {
            let map = self.env.call_method_unchecked(
                self.machine,
                self.get_components_method_id,
                JavaType::Object(format!("java/util/Map")),
                &[]
            )?;

            if let JValue::Object(map) = map {
                self.env.get_map(map)
            } else {
                Err(jni::errors::Error::from_kind(jni::errors::ErrorKind::Msg(format!("Components method did not return a map."))))
            }
        }
    
        pub fn component_count(&self) -> JNIResult<u32> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.component_count_id,
                JavaType::Primitive(Primitive::Int),
                &[]
            )?;

            Ok(result.i().unwrap() as u32)
        }
    
        pub fn max_component_count(&self) -> JNIResult<u32> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.max_component_count_id,
                JavaType::Primitive(Primitive::Int),
                &[]
            )?;

            Ok(result.i().unwrap() as u32)
        }
    
        pub fn get_cost_per_tick(&self) -> JNIResult<f64> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.component_count_id,
                JavaType::Primitive(Primitive::Double),
                &[]
            )?;

            Ok(result.d().unwrap())
        }
    
        pub fn set_cost_per_tick(&self, price: f64) -> JNIResult<()>{
            unimplemented!()
        }
    
        pub fn get_tmp_address(&self) -> JNIResult<String> {
            unimplemented!()
        }
    
        pub fn get_last_error(&self) -> JNIResult<String> {
            unimplemented!()
        }
    
        pub fn get_world_time(&self) -> JNIResult<u64> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.get_world_time_id,
                JavaType::Primitive(Primitive::Int),
                &[]
            )?;

            Ok(result.j().unwrap() as u64)
        }
    
        pub fn get_uptime(&self) -> JNIResult<f64> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.get_uptime_id,
                JavaType::Primitive(Primitive::Int),
                &[]
            )?;

            Ok(result.d().unwrap())
        }
    
        pub fn get_cpu_time(&self) -> JNIResult<f64> {
            let result = self.env.call_method_unchecked(
                self.machine,
                self.get_cpu_time_id,
                JavaType::Primitive(Primitive::Int),
                &[]
            )?;

            Ok(result.d().unwrap())
        }
    
        pub fn beep(&self, frequency: u16, duration: u16) -> JNIResult<()> {
            unimplemented!()
        }
    
        pub fn crash(&self, message: &str) -> JNIResult<bool> {
            unimplemented!()
        }
    
        pub fn pop_signal(&self) -> JNIResult<Signal> {
            unimplemented!()
        }
    
        pub fn methods(&self, value: ValEnviron) -> JNIResult<Map<String, Callback>> {
            unimplemented!()
        }

        pub fn invoke(&self, address: &str, method: &str, args: &[MethodValue]) -> Result<Box<[MethodValue]>, ()> {
            // TODO remember to handle an exception correctly.
            unimplemented!()
        }

        pub fn get_users(&self) -> JNIResult<Box<[String]>> {
            unimplemented!()
        }

        pub fn add_user(&self, name: &str) -> JNIResult<()> {
            unimplemented!()
        }

        pub fn remove_user(&self, name: &str) -> JNIResult<bool> {
            unimplemented!()
        }
    }
}
