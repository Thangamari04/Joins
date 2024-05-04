use std::error::Error;
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://postgres:postgres@localhost:5432/joins";
    let pool = PgPoolOptions::new()
        .connect(url)
        .await?;

    // Your SQL query with JOIN should be inside the main function
    let query = "
        SELECT employees.employee_id, employees.employee_name, departments.department_name
        FROM employees
        INNER JOIN departments ON employees.department_id = departments.department_id
    ";

    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let employee_id: i32 = row.get("employee_id");
        let employee_name: String = row.get("employee_name");
        let department_name: String = row.get("department_name");

        println!("Employee ID: {}, Name: {}, Department: {}", employee_id, employee_name, department_name);
    }

    Ok(())
}
