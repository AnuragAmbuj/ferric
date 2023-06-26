use std::fmt;
use std::fmt::{Display, Formatter, write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::task::{JoinHandle, self};

// Event type represents the different types of events that can occur
#[derive(Eq, PartialEq, Hash)]
enum Event {
    Start,
    Stop,
    Any(String),
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Event::Start => {
                write!(f,"Start")
            }
            Event::Stop => {
                write!(f, "Stopped")
            }
            Event::Any(name) => {
                write!(f, "{}", name)
            }
        }
    }
}

// A generic event loop that processes events
struct EventLoop<T> {
    event_sender: mpsc::Sender<Event>,
    event_receiver: Arc<Mutex<mpsc::Receiver<Event>>>,
    task_handle: Option<JoinHandle<()>>,
    result: Option<T>,
}

impl<T> EventLoop<T>
    where
        T: Send + 'static,
{
    // Create a new event loop
    fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel(100);

        EventLoop {
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
            task_handle: None,
            result: None,
        }
    }

    // Start the event loop
    fn start<F>(&mut self, mut event_handler: F)
        where
            F: 'static + FnMut(Event) -> T + Send,
    {
        let event_receiver = Arc::clone(&self.event_receiver);
        let task = async move {
            while let Some(event) = event_receiver.lock().unwrap().recv().await {
                event_handler(event);
            }
        };

        let task_handle = task::spawn_local(task);
        self.task_handle = Some(task_handle);
    }

    // Send an event to the event loop
    fn send_event(&mut self, event: Event) {
        let _ = self.event_sender.blocking_send(event);
    }

    // Stop the event loop
    async fn stop(&mut self) -> Option<T> {
        self.send_event(Event::Stop);

        if let Some(task_handle) = self.task_handle.take() {
            task_handle.await.ok()?;
            self.result.take()
        } else {
            None
        }
    }
}