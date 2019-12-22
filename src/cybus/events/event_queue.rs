//

use super::common::Event;
use std::sync::mpsc;

pub fn make_event_queue<T>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    let (sender, receiver): (mpsc::Sender<T>, mpsc::Receiver<T>) = mpsc::channel();
    (sender, receiver)
}

// --------------------------------------------- Tests --------------------------------------------

#[cfg(test)]
mod test {
    #[test]
    fn send_receive() {
        use crate::events;
        use std::thread;

        let (event_sender, event_reciever) = events::make_event_queue();

        let key_press = events::KeyPressedEvent {
            handled: false,
            key_code: 5,
            repeat_count: 10,
        };

        thread::spawn(move || {
            event_sender.send(key_press).unwrap();
        });

        // Let's see what that answer was
        assert_eq!(
            "KeyPressedEvent Key Code: 5, Repeats: 10",
            event_reciever.recv().unwrap().to_string()
        );
    }
}
