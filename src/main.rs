#![windows_subsystem = "windows"]
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

fn main() {
    let mut count: i8 = 0;
    loop {
        if count == 3 {
            Notification::new()
                .summary("Prend du temp pour tes yeux!")
                .body("Allez! Regarde dehors! \n Oublis pas de mettre des goutes aussi!")
                .show()
                .unwrap();
            count = 1;
        } else {
            Notification::new()
                .summary("Prend du temp pour tes yeux!")
                .body("On regarde dehors 20 secondes!")
                .show()
                .unwrap();
            count += 1;
        }
        thread::sleep(Duration::from_secs(1200));
    }
}
