use crate::agent_nodes::{Agent, AgentSignal};
use crate::llm::LlmInterface;
use std::error::Error;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;

struct LLMAgent<T> {
    name: String,
    prompt: String,
    prompt_formatter: fn(&String, T) -> String,
    return_data: fn(String) -> Result<AgentSignal<T>, dyn Error>,
    llm: Rc<dyn LlmInterface>,
    max_retries: usize,
}

impl<T> LLMAgent<T> {
    fn format_data(&self, data: T) -> String {
        self.prompt_formatter(&self.prompt, data)
    }
}

impl<T> Agent<T> for LLMAgent<T> {
    fn execute(&self, input: T) -> AgentSignal<T> {
        let llm_call = self.format_data(input);
        let mut timeout: u64 = 2;
        let mut retries = 0;
        loop {
            let llm_data = llm_call.clone();
            let llm_result = self.llm.make_call(llm_data);
            let result: Option<AgentSignal<T>> = match llm_result {
                Ok(x) => {
                    match self.return_data(x) {
                        Ok(x) => x,
                        Err(_) => { None }
                    }
                }
                Err(_) => { None }
            };

            match result {
                // Failure
                None => {
                    if (retries < self.max_retries) {
                        sleep(Duration::from_secs(self.max_retries));
                        timeout = timeout * timeout;
                        retries += 1;
                    } else {
                        panic!("Agent {} failed after {} retries", self.name, self.max_retries);
                    }
                }
                Some(x) => {
                    // Success
                    break x;
                }
            }
        }
    }
}