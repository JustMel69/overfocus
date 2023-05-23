// 25 min work
// 5 min break
// Repeat 3 times
// 30 min break

use std::{sync::{Mutex, Arc, MutexGuard}, thread, time::Duration};

use anyhow::{Result, Ok};
use thiserror::Error;

pub enum PomodoroStage {
    Work, ShortBreak, LongBreak
}

#[derive(Clone, Copy)]
enum UserInputFlags {
    None,
    Pause,
    Stop,
}

/// Main struct that allows the execution of a Pomodoro clock.<br>
/// A pomodoro clock consists of the following:<br>
/// - 3 cycles of 25 minutes of work and 5 minute breaks
/// - A big 30 minute break
pub struct Pomodoro {
    stage: PomodoroStage,
    repetitions: usize,
    pomodoros: usize,
    seconds: usize,

    input_flags: UserInputFlags,
}

#[derive(Error, Debug)]
pub enum PomodoroError {
    #[error("The Pomodoro thread was poisoned!")]
    PoisonedThread
}

pub type PomodoroHandle = Arc<Mutex<Pomodoro>>;

impl Pomodoro {
    // · · ·  Main Thread Functions  · · · //
    
    /// Starts a new pomodoro clock with its own thread
    pub fn start() -> PomodoroHandle {
        let pomodoro = Arc::new(Mutex::new(Self {
            stage: PomodoroStage::Work,
            repetitions: 0,
            pomodoros: 0,
            seconds: 0,
            input_flags: UserInputFlags::None,
        }));

        let thread_pomodoro = pomodoro.clone();
        thread::spawn(|| Self::tick(thread_pomodoro).unwrap()); // TODO: Actually send error to some logging system

        pomodoro
    }

    /// Pauses the pomodoro progression, but the thread remains
    pub fn pause(data: &PomodoroHandle) -> Result<()> {
        Self::lock_and(data, |mut x| {
            x.input_flags = UserInputFlags::Pause;
        })
    }

    /// Halts the pomodoro thread
    pub fn stop(data: &PomodoroHandle) -> Result<()> {
        Self::lock_and(data, |mut x| {
            x.input_flags = UserInputFlags::Stop;
        })
    }

    /// Locks a handle and allows to do something with it
    pub fn lock_and<T>(data: &PomodoroHandle, func: impl FnOnce(MutexGuard<Pomodoro>) -> T) -> Result<T> {
        let locked = data.lock().map_err(|_| PomodoroError::PoisonedThread)?;
        Ok(func(locked))
    }

    pub fn stage(&self) -> &PomodoroStage {
        &self.stage
    }

    pub fn repetitions(&self) -> usize {
        self.repetitions
    }

    pub fn pomodoros(&self) -> usize {
        self.pomodoros
    }

    pub fn seconds(&self) -> usize {
        self.seconds
    }



    // · · ·  Pomodoro Thread Functions  · · · //

    /// **NOT MAIN THREAD**<br>
    /// Executes the pomodoro routine
    fn tick(data: PomodoroHandle) -> Result<()> {
        loop {
            thread::sleep(Duration::from_secs(1));
            
            // Guard clause for user input
            match Self::lock_and(&data, |x| x.input_flags)? {
                UserInputFlags::Pause => continue,
                UserInputFlags::Stop => return Ok(()),
                _ => {}
            }
            
            // Actually do things
            Self::lock_and(&data, |mut data| {
                match data.stage {
                    PomodoroStage::Work => Self::handle_work(&mut data),
                    PomodoroStage::ShortBreak => Self::handle_short_break(&mut data),
                    PomodoroStage::LongBreak => Self::handle_long_break(&mut data),
                }

                data.seconds += 1;
            })?;
        }
    }

    fn handle_work(data: &mut MutexGuard<Pomodoro>) {
        if data.seconds < 25 * 60 {
            return;
        }
        
        if data.repetitions == 2 {
            data.start_long_break()
        } else {
            data.start_short_break()
        }
    }

    fn handle_short_break(data: &mut MutexGuard<Pomodoro>) {
        if data.seconds >= 5 * 60 {
            data.start_work();
            data.repetitions += 1;
        }
    }

    fn handle_long_break(data: &mut MutexGuard<Pomodoro>) {
        if data.seconds >= 30 * 60 {
            data.start_work();
            data.pomodoros += 1;
        }
    }

    fn start_work(&mut self) {
        self.seconds = 0;
        self.stage = PomodoroStage::Work;
    }

    fn start_short_break(&mut self) {
        self.seconds = 0;
        self.stage = PomodoroStage::ShortBreak;
    }

    fn start_long_break(&mut self) {
        self.seconds = 0;
        self.stage = PomodoroStage::LongBreak;
    }
}