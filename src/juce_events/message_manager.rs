use crate::define_trait;

pub(crate) use juce::MessageManager;

impl MessageManager {
    pub fn call_async(callback: impl FnOnce() + Send + 'static) {
        struct CallbackWrapper<F: FnOnce() + Send>(Option<F>);

        impl<F: FnOnce() + Send> CallAsyncCallback for CallbackWrapper<F> {
            fn call(&mut self) {
                if let Some(f) = self.0.take() {
                    f();
                }
            }
        }

        juce::call_async(Box::new(CallbackWrapper(Some(callback))))
    }

    pub fn has_stop_message_been_sent() -> bool {
        juce::has_stop_message_been_sent()
    }

    pub fn run_dispatch_loop() {
        juce::run_dispatch_loop();
    }

    pub fn stop_dispatch_loop() {
        juce::stop_dispatch_loop()
    }
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type MessageManager;

        #[namespace = "cxx_juce"]
        type BoxDynCallAsyncCallback = Box<dyn super::CallAsyncCallback>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "callAsync"]
        fn call_async(callback: BoxDynCallAsyncCallback);

        #[namespace = "cxx_juce"]
        #[cxx_name = "hasStopMessageBeenSent"]
        fn has_stop_message_been_sent() -> bool;

        #[namespace = "cxx_juce"]
        #[cxx_name = "runDispatchLoop"]
        fn run_dispatch_loop();

        #[namespace = "cxx_juce"]
        #[cxx_name = "stopDispatchLoop"]
        fn stop_dispatch_loop();
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        type CallAsyncCallbackImpl;

        #[Self = "CallAsyncCallbackImpl"]
        unsafe fn drop(callback: *mut BoxDynCallAsyncCallback);

        #[Self = "CallAsyncCallbackImpl"]
        fn call(callback: &mut BoxDynCallAsyncCallback);
    }
}

define_trait! {
    CallAsyncCallback: Send,
    CallAsyncCallbackImpl,
    "cxx_juce::BoxDynCallAsyncCallback",
    fn call(&mut self);
}
