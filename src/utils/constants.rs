pub mod Requests {
    use crate::library::Song;

    #[derive(Debug, Copy, Clone)]
    pub enum UIRequests {
        Up,
        Down,
        Quit,
        Enter,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum PlayerRequests<'a> {
        Stop,
        Start(&'a str),
        Resume,
        Pause,
        Next,
        Previous,
        Seek(u64),
        ChangeVolume(f32),
    }

    #[derive(Debug, Copy, Clone)]
    pub enum AppRequests<'a> {
        UIRequests(UIRequests),
        PlayerRequests(PlayerRequests<'a>),
        Quit,
    }
}

pub mod Errors {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AudioDeviceError {
        #[error("No audio device found")]
        AudioDeviceNotFound,
    }
}
