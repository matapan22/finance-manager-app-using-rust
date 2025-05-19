use rusqlite::Connection;
use crate::finance::FinanceVariables;
/// Implementation FinanceVariables struct for using expenses
/// Functions for expenses

impl super::FinanceVariables {

    /// Adds a new expense to the database 
    pub fn add_expenses( &mut self, expense: f64, connect: &Connection ) -> rusqlite::Result<()> {
        
        // Inserting new expense to expense table
        connect.execute(
            "INSERT INTO Expense (Expense, Date) VALUES (?1, datetime('now'))", 
            [ expense ]
        )?;

        println!( "New expense entrance: ${:.2}", expense );

        Ok(())

    }

    /// Function to show most recent last 10 expenses
    pub fn last_10expenses( &self, connect: &Connection ) -> rusqlite::Result<()> {

        // Preparing the SQL query to get last 10 expenses
        let mut last_expenses = connect.prepare( "SELECT Expense FROM Expense ORDER BY Expense_id DESC LIMIT 10", )?;
        
        // Executing query and getting map for each row f64
        let rows = last_expenses.query_map( [], |row| row.get::<_, f64>(0) )?;

        println!( "Last 10 expenses " );

        for expense in rows {
            match expense {

                Ok( amount ) => println!( "${:.2}", amount ),

                Err( er ) => {

                    println!( "Error while trying to get expenses: {}", er );
                    return Err( er );

                }
            }
        }

        Ok(())
    }

}

#[cfg( test )]
mod testing {
    // Allow us to use functions exist in this module 
    use super::*; 

    fn database_setup( ) -> rusqlite::Connection {

        let connection: Connection = rusqlite::Connection::open_in_memory( ).unwrap( );

        // Creating 
        connection.execute(

            "CREATE TABLE Expense ( Expense_id INTEGER PRIMARY KEY, Expense REAL, Date TEXT )",
            [],

        ).unwrap();

        connection
    }
    
    #[test]
    /// Function for testing expense
    fn test_expense() {

        // Using instance of FinanceVariable and also database
        let connection = database_setup(); 
        let mut finance_instance = FinanceVariables::new(); 

        // Adding expense
        finance_instance.add_expenses( 50.0, &connection ).unwrap( ); 

        // Checking if expense added correctly
        let counting: i32 = connection
           .query_row( 
            "SELECT COUNT(*) FROM Expense",
            [ ], | row | row.get( 0 ) ).unwrap( );

        // Checking result
        assert_eq!( counting, 1 ); 
    }

}