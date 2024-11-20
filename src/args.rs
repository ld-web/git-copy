use std::env;

#[derive(Debug)]
pub struct Args {
    pub start_commit: String,
    pub end_commit: String,
    pub dest_dir: String,
    pub ignore_exts: Option<Vec<String>>,
}

impl Args {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 4 {
            eprintln!(
                "Usage: {} <start_commit> <end_commit> <dest_dir> [ignore_ext]",
                args[0]
            );
            std::process::exit(1);
        }

        Self {
            start_commit: args[1].clone(),
            end_commit: args[2].clone(),
            dest_dir: args[3].clone(),
            ignore_exts: args
                .get(4)
                .cloned()
                .map(|ext| ext.split(',').map(String::from).collect()),
        }
    }
}
