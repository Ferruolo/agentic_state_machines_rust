use crate::agent_nodes::{Agent, AgentSignal};
use crate::llm::LlmInterface;
use std::rc::Rc;

struct LLMAgent<T> {
    key: i32,
    prompt: String,
    prompt_formatter: fn(&String, T) -> String,
    return_data: fn(String) -> crate::agent_nodes::AgentSignal<T>,
    llm: Rc<dyn LlmInterface>,
    max_retries: usize,
}

impl<T> LLMAgent<T> {
    fn format_data(&self, data: T) -> String {
        self.prompt_formatter(&self.prompt, data)
    }


}

impl <T> Agent<T> for LLMAgent<T> {
    fn execute(&self, input: T) -> AgentSignal<T> {
        let llm_call = self.format_data(input);
        let mut timeout = 2;
        let retries = 0;
        let result = loop {
            let llm_data = llm_call.clone();
            let llm_result = self.llm.make_call(llm_data);
            match llm_result {
                Ok(x) => {

                }
                Err(err) => {

                }
            }
        };

    }
}