use crate::semantic::center::sema::Semantic;

impl Semantic {
    pub fn analyze(&mut self){
        self.collect_control_flow();
       // self.analyze_flow_context();
    }
}
