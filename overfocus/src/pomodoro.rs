// 25 min work
// 5 min break
// Repeat 3 times
// 30 min break

use std::{sync::{Mutex, Arc, MutexGuard}, thread, time::Duration};

enum PomodoroStage {
    Work, ShortBreak, LongBreak
}

pub struct Pomodoro {
    stage: PomodoroStage,
    repetitions: usize,
    pomodoros: usize,
}

pub type PomodoroHandle = Arc<Mutex<Pomodoro>>;

impl Pomodoro {
    pub fn start() -> PomodoroHandle {
        let pomodoro = Arc::new(Mutex::new(Self {
            stage: PomodoroStage::Work,
            repetitions: 0,
            pomodoros: 0,
        }));

        let thread_pomodoro = pomodoro.clone();
        thread::spawn(|| Self::tick(thread_pomodoro));

        pomodoro
    }

    fn tick(data: PomodoroHandle) {
        loop {
            Self::lock_and(&data, |mut data| {
                data.stage = PomodoroStage::Work;
                data.repetitions = 0;
            });

            for _ in 0..3 {
                thread::sleep(Duration::from_secs(25 * 60)); // Wait 25 minutes
                Self::lock_and(&data, |mut data| data.stage = PomodoroStage::ShortBreak);

                thread::sleep(Duration::from_secs(5 * 60)); // Wait 5 minutes
                Self::lock_and(&data, |mut data| {
                    data.stage = PomodoroStage::Work;
                    data.repetitions += 1;
                });
            }
            Self::lock_and(&data, |mut data| data.stage = PomodoroStage::LongBreak);

            thread::sleep(Duration::from_secs(30 * 60)); // Wait 30 minutes
            Self::lock_and(&data, |mut data| data.pomodoros += 1);
        }
    }

    fn lock_and(data: &PomodoroHandle, func: impl FnOnce(MutexGuard<Pomodoro>)) {
        let locked = data.lock().unwrap();
        func(locked);
    }    
}