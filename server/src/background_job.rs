#[derive(Clone, Debug)]
pub struct MyState {
    pub app_name: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MyJob {
    some_usize: usize,
    other_usize: usize,
}

impl MyState {
    pub fn new(app_name: &str) -> Self {
        MyState {
            app_name: app_name.to_owned(),
        }
    }
}