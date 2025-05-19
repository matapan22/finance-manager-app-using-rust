use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Builder, Button, ToggleButton};
use std::io;
use chrono::Local; // For getting the current date automatically
use crate:: finance; // Your existing module
use finance::FinanceVariables;
use rusqlite::Connection;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::rc::Rc;
use std::cell::RefCell;





//load transaction window ui
const TRANSACTION_FILE: &str = "src/resources/transactionwindow.ui";
pub struct NewWindow {
    pub window: ApplicationWindow,
}

impl NewWindow {
    
    pub fn new_from_builder(builder: &Builder) -> Self {
        let builder = Builder::from_file(TRANSACTION_FILE);
        // Load the new window from the builder
        let window: ApplicationWindow = builder
            .object("transactionwindow")
            .expect("Failed to load 'Transaction window' from UI file");
            //connect to the data base
            let finance_database = Connection::open("Finance_app.db").expect("Failed to connect to database");
            println!("Connection to the database is successful in transaction window.");
        
            // Initialize the FinanceVariables struct
            let mut finance = FinanceVariables::new();
            let finance = Rc::new(RefCell::new(finance)); // Wrap finance in Rc<RefCell<T>>
            let status_value = Rc::new(RefCell::new(1)); // 0 = Expense, 1 = Income
            let category_id = Rc::new(RefCell::new(String::from("1"))); // Default: "1" globally declaring

    // Load the transaction entry field
    let transaction_entry: gtk4::Entry = builder
    .object("transaction_entry")
    .expect("Failed to load entry");



    
  
    static amount: Lazy<Mutex<f64>> = Lazy::new(|| Mutex::new(0.0));
    static category_id_converted: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
    
    //button for selecting income    
    let button_income: ToggleButton = builder
       .object("button_Income")
       .expect("Failed to load button_Income");
    

    //butt on for selecting expense
    let button_expense: ToggleButton = builder
        .object("button_Expense")
        .expect("Failed to load button_Expense");
    let status_clone = Rc::clone(&status_value);


    // Connect signals to check when income button toggled
    button_income.connect_toggled(move |btn| {
        if btn.is_active() {
            println!("Selected: Income");
            *status_clone.borrow_mut() = 1;  // Modify shared status_value
            println!("Status is: {}", *status_clone.borrow());  
        }
    });

    let status_clone = Rc::clone(&status_value);
       // Connect signals to check when expense button toggled
    button_expense.connect_toggled(move |btn| {
        if btn.is_active() {
            println!("Selected: Expense");
            //let status_value = 1;
            *status_clone.borrow_mut() = 0;  //Modify shared status_value
            println!("Status is: {}", *status_clone.borrow()); 
        }
    });



    //dropdown for category
    let category_dropdown: gtk4::ComboBoxText = builder
            .object("category_dropdown")
            .expect("Failed to load category_dropdown");

    // Set "Select Category" as the default
    category_dropdown.append(Some("default"), "--Select Category--"); // Placeholder

    //content or category for displaying same as in database
    category_dropdown.set_active_id(Some("default"));
        let categories = [
            ("1", "Rent"),
            ("2", "Transport"),
            ("3", "Food"),
            ("4", "Utilities"),
            ("5", "Clothing"),
            ("6", "Insurance"),
            ("7", "Entertainment"),
            ("8", "Miscellaneous"),
            ("9", "HouseHold Items"),
            ("10", "Salary"),
            ("11", "Personal Income"),
            ("12", "Personal Expense"),
            ("13", "Other"),
        ];
        
        //Populate dropdown with category IDs and labels
        for (id, label) in &categories {
            category_dropdown.append(Some(id), label);
        }
        
        //Ensure at least one category is selected by default
        if category_dropdown.active_text().is_none() || category_dropdown.active_id().is_none() {
            category_dropdown.set_active(Some(0)); 
            
        }
        
        let category_id_clone = category_id.clone(); // Clone for closure

        //Capture dropdown selection dynamically
        category_dropdown.connect_changed(move |combo| {
            if let Some(selected) = combo.active_id() {
                println!("Selected Category ID: {}", selected);
                *category_id_clone.borrow_mut() = selected.to_string(); //Update global category_id
            }
        }); 

    //button to save tranasaction
    let button: Button = builder
        .object("add_transactions_button")
        .expect("Failed to load my_button");


    
               


        let button_status = Rc::clone(&status_value);
        button.connect_clicked({
            let transaction_entry = transaction_entry.clone(); // Clone the entry for use in the closure
            let mut finance = finance;
        
        move |_| {
            // Get the text from the Entry
            let data1 = transaction_entry.text();
            // Try to parse it as an f64
            match data1.parse::<f64>() {
                Ok(transaction_p_value) => {
                    println!("Parsed f64 value: {}", transaction_p_value);
                    {
                    let mut global_value = amount.lock().unwrap();
                    *global_value = transaction_p_value;
                    }
                    
                },
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                },
            }

            println!("Amount Entered is: {}", *amount.lock().unwrap());



            let amount_value = *amount.lock().unwrap(); // Extract the f64
            
            //label for transaction error
            let transaction_error: gtk4::Label = builder
                .object("transaction_error")
                .expect("Failed to load balance_label");
            
            
                       
            // Step 3: Get the Current Date Automatically
            let current_date = Local::now().format("%Y-%m-%d").to_string();


            let current_status = *button_status.borrow();
            println!("Button Clicked! Current status: {}", current_status);
            drop(current_status); // Drop immutable borrow before modifying
            if current_status == 0 || current_status == 1 {
                // Step 4: Determine Transaction Status Based on ID and category id
                let status_str = if *button_status.borrow() == 0 { "Expense" } else { "Income" };

                //read category id selected by user using drop down
                let category_str = category_id.borrow();
                println!("Trial: {}", category_str);
                match category_str.parse::<i32>() {
                    Ok(parsed_id) => {
                        let mut id = category_id_converted.lock().unwrap(); // Lock and modify
                        *id = parsed_id; // Store globally
                        println!("Updated CATEGORY_ID: {}", *id);
                    }
                    Err(_) => {
                        println!("Invalid category ID");
                    }
                }


                // Step 5: Save Transaction to Database
                let c_id =*category_id_converted.lock().unwrap();
                let mut finance_mut = finance.borrow_mut();
                match finance_mut.transaction(amount_value, &current_date, status_str, &c_id, &finance_database) {
                    Ok(_) => {println!("Transaction added successfully.");
                            transaction_error.set_label("Transaction Added Sucessfully");}
                   Err(err) => println!("Failed to save transaction: {}", err),
                }

            }
            else{ println!("Invalid value. Allowed values: 0 (Expense) or 1 (Income).");
                    transaction_error.set_label("Invalid Input");
                }
            transaction_entry.set_text(""); // Clears the entry field
            println!("Entry field reset!");
        }




    });
    



        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
