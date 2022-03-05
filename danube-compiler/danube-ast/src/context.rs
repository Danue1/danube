use danube_diagnostics::Message;

pub trait Context {
    fn report(&self, message: Message);
}
