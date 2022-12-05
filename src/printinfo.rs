pub trait PrintInfo {
    fn print_info(&self);
}

pub fn static_dispatch(d: impl PrintInfo) {
    d.print_info();
}

pub fn dynamic_dispatch(d: Box<dyn PrintInfo>) {
    d.print_info();
}
