use zero2prod::run;

#[tokio::main]
fn main() -> std::io::Result<()> {
    run()?.await
}
