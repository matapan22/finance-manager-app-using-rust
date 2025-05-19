use rusqlite::Connection;
use crate::finance::FinanceVariables;
/// Implementation FinanceVariables struct for using transactions
/// Functions for transactions
impl super::FinanceVariables {

    /// Adds a new transaction to the database  
    pub fn transaction( &mut self, quantity: f64, time: &str, state: &str, class: &i32, connect: &Connection ) -> rusqlite::Result<()> {

        connect.execute( 

            r#"INSERT INTO "Transaction" (Amount, Date, Transaction_status, category_id ) VALUES ( ?1, ?2, ?3, ?4 )"#, 
            rusqlite::params! [ &quantity.to_string(), time, state, class ], 

        )?;

        Ok(())

    }

    /// Function to print last 10 transaction
    pub fn last_10transaction ( &self, connect: &Connection ) -> rusqlite::Result< Vec< ( f64, String, String, i32 ) > > {

        // Preparing the SQL query to get last 10 income
        let mut last_transactions = connect.prepare( 
            r#"SELECT Amount, Date, Transaction_status, category_id FROM "Transaction" ORDER BY Transaction_id DESC LIMIT 10"#,
        )?;
        
        // Saving values to rust tuple
        let rows = last_transactions.query_map( [], |row| {

            Ok( (

                row.get::<_, f64>( 0 )?,   
                row.get::<_, String>( 1 )?,     
                row.get::<_, String>( 2 )?,   
                row.get::<_, Option<i32>>( 3 )?.unwrap_or(0), // Handling null variables

            ) )

        } ) ?;
    
        // Creating empty vector to store database values
        let mut last10_transactios = Vec::new();

        // Printing each transactions
        for transactions in rows {

            match transactions {

                Ok ( data ) => last10_transactios.push( data ),
                Err ( err ) => return Err ( err ), 

            }

        }

        Ok ( last10_transactios )

    }

    /// Function to print all transactions
    pub fn all_transactions( &self, connect: &Connection ) -> rusqlite::Result<Vec<( f64, String, String, i32 )>> {
        
        // Preparing the SQL query to get all transactions
        let mut transactions = connect.prepare(
            r#"SELECT Amount, Date, Transaction_status, category_id FROM "Transaction" ORDER BY Transaction_id DESC"#,
        )?;
    
        // Saving values to rust tuple
        let row = transactions.query_map([], |row| {

            Ok( (

                row.get::<_, f64>(0)?,   
                row.get::<_, String>(1)?,     
                row.get::<_, String>(2)?,   
                row.get::<_, Option<i32>>(3)?.unwrap_or(0), // Handling NULL values

            ) )

        } ) ?;
    
        // Creating empty vector to store database values
        let mut transactions = Vec::new();
    
        // Printing each transactions
        for transaction in row {

            match transaction {

                Ok(data) => transactions.push( data ),
                Err(err) => return Err(err),

            }

        }
    
        Ok( transactions )

    }

    /// Function to print all transactions( Income )
    pub fn income_transactions( &self, connect: &Connection ) -> rusqlite::Result<Vec<(f64, String, String, i32)>> {
        // Prepare SQL query to get only Income transactions
        let mut incomes = connect.prepare(
            r#"SELECT Amount, Date, Transaction_status, category_id FROM "Transaction" WHERE Transaction_status = 'Income' ORDER BY Transaction_id DESC"#,
        )?;
    
        // Saving values to rust tuple
        let row = incomes.query_map( [], |row| {
            Ok( (

                row.get::<_, f64>( 0 )?,   
                row.get::<_, String>( 1 )?,     
                row.get::<_, String>( 2 )?,   
                row.get::<_, Option<i32>>( 3 )?.unwrap_or( 0 ), // Handling NULL values

            ) )
        } )?;
    
        // Creating empty vector to store database values
        let mut transactions_income = Vec::new();
    
        // Printing each transactions
        for all_incomes in row {

            match all_incomes {

                Ok( data ) => transactions_income.push( data ),
                Err(err) => return Err(err),

            }

        }
    
        Ok( transactions_income )

    }

    /// Function to print all transactions ( Expense )
    pub fn expense_transactions( &self, connect: &Connection ) -> rusqlite::Result<Vec<( f64, String, String, i32 )>> {

    // Prepare SQL query to get only Expense transactions
    let mut expenses = connect.prepare( 
        r#"SELECT Amount, Date, Transaction_status, category_id FROM "Transaction" WHERE Transaction_status = 'Expense' ORDER BY Transaction_id DESC"#,
    )?;

    // Saving values to rust tuple
    let row = expenses.query_map( [], |row| {

        Ok( (

            row.get::<_, f64>( 0 )?,   
            row.get::<_, String>( 1 )?,     
            row.get::<_, String>( 2 )?,   
            row.get::<_, Option<i32>>( 3 )?.unwrap_or( 0 ), // Handling NULL values

        ) )
    } )?;

    // Creating empty vector to store database values
    let mut transactions_expense = Vec::new();

    // Printing each transactions
    for all_expenses in row {

        match all_expenses {

            Ok( data ) => transactions_expense.push( data ),
            Err(err) => return Err(err),

        }
        
    }

        Ok( transactions_expense )

    }    

}

mod testing {
    use crate::finance::transaction;

    // Allow us to use functions exist in this module 
    use super::*; 

    fn database_setup( ) -> rusqlite::Connection {

        let connection: Connection = rusqlite::Connection::open_in_memory( ).unwrap( );

        connection.execute(

            "CREATE TABLE \"Transaction\" (
                Transaction_id INTEGER PRIMARY KEY,
                Amount REAL, 
                Date TEXT, 
                Transaction_status TEXT, 
                category_id INTEGER
            )",
            [],

        ).unwrap();

        connection
        
    }
    
    #[test]
    /// Function for testing transaction
    fn test_transactions() {

        // Using instance of FinanceVariable and also database
        let connection = database_setup( ); 
        let mut finance_instance = FinanceVariables::new(); 

        // Adding a transaction
        let transaction = finance_instance.transaction( 200.0, "2025-02-28", "Expense", &1, &connection );

        // Checking If transaction added
        assert!( transaction.is_ok( ), "Adding transaction is failed" );

        // Check if the transaction was added
        let check: i32 = connection
            .query_row(
            "SELECT COUNT(*) FROM \"Transaction\"",
            [], | row | row.get( 0 ) ).unwrap( );

        assert_eq!( check, 1, "Transaction is not in the database" );
     
    }

}