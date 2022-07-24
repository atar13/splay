pub mod Requests {
    pub enum UIRequests {
        Up,
        Down,
        Quit,
        Enter,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum AppRequests {
        UIUp,
        UIDown,
        UIEnter,
        Quit,
    }
}
