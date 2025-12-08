use crate::{define_trait, juce_core::JuceString, juce_events::MessageManager, JUCE};
use cxx::UniquePtr;
use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
    thread_local,
    time::Duration,
};

pub trait App {
    fn name(&self) -> JuceString;
    fn version(&self) -> JuceString;
    fn initialise(&mut self, handle: AppHandle<Self>);
    fn shutdown(&mut self);
    fn timer_callback(&mut self, callback: AppHandle<Self>, timer: AppTimerId);
}

pub struct AppHandle<T>
where
    T: ?Sized,
{
    state: Arc<AppState>,
    _marker: PhantomData<T>,
}

impl<T> Clone for AppHandle<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            _marker: PhantomData,
        }
    }
}

pub trait On<Message>: App {
    fn on(&mut self, message: Message);
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct AppTimerId(i32);

struct AppWrapper<T> {
    app: T,
    state: Arc<AppState>,
}

impl<T> AppWrapper<T> {
    fn handle(&self) -> AppHandle<T> {
        let state = Arc::clone(&self.state);
        AppHandle {
            state,
            _marker: PhantomData,
        }
    }
}

#[derive(Default)]
struct AppState {
    timer_id: AtomicI32,
}

impl<T> JuceApplicationBase for RefCell<AppWrapper<T>>
where
    T: App + 'static,
{
    fn get_application_name(&self) -> JuceString {
        debug_assert!(JUCE::is_this_the_message_thread());

        self.borrow().app.name()
    }

    fn get_application_version(&self) -> JuceString {
        debug_assert!(JUCE::is_this_the_message_thread());

        self.borrow().app.version()
    }

    fn initialise(&mut self, _: &JuceString) {
        debug_assert!(JUCE::is_this_the_message_thread());

        let handle = self.borrow().handle();
        self.borrow_mut().app.initialise(handle);
    }

    fn shutdown(&mut self) {
        debug_assert!(JUCE::is_this_the_message_thread());

        self.borrow_mut().app.shutdown();
    }

    fn system_requested_quit(&mut self) {}

    fn more_than_one_instance_allowed(&self) -> bool {
        true
    }

    fn another_instance_started(&mut self, _: &JuceString) {}

    fn suspended(&mut self) {}

    fn resumed(&mut self) {}

    fn unhandled_exception(&mut self) {}

    fn timer_callback(&mut self, timer_id: i32) {
        debug_assert!(JUCE::is_this_the_message_thread());

        let handle = self.borrow().handle();
        self.borrow_mut()
            .app
            .timer_callback(handle, AppTimerId(timer_id));
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<A> AppHandle<A>
where
    A: App + 'static,
{
    pub fn send<M>(&self, message: M)
    where
        M: Send + 'static,
        A: On<M>,
    {
        MessageManager::call_async(|| {
            with_app(|app: &mut A| {
                app.on(message);
            });
        });
    }

    pub fn start_timer(&self, interval: Duration) -> AppTimerId {
        let timer_id = self.state.timer_id.fetch_add(1, Ordering::Relaxed);
        let interval = interval.as_millis().try_into().unwrap_or(i32::MAX);

        MessageManager::call_async(move || {
            juce::start_app_timer(timer_id, interval);
        });

        AppTimerId(timer_id)
    }

    pub fn stop_timer(&self, AppTimerId(timer_id): AppTimerId) {
        MessageManager::call_async(move || {
            juce::stop_app_timer(timer_id);
        });
    }

    pub fn call_on_main_thread(&self, func: impl FnOnce(&mut A, AppHandle<A>) + Send + 'static) {
        MessageManager::call_async(|| {
            with_instance(|wrapper| {
                let handle = wrapper.handle();
                func(&mut wrapper.app, handle);
            });
        });
    }

    pub fn quit(self) {
        juce::JUCEApplicationBase::quit()
    }

    pub fn is_quitting(&self) -> bool {
        MessageManager::has_stop_message_been_sent()
    }
}

type AppConstructor = Box<dyn FnOnce() -> UniquePtr<juce::JUCEApplicationBase> + 'static>;

thread_local! {
    static CREATE_INSTANCE: RefCell<Option<AppConstructor>> = RefCell::new(None);
}

pub(crate) fn run_app<F, A>(app: F) -> i32
where
    F: FnOnce(&JUCE) -> A + 'static,
    A: App + 'static,
{
    CREATE_INSTANCE.replace(Some(Box::new(move || {
        let juce = JUCE::initialise();
        let app = AppWrapper {
            app: app(&juce),
            state: Arc::default(),
        };

        juce::wrap_juce_application(Box::new(RefCell::new(app)))
    })));

    juce::run_app()
}

fn create_instance() -> UniquePtr<juce::JUCEApplicationBase> {
    match CREATE_INSTANCE.take() {
        Some(constructor) => constructor(),
        None => UniquePtr::null(),
    }
}

fn with_app<A, F, R>(func: F) -> Option<R>
where
    A: App + 'static,
    F: FnOnce(&mut A) -> R,
    R: 'static,
{
    with_instance(|instance| func(&mut instance.app))
}

fn with_instance<A, F, R>(func: F) -> Option<R>
where
    A: App + 'static,
    F: FnOnce(&mut AppWrapper<A>) -> R,
    R: 'static,
{
    assert!(
        JUCE::is_this_the_message_thread(),
        "this function must be called on the message thread"
    );

    unsafe { juce::cast_app_instance().as_ref() }
        .and_then(|instance| instance.as_ref().as_any().downcast_ref())
        .and_then(|app: &RefCell<AppWrapper<A>>| {
            let app = app.try_borrow_mut();

            debug_assert!(
                app.is_ok(),
                "app is already borrowed mutably, did you call this recursively?"
            );

            app.ok().map(|mut app| func(&mut app))
        })
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    unsafe extern "C++" {
        include!("cxx_juce.h");

        type JUCEApplicationBase;

        type JuceString = crate::juce_core::JuceString;

        #[namespace = "cxx_juce"]
        type BoxDynJuceApplicationBase = Box<dyn super::JuceApplicationBase>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "wrap"]
        fn wrap_juce_application(app: BoxDynJuceApplicationBase) -> UniquePtr<JUCEApplicationBase>;

        #[namespace = "cxx_juce"]
        #[cxx_name = "castAppInstance"]
        fn cast_app_instance() -> *mut BoxDynJuceApplicationBase;

        #[namespace = "cxx_juce"]
        #[cxx_name = "runApp"]
        fn run_app() -> i32;

        #[namespace = "cxx_juce"]
        #[cxx_name = "startAppTimer"]
        fn start_app_timer(id: i32, interval: i32);

        #[namespace = "cxx_juce"]
        #[cxx_name = "stopAppTimer"]
        fn stop_app_timer(id: i32);

        #[Self = "JUCEApplicationBase"]
        fn quit();
    }

    #[namespace = "cxx_juce"]
    extern "Rust" {
        #[cxx_name = "createInstance"]
        fn create_instance() -> UniquePtr<JUCEApplicationBase>;

        type JuceApplicationBaseImpl;

        #[Self = "JuceApplicationBaseImpl"]
        unsafe fn drop(app: *mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "getApplicationName"]
        fn get_application_name(app: &BoxDynJuceApplicationBase) -> JuceString;

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "getApplicationVersion"]
        fn get_application_version(app: &BoxDynJuceApplicationBase) -> JuceString;

        #[Self = "JuceApplicationBaseImpl"]
        fn initialise(app: &mut BoxDynJuceApplicationBase, command_line: &JuceString);

        #[Self = "JuceApplicationBaseImpl"]
        fn shutdown(app: &mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "systemRequestedQuit"]
        fn system_requested_quit(app: &mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "moreThanOneInstanceAllowed"]
        fn more_than_one_instance_allowed(app: &BoxDynJuceApplicationBase) -> bool;

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "anotherInstanceStarted"]
        fn another_instance_started(app: &mut BoxDynJuceApplicationBase, command_line: &JuceString);

        #[Self = "JuceApplicationBaseImpl"]
        fn suspended(app: &mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        fn resumed(app: &mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "unhandledException"]
        fn unhandled_exception(app: &mut BoxDynJuceApplicationBase);

        #[Self = "JuceApplicationBaseImpl"]
        #[cxx_name = "timerCallback"]
        fn timer_callback(app: &mut BoxDynJuceApplicationBase, timer_id: i32);
    }
}

define_trait! {
    /// A trait that can be implemented to create a JUCE application.
    JuceApplicationBase,
    JuceApplicationBaseImpl,
    "cxx_juce::BoxDynJuceApplicationBase",

    /// Returns the name of this application.
    fn get_application_name(&self) -> JuceString;

    /// Returns the version number of this application.
    fn get_application_version(&self) -> JuceString;

    /// Called when the application starts.
    ///
    /// The command_line parameter contains the command line arguments.
    fn initialise(&mut self, command_line_args: &JuceString);

    /// Called when the application is being shut down.
    ///
    /// This is the last chance to clean up and save any state before the application exits.
    fn shutdown(&mut self);

    /// Called when the system requests that the application quit.
    fn system_requested_quit(&mut self);

    /// Returns whether more than one instance of the application is allowed.
    fn more_than_one_instance_allowed(&self) -> bool;

    /// Called when another instance of the application is started.
    fn another_instance_started(&mut self, command_line: &JuceString);

    /// Called when the application is suspended (mobile/background).
    fn suspended(&mut self);

    /// Called when the application is resumed from suspension.
    fn resumed(&mut self);

    /// Called when an unhandled exception occurs.
    fn unhandled_exception(&mut self);

    /// Callback for timer events.
    fn timer_callback(&mut self, timer_id: i32);

    fn as_any(&self) -> &dyn Any, @nobind;
}
