use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder, Button, TreeView, TreeViewColumn, ListStore,};
use std::io;
use chrono::Local; // For getting the current date automatically
use crate:: finance; // Your existing module
use finance::FinanceVariables;
use rusqlite::Connection;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;




//treeview for custom tree view
fn setup_treeview(treeview: &gtk4::TreeView) {
    let column_titles = ["Transaction Type", "Amount", "Date", "Category"];
    let column_widths = [200, 100, 120, 150]; // Setting custom widths for each column

    //Removing old coloums to prevent duplicates
    for col in treeview.columns() {
        treeview.remove_column(&col);
    }

    for (i, title) in column_titles.iter().enumerate() {
        let renderer = gtk4::CellRendererText::new();
        let column = gtk4::TreeViewColumn::new();

        column.set_title(title);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, "text", i as i32);

        // Set column width
        column.set_fixed_width(column_widths[i]);

        // Add the column to the TreeView
        treeview.append_column(&column);
    }
}






//load UI file
const TRANSACTION_VIEW_FILE: &str = "src/resources/viewtransactions.ui";//UI file location
pub struct NewWindow {
    pub window: ApplicationWindow,
}

impl NewWindow {
    
    pub fn new_from_builder(builder: &Builder) -> Self {
        let builder = Builder::from_file(TRANSACTION_VIEW_FILE);
        // Load the new window from the builder
        let window: ApplicationWindow = builder
            .object("viewtransactions")
            .expect("Failed to load 'Transaction window' from UI file");
        let db_path = "Finance_app.db"; 
    //connect with the database
    let conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(err) => {
            eprintln!("Error opening database: {}", err);
            return Self { window }; 
        }
    };


     // Initialize FinanceVariables
     let mut finance = finance::FinanceVariables::new();
    

    // Load TreeView from UI file for viewing all transaction
    let viewtransactions_treeview: TreeView = builder
    .object("viewtransactions_treeview")
    .expect("Failed to find 'transactions_treeview' in UI");

    setup_treeview(&viewtransactions_treeview);


    // Create a ListStore for all transactions
    let list_store = ListStore::new(&[
        String::static_type(),
        f64::static_type(),
        String::static_type(),
        i32::static_type()
        ]);
    
    // Set ListStore as the model for the TreeView
    viewtransactions_treeview.set_model(Some(&list_store));


    // Retrieve all transactions
     println!("--- Loading All Transactions ---");
     match finance.all_transactions(&conn) {
         Ok(transactions) => {
             for (amount, date, status, category_id) in transactions {
                 list_store.insert_with_values(
                    None, 
                    &[
                        (0, &status), 
                        (1, &amount), 
                        (2, &date),
                        (3, &category_id)
                        ],
                    );
             }
         }
         Err(err) => {
             eprintln!("Error retrieving transactions: {}", err);
         }
     }






    // Load TreeView from UI file for income file
    let viewincome_treeview: TreeView = builder
    .object("viewincome_treeview")
    .expect("Failed to find 'viewincome_treeview' in UI");

    setup_treeview(&viewincome_treeview);


    // Create a ListStore for income file
    let list_store1 = ListStore::new(&[
        String::static_type(),
        f64::static_type(),
        String::static_type(),
        i32::static_type()
        ]);

    // Set ListStore as the model for the TreeView
    viewincome_treeview.set_model(Some(&list_store1));


    // Updated: Retrieve incoming transactions
     println!("--- Loading All Transactions ---");
     match finance.income_transactions(&conn) {
         Ok(transactions) => {
             for (amount, date, status, category_id) in transactions {
                 list_store1.insert_with_values(
                    None, 
                    &[
                        (0, &status), 
                        (1, &amount), 
                        (2, &date),
                        (3, &category_id)
                        ],
                    );
             }
         }
         Err(err) => {
             eprintln!("Error retrieving transactions: {}", err);
         }
     }






    // Load TreeView from UI file for expense transactions
      let viewexpense_treeview: TreeView = builder
      .object("viewexpense_treeview")
      .expect("Failed to find 'viewexpense_treeview' in UI");

      setup_treeview(&viewexpense_treeview);
  
  
      // Create a ListStore 
      let list_store2 = ListStore::new(&[
          String::static_type(),
          f64::static_type(),
          String::static_type(),
          i32::static_type()
          ]);
  
        // Set ListStore as the model for the TreeView
          viewexpense_treeview.set_model(Some(&list_store2));
  

        // Updated: Retrieve all ezpenses transactions
       println!("--- Loading All Transactions ---");
       match finance.expense_transactions(&conn) {
           Ok(transactions) => {
               for (amount, date, status, category_id) in transactions {
                   list_store2.insert_with_values(
                      None, 
                      &[
                          (0, &status), 
                          (1, &amount), 
                          (2, &date),
                          (3, &category_id)
                          ],
                      );
               }
           }
           Err(err) => {
               eprintln!("Error retrieving transactions: {}", err);
           }
       }
  



     

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}