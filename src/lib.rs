pub mod infrastructure {
   pub mod bootstrap;

   pub mod adapters {
        pub mod inbound {
            pub mod dto;
            pub mod ssh;
            pub mod http_server;
        }

        pub mod outbound {
            pub mod dto;
            pub mod user;
        }
   }
}

pub mod middlewares {
    pub mod http_logging_middleware;
}

pub mod application {
    pub mod app;
    pub mod dto;
    pub mod helpers;
    pub mod parser;
    pub mod ports;
}

pub mod modules {
    pub mod dtos;
    pub mod frankfurter_module;
    pub mod freeweatherapi_module;
    pub mod openmeteo_module;
}

pub mod domain {
    pub mod exceptions;

    pub mod models {
        pub mod user;
    }
}
