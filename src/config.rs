struct Config {
    config_filename: String,
    hosts: [String],
    routes: [Routing]
}

struct Routing {
    path_matcher: String,
    destination_host: String,
}

pub impl Config {
    pub fn parse(config_filename = ".proxee.json") -> Self {
        Config {
            config_filename
        }
    }
}
