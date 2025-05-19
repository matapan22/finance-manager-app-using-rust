use std::io;

pub mod incomes;
pub mod expenses;
pub mod transaction;
use rusqlite::Connection;

/// Struct for finance application variables
pub struct FinanceVariables {

    total_incomes: f64,  
    total_expenses: f64, 

}
/// Functions for general use
impl FinanceVariables {

    /// Setting starting values
    pub fn new() -> Self {

        FinanceVariables {

            total_incomes: 0.0,  
            total_expenses: 0.0, 

        }

    }

    /// Function for calculating total balance in the account
    pub fn total_budget( &self, connect: &Connection ) -> rusqlite::Result<f64> {

        // Get incomes part from the database
        let total_income: f64 = connect.query_row( "SELECT COALESCE(SUM(Income), 0) FROM Income", [], |row: &rusqlite::Row| row.get(0), )?;
        
        // Get expenses from the database
        let total_expenses: f64 = connect.query_row( "SELECT COALESCE(SUM(Expense), 0) FROM Expense", [], |row: &rusqlite::Row| row.get(0), )?;

        // Get total incomes from the Transaction part in the database
        let transaction_income: f64 = connect.query_row(
        "SELECT COALESCE(SUM(Amount), 0) FROM \"Transaction\" WHERE Transaction_status = 'Income'",
        [], |row| row.get( 0 ),)?;

        // Get total expenses from the Transaction part in the database
        let transaction_expenses: f64 = connect.query_row( "SELECT COALESCE(SUM(Amount), 0) FROM \"Transaction\" WHERE Transaction_status = 'Expense'",
        [], |row| row.get( 0 ),)?;

        // Calculate the total budget
        Ok( ( total_income + transaction_income ) - ( total_expenses + transaction_expenses ) )
    }
    
}

mod testing {
    // Allow us to use functions exist in this module 
    use super::*; 

    fn database_setup( ) -> rusqlite::Connection {

        let connection: Connection = rusqlite::Connection::open_in_memory( ).unwrap( );

        connection.execute(

            "CREATE TABLE Income ( Income_id INTEGER PRIMARY KEY, Income REAL, Date TEXT )",
            [],

        ).unwrap();

        connection.execute(

            "CREATE TABLE Expense ( Expense_id INTEGER PRIMARY KEY, Expense REAL, Date TEXT )",
            [],

        ).unwrap();

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
    /// Function for testing total_balance 
    fn test_total_balance() {

        // Using instance of FinanceVariable and also database
        let connection = database_setup( ); 
        let mut finance_instance = FinanceVariables::new(); 

        // Adding Income
        finance_instance.add_income( 50.0, &connection ).unwrap( ); 

        // Adding expense
        finance_instance.add_expenses( 50.0, &connection ).unwrap( ); 

        let total_budget = finance_instance.total_budget( &connection ).unwrap( );
        assert_eq!( total_budget, 0.0 ); 
    }

}