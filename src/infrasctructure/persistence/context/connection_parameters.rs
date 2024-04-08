use secrecy::{ExposeSecret, Secret};

#[derive(Debug)]
pub struct ConnectionBuilder {
    host: String,
    port: u16,
    user: String,
    password: Secret<String>,
    db_name: String,
}

#[derive(Debug)]
pub struct ConnectionParameters {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: Secret<String>,
    pub(crate) db_name: String,
}

impl ConnectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_credentials(self, user: &str, password: Secret<String>) -> Self {
        Self {
            host: self.host,
            port: self.port,
            db_name: self.db_name,
            user: user.to_owned(),
            password,
        }
    }

    pub fn with_database(self, name: &str) -> Self {
        Self {
            db_name: name.to_owned(),
            host: self.host,
            port: self.port,
            user: self.user,
            password: self.password,
        }
    }

    pub fn with_host(self, name: &str, port: Option<u16>) -> Self {
        let port = port.unwrap_or(5432);
        Self {
            host: name.to_owned(),
            port,
            user: self.user,
            password: self.password,
            db_name: self.db_name,
        }
    }

    pub fn build(self) -> ConnectionParameters {
        ConnectionParameters {
            host: self.host,
            port: self.port,
            user: self.user,
            password: self.password,
            db_name: self.db_name,
        }
    }
}

impl Default for ConnectionBuilder {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 5432,
            user: String::default(),
            password: Secret::new(String::new()),
            db_name: String::default(),
        }
    }
}

impl ConnectionParameters {
    pub fn base_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }

    pub fn database_name(&self) -> String {
        self.db_name.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use secrecy::{ExposeSecret, Secret};

    use super::ConnectionBuilder;

    #[test]
    fn given_new_builder_then_default_host_and_port() {
        let p = ConnectionBuilder::new();

        assert_eq!(p.host, "127.0.0.1");
        assert_eq!(p.port, 5432);
    }

    #[test]
    fn given_new_builder_when_build_then_values_are_valid() {
        let p = ConnectionBuilder::new()
            .with_credentials("user", Secret::new("password".into()))
            .with_database("db")
            .with_host("name", Some(2345))
            .build();

        assert_eq!(p.host, "name");
        assert_eq!(p.port, 2345);
        assert_eq!(p.user, "user");
        assert_eq!(p.password.expose_secret(), "password");
        assert_eq!(p.database_name(), "db");
        assert_eq!(p.base_uri(), "postgresql://user:password@name:2345");
    }

    #[test]
    fn given_new_builder_when_build_with_host_default_port_is_5432() {
        let p = ConnectionBuilder::new()
            .with_credentials("user", Secret::new("password".into()))
            .with_database("db")
            .with_host("name", None)
            .build();

        assert_eq!(p.host, "name");
        assert_eq!(p.port, 5432);
    }
}
