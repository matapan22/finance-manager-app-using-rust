use rusqlite::Connection;
use crate::finance::FinanceVariables;

/// Implementation FinanceVariables struct for using incomes
/// Functions for incomes
impl super::FinanceVariables {

    /// Adds a new income to the database 
    pub fn add_income( &mut self, income: f64, connect: &Connection ) -> rusqlite::Result<()> {
        
        // Inserting new income to incomes table
        connect.execute( 
            "INSERT INTO Income (Income, Date) VALUES (?1, datetime('now'))",
            [ income ] 
        )?;

        println!( "New income entrance: ${:.2}", income );

        Ok(())
        
    }

    /// Function to entered most recent last 10 income
    pub fn last_10incomes ( &self, connect: &Connection ) -> rusqlite::Result<()> {

        // Preparing the SQL query to get last 10 income
        let mut last_incomes = connect.prepare( "SELECT Income FROM Income ORDER BY Income_id DESC LIMIT 10", )?;
        
        // Executing query and getting map for each row f64
        let rows = last_incomes.query_map( [], |row| row.get::<_, f64>(0) )?;
    
        println!("Last 10 incomes ");

        for income in rows {
            match income {

                Ok( amount ) => println!( "${:.2}", amount ),

                Err( er ) => {

                    println!("Error while trying to get incomes: {}", er);
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

        connection.execute(

            "CREATE TABLE Income ( Income_id INTEGER PRIMARY KEY, Income REAL, Date TEXT )",
            [],

        ).unwrap();

        connection
    }
    
    #[test]
    /// Function for testing expense
    fn test_income() {

        // Using instance of FinanceVariable and also database
        let connection = database_setup(); 
        let mut finance_instance = FinanceVariables::new(); 

        // Adding Income
        finance_instance.add_income( 50.0, &connection ).unwrap( ); 

        // Checking if Income added correctly
        let counting: i32 = connection
           .query_row( 
            "SELECT COUNT(*) FROM Income",
            [ ], | row | row.get( 0 ) ).unwrap( );

        // Checking result
        assert_eq!( counting, 1 ); 
    }

}