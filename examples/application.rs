use cxx_juce::{
    juce_core::JuceString,
    juce_events::{App, AppHandle, AppTimerId, On},
    JUCE,
};
use std::{thread, time::Duration};

#[derive(Default)]
struct ExampleApp {
    task_result: Option<i32>,
}

impl App for ExampleApp {
    fn name(&self) -> JuceString {
        "Example App".into()
    }

    fn version(&self) -> JuceString {
        "1.0".into()
    }

    fn initialise(&mut self, app: AppHandle<Self>) {
        println!("🧃 Initialising {} v{}...", self.name(), self.version());

        app.start_timer(Duration::from_secs(3));

        thread::spawn(move || background_task(app));
    }

    fn shutdown(&mut self) {
        println!("👋 Cya!")
    }

    fn timer_callback(&mut self, _handle: AppHandle<Self>, id: AppTimerId) {
        println!("⏲️ {id:?} callback");
    }
}

fn background_task(app: AppHandle<ExampleApp>) {
    for progress in (0..=100).step_by(10) {
        app.send(TaskStatus::InProgress(progress));

        if progress < 100 {
            thread::sleep(Duration::from_secs(1));
        }
    }

    app.call_on_main_thread(|app: &mut ExampleApp, _| {
        app.task_result = Some(42);
    });
    app.send(TaskStatus::Complete);
    app.quit();
}

enum TaskStatus {
    InProgress(i32),
    Complete,
}

impl On<TaskStatus> for ExampleApp {
    fn on(&mut self, status: TaskStatus) {
        match status {
            TaskStatus::InProgress(i) => {
                println!("✉️ Task {i}% complete...");
            }
            TaskStatus::Complete => {
                println!("✔️ Task complete! Result is {:?}", self.task_result);
            }
        }
    }
}

fn main() {
    JUCE::run_app(|_| ExampleApp::default());
}
