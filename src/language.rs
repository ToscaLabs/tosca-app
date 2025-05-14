#[cfg(not(feature = "italian"))]
pub(crate) mod lang {
    // Errors.
    pub(crate) const SUBSCRIBER_ERROR: &str = "Subscriber initialization failed";
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Built-in template internal failure";
    pub(crate) const LISTENER_ERROR: &str = "Listener creation failed";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Server startup failed";

    // Informative messagges.
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str = "Web controller reachable at this address";
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Starting web app...";
}

#[cfg(feature = "italian")]
pub(crate) mod lang {
    // Errors.
    pub(crate) const SUBSCRIBER_ERROR: &str = "Inizializzazione subscriber fallita";
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Errore caricamento template";
    pub(crate) const LISTENER_ERROR: &str = "Creazione listener fallita";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Avvio server fallito";

    // Informative messagges.
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str =
        "Web controller accedibile a questo indirizzo";
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Avvio applicazione web...";
}
