#[cfg(not(feature = "italian"))]
pub(crate) mod lang {
    // Errors.
    #[cfg(feature = "logging")]
    pub(crate) const LOG_ERROR_FILE_ERROR: &str = "Creation of log error failed";
    #[cfg(feature = "logging")]
    pub(crate) const LOG_DEBUG_FILE_ERROR: &str = "Creation of log debug failed";
    #[cfg(feature = "logging")]
    pub(crate) const SUBSCRIBER_ERROR: &str = "Subscriber initialization failed";
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Built-in template internal failure";
    pub(crate) const LISTENER_ERROR: &str = "Listener creation failed";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Server startup failed";

    // Informative messagges.
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str = "Web controller reachable at this address";
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Starting web app...";
}

#[cfg(feature = "italian")]
pub(crate) mod lang {
    // Errors.
    #[cfg(feature = "logging")]
    pub(crate) const LOG_ERROR_FILE_ERROR: &str = "Creazione del log degli errori fallita";
    #[cfg(feature = "logging")]
    pub(crate) const LOG_DEBUG_FILE_ERROR: &str = "Creazione del log di debug fallita";
    #[cfg(feature = "logging")]
    pub(crate) const SUBSCRIBER_ERROR: &str = "Inizializzazione subscriber fallita";
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Errore caricamento template";
    pub(crate) const LISTENER_ERROR: &str = "Creazione listener fallita";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Avvio server fallito";

    // Informative messagges.
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str =
        "Web controller accedibile a questo indirizzo";
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Avvio applicazione web...";
}
