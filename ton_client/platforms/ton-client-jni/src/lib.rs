#[macro_use]
extern crate lazy_static;
extern crate jni;
extern crate ton_client;

pub mod client {
    use ton_client::{ContextHandle, create_context, destroy_context, request};
    use jni::{JavaVM, JNIEnv};
    use jni::sys::{jint};
    use jni::objects::{JClass, JString, JValue};
    use std::sync::{Mutex};

    lazy_static! {
        static ref DATA: Mutex<Wrapper> = Mutex::new(Wrapper {data:None});
    }
    struct Data {
        jvm: JavaVM,
    }

    struct Wrapper {
        data: Option<Data>,
    }

    impl Wrapper {
        pub fn set_data(&mut self, data: Data) {
            self.data = Some(data);
        }
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern fn Java_ton_sdk_TONContext_createContext<'a>(
        env: JNIEnv<'a>,
        _class: JClass,
        config: JString
    ) -> JString<'a> {

        {
            let mut wrapper = DATA.lock().unwrap();
            if  wrapper.data.is_none() {
                wrapper.set_data(Data {
                    jvm: env.get_java_vm().unwrap(),
                });
            }
        }

        env.new_string(
            create_context(env.get_string(config).unwrap().into())
        ).unwrap()
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern fn Java_ton_sdk_TONContext_destroyContext(
        _env: JNIEnv,
        _class: JClass,
        context: jint,
    ) {
        destroy_context(context as ContextHandle)
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    pub unsafe extern fn Java_ton_sdk_TONContext_request(
        env: JNIEnv,
        _class: JClass,
        context: jint,
        method: JString,
        params: JString,
        request_id: jint,
    ) {
        request(
            context as ContextHandle,
            env.get_string(method).unwrap().into(),
            env.get_string(params).unwrap().into(),
            request_id as u32,
            handler_callback
        );
    }

    fn handler_callback(request_id: u32, params: String, response_type: u32, finished: bool) {
        let wr = &DATA.lock().unwrap();
        let jvm = &wr.data.as_ref().unwrap().jvm;
        drop(wr);

        let env = jvm.attach_current_thread().unwrap();
        let class = env.find_class("ton/sdk/TONContext").unwrap();
        env.call_static_method(
            class,
            "responseHandler",
            "(ILjava/lang/String;IZ)V",
            &[
                JValue::Int(request_id as i32),
                JValue::Object(env.new_string(params.as_str()).unwrap().into()),
                JValue::Int(response_type as i32),
                JValue::Bool(finished as u8),
             ],
        ).unwrap();
    }
}


