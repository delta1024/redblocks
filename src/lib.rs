use std::fmt::Display;
use std::time::{Duration, Instant};

pub type StatusBar = Vec<Widget>;
pub struct Bar(pub StatusBar);

pub struct Widget {
    pub content: Box<dyn Update>,
    pub intervel: Duration,
    pub elapsed: Instant,
}

impl Widget {
    pub fn new(content: Box<dyn Update>, intervel: u64) -> Widget {
        Widget {
            content,
            intervel: Duration::from_secs(intervel),
            elapsed: Instant::now(),
        }
    }
    pub fn update(&mut self) {
        if self.elapsed.elapsed() >= self.intervel {
            self.content.refresh();

            self.elapsed = Instant::now();
        }
    }
}

pub trait Update: Display {
    fn refresh(&mut self);
}

#[macro_export]
macro_rules! start_bar  {
    ($($v:ident), +) => {

        use std::process::Command;
	use redblocks::Bar;
        let mut bar = Bar(Vec::new());

	$(
	    let var = $v;
	    bar.0.push(var);
        )+

        let mut comm = Command::new("xsetroot");
        loop {
            let mut output = String::new();

            for i in &mut bar.0 {
                i.update();
                let pushing = format!("| {} ", i.content);
                output.push_str(&pushing);
            }
            comm.args(&["-name", &output])
                .spawn()
                .expect("xset root not installed");
    }
    }
}
