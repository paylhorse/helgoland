use godot::prelude::*;
use godot::engine::{ICodeEdit, CodeEdit};
use jlrs::prelude::*;
use jlrs::memory::stack_frame::StackFrame;

// TODO: implement ans

#[derive(GodotClass)]
#[class(base = CodeEdit)]
pub struct Blackboard {
    #[base]
    pub node: Base<CodeEdit>,
    pub julia: Option<PendingJulia>,
}

#[godot_api]
impl ICodeEdit for Blackboard {
    fn init(node: Base<CodeEdit>) -> Self {
        Blackboard {
            node,
            julia: None
        }
    }

    fn ready(&mut self) {
        self.julia = unsafe { Some(RuntimeBuilder::new().start().unwrap()) };
    }

    fn process(&mut self, delta: f64) {}
}

#[godot_api]
impl Blackboard {
    pub fn execute_julia(&mut self, string: String) {
        let mut frame = StackFrame::new();
        let julia = self.julia.as_mut().unwrap();
        let mut julia = julia.instance(&mut frame);
        let result = julia.scope(|mut frame| {
            let output = unsafe { Value::eval_string(&mut frame, string).unwrap() };
            let print_func = Module::base(&frame).function(&mut frame, "string")?;
            let string_result = unsafe { print_func.call1(&mut frame, output).into_jlrs_result()? };
            string_result.unbox::<String>()
        }).expect("Julia's cocked up!");

        self.node.set_text(format!("Julia output: {}", result.unwrap()).into());
    }
}
