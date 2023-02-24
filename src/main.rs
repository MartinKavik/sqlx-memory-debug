#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenvy::var("DATABASE_URL").unwrap()).await?;

    let data = b"a new ticket";

    let row = sqlx::query!("INSERT INTO test_table VALUES ($1)", data)
		.execute(&pool)
		.await?;

    println!("{row:#?}");

    println!("press ctrl+c to stop the program");
    tokio::signal::ctrl_c().await?;
    println!("\nctrl-c received!");
    Ok(())
}
