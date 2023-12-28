CREATE TABLE IF NOT EXISTS employees (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(50),
  last_name VARCHAR(50),
  hire_date DATE,
  salary DECIMAL(10, 2)
)
