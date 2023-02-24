#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenvy::dotenv().unwrap();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&dotenvy::var("DATABASE_URL").unwrap()).await?;

    let data: Vec<u8> = vec![1;100_000_000];
    println!("test data size: {} MB", std::mem::size_of_val(data.as_slice()) as f32 / 1_000_000.);

    let row = sqlx::query!("INSERT INTO test_table VALUES ($1)", data)
		.execute(&pool)
		.await?;
    println!("{row:#?}");

    drop(data);

    println!("press ctrl+c to delete data in the table and close the pool");
    tokio::signal::ctrl_c().await?;

    let row = sqlx::query!("DELETE FROM test_table")
        .execute(&pool)
        .await?;
    println!("\n{row:#?}");

    pool.close().await;

    println!("press ctrl+c to stop the program");
    tokio::signal::ctrl_c().await?;
    Ok(())
}
