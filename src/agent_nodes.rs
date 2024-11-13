

pub (crate) enum AgentSignal<T> {
    Terminate(String),
    Success(T),
    Failure(T),
    RedirectTo(i32, T)
}


pub(crate) trait Agent <T> {
    fn execute(&self, input: T) -> AgentSignal<T>;
}

pub(crate) struct AgenticStructure<T> {
    agents: Vec<dyn Agent<T>>,
    entry_point: usize,
}


impl <T> AgenticStructure<T> {
    pub fn run_agentic(&self, input: T) -> T {
        let mut selected = &self.agents[self.entry_point];
        let mut current_data = input;
        loop {
            match selected.execute(current_data) {
                AgentSignal::Terminate(error_message) => {
                    panic!("{}", error_message);
                }
                AgentSignal::Success(data) => {
                    return data
                }
                AgentSignal::Failure(data) => {
                    current_data = data
                }
                AgentSignal::RedirectTo(idx, data) => {
                    selected = &self.agents[idx];
                    current_data = data
                }
            }
        }
    }
}