pub struct Client {
    name: String,
    cpf: String,
}

impl Client {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, new_name: String) -> &String {
        self.name = new_name;
        &self.name
    }

    pub fn set_cpf(&mut self, new_cpf: String) -> &String {
        self.cpf = new_cpf;
        &self.name
    }

    pub fn get_cpf(&self) -> &String {
        &self.cpf
    }

    pub fn create_client(name: String, cpf: String) -> Client {
        Client {
            name,
            cpf
        }
    }
}