use crate::semantic::center::sema::Semantic;

impl Semantic {
    pub fn analyze(&mut self){
        self.collect_control_flow();
        self.analyze_flow_context();
        if self.error.has_error{
            self.output_semantic_error();
        }
    }
}
