use std::collections::HashMap;
use testcontainers::{core::WaitFor, Image};

const NAME: &str = "postgres";
const TAG: &str = "11-alpine";

#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());
        env_vars.insert("POSTGRES_PASSWORD".to_owned(), "test".to_owned());
        env_vars.insert("POSTGRES_USER".to_owned(), "test".to_owned());

        Self { env_vars }
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }
    fn expose_ports(&self) -> Vec<u16> {
        vec![5432]
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(std::iter::empty())
    }
}
