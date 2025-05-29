#[cfg(not(feature = "italian"))]
pub(crate) mod lang {
    // Web controller startup errors.
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Built-in template internal failure";
    pub(crate) const LISTENER_ERROR: &str = "Listener creation failed";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Server startup failed";

    // Error route errors.
    pub(crate) const ERROR_TEMPLATE_ERROR: &str = "Error in retrieving the `error` template";
    pub(crate) const ERROR_RENDER_ERROR: &str = "Error in rendering the `error` template";

    // Index route errors.
    pub(crate) const INDEX_TEMPLATE_ERROR: &str = "Error in retrieving the `index` template";
    pub(crate) const INDEX_RENDER_ERROR: &str = "Error in rendering the `index` template";
    // Discovery route errors.
    pub(crate) const DISCOVERY_ERROR: &str = "Error in discovering devices";
    // Privacy route errors.
    pub(crate) const PRIVACY_TEMPLATE_ERROR: &str = "Error in retrieving the `policy` template";
    pub(crate) const PRIVACY_RENDER_ERROR: &str = "Error in rendering the `policy` template";
    // Stream route errors.
    pub(crate) const STREAM_TEMPLATE_ERROR: &str = "Error in retrieving the `stream` template";
    pub(crate) const STREAM_RENDER_ERROR: &str = "Error in rendering the `stream` template";

    // Navbar items.
    pub(crate) const INDEX_ITEM: &str = "Home";
    pub(crate) const PRIVACY_ITEM: &str = "Privacy";

    // Error page messages.
    pub(crate) const GOTO_DEVICES: &str = "Go to devices";
    pub(crate) const ASSETS_ERROR: &str = "Error in loading the `assets` directory";
    pub(crate) const MISSING_ROUTE: &str = "No route for";

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
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Errore di caricamento template";
    pub(crate) const LISTENER_ERROR: &str = "Creazione del listener fallita";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Fallimento nell'avvio del server";

    // Error route errors.
    pub(crate) const ERROR_TEMPLATE_ERROR: &str = "Errore nel trovare il template `error`";
    pub(crate) const ERROR_RENDER_ERROR: &str = "Errore nel renderizzare il template `error`";

    // Index route errors.
    pub(crate) const INDEX_TEMPLATE_ERROR: &str = "Errore nel trovare il template `index`";
    pub(crate) const INDEX_RENDER_ERROR: &str = "Errore nel renderizzare il template `index`";
    // Discovery route errors.
    pub(crate) const DISCOVERY_ERROR: &str = "Errore nell'individuare i dispositivi";
    // Privacy route errors.
    pub(crate) const PRIVACY_TEMPLATE_ERROR: &str = "Errore nel trovare il template `policy`";
    pub(crate) const PRIVACY_RENDER_ERROR: &str = "Errore nel renderizzare il template `policy`";
    // Stream route errors.
    pub(crate) const STREAM_TEMPLATE_ERROR: &str = "Errore nel trovare il template `stream`";
    pub(crate) const STREAM_RENDER_ERROR: &str = "Errore nel renderizzare il template `stream`";

    // Navbar items.
    pub(crate) const INDEX_ITEM: &str = "Dispositivi";
    pub(crate) const PRIVACY_ITEM: &str = "Privacy";

    // Error page messages.
    pub(crate) const GOTO_DEVICES: &str = "Torna ai dispositivi";
    pub(crate) const ASSETS_ERROR: &str = "Errore nel caricare la cartella `assets`";
    pub(crate) const MISSING_ROUTE: &str = "Nessun percorso chiamato";

    // Devices messages.
    pub(crate) const NO_DEVICES: &str = "Nessun dispositivo trovato.";
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
