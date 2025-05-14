#[cfg(not(feature = "italian"))]
pub(crate) mod lang {
    // Web controller startup errors.
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Built-in template internal failure";
    pub(crate) const LISTENER_ERROR: &str = "Listener creation failed";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Server startup failed";

    // Route errors.
    pub(crate) const INDEX_TEMPLATE_ERROR: &str = "Error in retrieving the `index` template";
    pub(crate) const INDEX_RENDER_ERROR: &str = "Error in rendering the `index` template";
    pub(crate) const POLICY_TEMPLATE_ERROR: &str = "Error in retrieving the `policy` template";
    pub(crate) const POLICY_RENDER_ERROR: &str = "Error in rendering the `policy` template";

    // Route page titles.
    pub(crate) const POLICY_TITLE: &str = "Policies";
    pub(crate) const INDEX_TITLE: &str = "Devices";

    // Devices messages.
    pub(crate) const NO_DEVICES: &str = "No devices found.";
    pub(crate) const DISCOVER_DEVICES: &str = "Discover Devices";

    // Logging errors.
    #[cfg(feature = "logging")]
    pub(crate) const LOG_ERROR_FILE_ERROR: &str = "Creation of log error failed";
    #[cfg(feature = "logging")]
    pub(crate) const LOG_DEBUG_FILE_ERROR: &str = "Creation of log debug failed";
    #[cfg(feature = "logging")]
    pub(crate) const SUBSCRIBER_ERROR: &str = "Subscriber initialization failed";
    #[cfg(feature = "logging")]
    pub(crate) const REQUEST_ERROR: &str = "Request error";

    // Informative messages.
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str = "Web controller reachable at this address";
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Starting web app...";
}

#[cfg(feature = "italian")]
pub(crate) mod lang {
    // Web controller startup errors.
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Errore caricamento template";
    pub(crate) const LISTENER_ERROR: &str = "Creazione listener fallita";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Avvio server fallito";

    // Route errors.
    pub(crate) const INDEX_TEMPLATE_ERROR: &str = "Errore nel trovare il template `index`";
    pub(crate) const INDEX_RENDER_ERROR: &str = "Errore nel renderizzare il template `index`";
    pub(crate) const POLICY_TEMPLATE_ERROR: &str = "Errore nel trovare il template `policy`";
    pub(crate) const POLICY_RENDER_ERROR: &str = "Errore nel renderizzare il template `policy`";

    // Route page names.
    pub(crate) const POLICY_TITLE: &str = "Politiche";
    pub(crate) const INDEX_TITLE: &str = "Dispositivi";

    // Devices messages.
    pub(crate) const NO_DEVICES: &str = "Nessun dispositivo è stato trovato.";
    pub(crate) const DISCOVER_DEVICES: &str = "Ricerca Dispositivi";

    // Logging errors.
    #[cfg(feature = "logging")]
    pub(crate) const LOG_ERROR_FILE_ERROR: &str = "Creazione del log di errore fallita";
    #[cfg(feature = "logging")]
    pub(crate) const LOG_DEBUG_FILE_ERROR: &str = "Creazione del log di debug fallita";
    #[cfg(feature = "logging")]
    pub(crate) const SUBSCRIBER_ERROR: &str = "Inizializzazione subscriber fallita";
    #[cfg(feature = "logging")]
    pub(crate) const REQUEST_ERROR: &str = "Errore nella richiesta";

    // Informative messages.
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_ADDRESS_MESSAGE: &str =
        "Applicazione visualizzabile a questo indirizzo";
    #[cfg(feature = "logging")]
    pub(crate) const CONTROLLER_STARTUP_MESSAGE: &str = "Avvio applicazione...";
}
