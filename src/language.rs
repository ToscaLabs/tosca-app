#[cfg(not(feature = "italian"))]
pub(crate) mod lang {
    // Web controller startup errors.
    pub(crate) const LOADING_TEMPLATE_ERROR: &str = "Built-in template internal failure";
    pub(crate) const LISTENER_ERROR: &str = "Listener creation failed";
    pub(crate) const SERVER_STARTUP_ERROR: &str = "Server startup failed";

    // Routes.
    pub(crate) const INFO_ROUTE: &str = "/view-info/{device_id}";
    pub(crate) const STREAM_ROUTE: &str = "/view-stream/{device_id}";
    pub(crate) const EVENT_ROUTE: &str = "/event-log/{device_id}";
    pub(crate) const RESPONSE_ROUTE: &str = "/response-log/{device_id}";
    pub(crate) const DISCOVERY_ROUTE: &str = "/discovery";
    pub(crate) const REQUEST_ROUTE: &str = "/request";

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
    // Info route errors.
    pub(crate) const INFO_TEMPLATE_ERROR: &str = "Error in retrieving the `info` template";
    pub(crate) const INFO_RENDER_ERROR: &str = "Error in rendering the `info` template";
    // Response log route errors.
    pub(crate) const RESPONSE_TEMPLATE_ERROR: &str =
        "Error in retrieving the `response-log` template";
    pub(crate) const RESPONSE_RENDER_ERROR: &str = "Error in rendering the `response-log` template";
    // Event log route errors.
    pub(crate) const EVENT_TEMPLATE_ERROR: &str = "Error in retrieving the `event-log` template";
    pub(crate) const EVENT_RENDER_ERROR: &str = "Error in rendering the `event-log` template";

    // Navbar items.
    pub(crate) const INDEX_ITEM: &str = "Home";
    pub(crate) const PRIVACY_ITEM: &str = "Privacy";

    // Request page messages.
    pub(crate) const U8_ERROR: &str = "Error in parsing the `u8` parameter value";
    pub(crate) const U16_ERROR: &str = "Error in parsing the `u16` parameter value";
    pub(crate) const U32_ERROR: &str = "Error in parsing the `u32` parameter value";
    pub(crate) const U64_ERROR: &str = "Error in parsing the `u64` parameter value";
    pub(crate) const F32_ERROR: &str = "Error in parsing the `f32` parameter value";
    pub(crate) const F64_ERROR: &str = "Error in parsing the `f64` parameter value";

    pub(crate) const REQUEST_DEVICE_ERROR: &str = "Error in finding the device";
    pub(crate) const REQUEST_SENDER_ERROR: &str = "Error in creating the request for the device";
    pub(crate) const REQUEST_SENDER_DEFAULT_PARAMS_ERROR: &str =
        "Error in sending the request with default parameters";
    pub(crate) const REQUEST_SENDER_PARAMS_ERROR: &str =
        "Error in sending the request with input parameters";

    pub(crate) const RESPONSE_OK_ERROR: &str = "Error in parsing the `Ok` response";
    pub(crate) const RESPONSE_SERIAL_ERROR: &str = "Error in parsing the `Serial` response";
    pub(crate) const RESPONSE_WRONG_STREAM_ERROR: &str =
        "This is a `Stream` response, something went really wrong.";

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

    // Routes.
    pub(crate) const INFO_ROUTE: &str = "/visualizza-informazioni/{device_id}";
    pub(crate) const STREAM_ROUTE: &str = "/visualizza-stream/{device_id}";
    pub(crate) const EVENT_ROUTE: &str = "/registro-eventi/{device_id}";
    pub(crate) const RESPONSE_ROUTE: &str = "/registro-risposte/{device_id}";
    pub(crate) const DISCOVERY_ROUTE: &str = "/scopri-device";
    pub(crate) const REQUEST_ROUTE: &str = "/richiesta";

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
    // Info route errors.
    pub(crate) const INFO_TEMPLATE_ERROR: &str = "Errore nel trovare il template `info`";
    pub(crate) const INFO_RENDER_ERROR: &str = "Errore nel renderizzare il template `info`";
    // Response log route errors.
    pub(crate) const RESPONSE_TEMPLATE_ERROR: &str =
        "Errore nel trovare il template `response-log`";
    pub(crate) const RESPONSE_RENDER_ERROR: &str =
        "Errore nel renderizzare il template `response-log`";
    // Event log route errors.
    pub(crate) const EVENT_TEMPLATE_ERROR: &str = "Errore nel trovare il template `event-log`";
    pub(crate) const EVENT_RENDER_ERROR: &str = "Errore nel renderizzare il template `event-log`";

    // Navbar items.
    pub(crate) const INDEX_ITEM: &str = "Dispositivi";
    pub(crate) const PRIVACY_ITEM: &str = "Privacy";

    // Request page messages.
    pub(crate) const U8_ERROR: &str = "Errore durante l'analisi del parametro di tipo `u8`";
    pub(crate) const U16_ERROR: &str = "Errore durante l'analisi del parametro di tipo `u16`";
    pub(crate) const U32_ERROR: &str = "Errore durante l'analisi del parametro di tipo `u32`";
    pub(crate) const U64_ERROR: &str = "Errore durante l'analisi del parametro di tipo `u64`";
    pub(crate) const F32_ERROR: &str = "Errore durante l'analisi del parametro di tipo `f32`";
    pub(crate) const F64_ERROR: &str = "Errore durante l'analisi del parametro di tipo `f64`";

    pub(crate) const REQUEST_DEVICE_ERROR: &str = "Errore nell'individuare il device";
    pub(crate) const REQUEST_SENDER_ERROR: &str = "Errore nel creare la richiesta per il device";
    pub(crate) const REQUEST_SENDER_DEFAULT_PARAMS_ERROR: &str =
        "Errore nell'inviare la richiesta con parametri di default";
    pub(crate) const REQUEST_SENDER_PARAMS_ERROR: &str =
        "Errore nell'inviare la richiesta con parametri";

    pub(crate) const RESPONSE_OK_ERROR: &str = "Errore durante l'analisi della risposta `Ok`";
    pub(crate) const RESPONSE_SERIAL_ERROR: &str =
        "Errore durante l'analisi della risposta `Serial`";
    pub(crate) const RESPONSE_WRONG_STREAM_ERROR: &str =
        "Questa è una risposta di tipo `Stream`, qualcosa non è andato per il verso giusto.";

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
