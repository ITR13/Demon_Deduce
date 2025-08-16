use demon_deduce::{run_args, run_clipboard_loop, run_from_clipboard};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"-c".to_string()) {
        run_from_clipboard();
        return;
    }

    if args.contains(&"-l".to_string()) {
        run_clipboard_loop();
        return;
    }

    run_args(args);
}
