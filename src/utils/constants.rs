pub mod Requests {
    use crate::library::Song;

    #[derive(Debug, Copy, Clone)]
    pub enum UIStuff {
        SelectedSong,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum UIRequests {
        Up,
        Down,
        Quit,
        Enter,

        GoBack,

        ShowItemInfo,
        ShowSearch,
        SearchInput(char),

        UpdateBar,

        Query(UIStuff),
    }

    #[derive(Debug, Clone)]
    pub enum PlayerRequests {
        Stop,
        Start,
        Resume,
        Pause,
        Next,
        Previous,
        Seek(u64),
        ChangeVolume(f32),
        Quit,
    }

    #[derive(Debug, Clone)]
    pub enum AppRequests {
        UIRequests(UIRequests),
        PlayerRequests(PlayerRequests),
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

#[derive(Debug)]
pub enum PlayerStates {
    STOPPED,
    PLAYING,
    PAUSED,
}
