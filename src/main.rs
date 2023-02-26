use todo_app::deploy;

fn main() {
    match deploy() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("ERROR: {e}");
        }
    }
}
