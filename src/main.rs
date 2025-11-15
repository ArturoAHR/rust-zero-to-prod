use rust_zero_to_prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
