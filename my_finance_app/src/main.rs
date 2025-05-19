use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Builder, Button, glib, TreeView, TreeViewColumn, ListStore, CellRendererText, MessageDialog, DialogFlags, MessageType, ButtonsType};
mod finance;
use finance::FinanceVariables;
use gtk4::gio;
use rusqlite::{Connection, Result};


//function to display popup
fn show_note_popup(parent: &ApplicationWindow) {
    let dialog = MessageDialog::new(
        Some(parent),
        DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        "This Feature Will be Available Soon!",
    );

    dialog.set_title(Some("We are Working on it"));
    dialog.connect_response(|dialog, _| {
        dialog.close(); // Close the dialog when OK is clicked
    });

    dialog.show();
}




mod transactionwindow;
mod viewtransactions;

const APP_ID: &str = "org.gtk_rs.MyGtkApp";
const UI_FILE: &str = "src/resources/window.ui";//location to the ui file

fn main() -> glib::ExitCode {
    gtk4::init().expect("Failed to initialize GTK");

    // Load the compiled resources
    let resource = gio::Resource::load("resources.gresource")
        .expect("Failed to load resources.gresource");
    gio::resources_register(&resource);
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}









fn build_ui(app: &Application) {
    let builder =  Builder::from_file(UI_FILE);
    //declaration of application window
    let main_window: ApplicationWindow = builder
        .object("main_window")
        .expect("Failed to load main_window");
    main_window.set_application(Some(app));


    // Connect to your SQLite database
    let db_path = "Finance_app.db"; // Adjust this path to your database location
    let conn = match Connection::open(db_path) {
        Ok(conn) => {
            print!("Database Connection established in main window ");
            conn},
        Err(err) => {
            eprintln!("Error opening database: {}", err);
            return;
        }
    };

    // Initialize FinanceVariables
    let mut finance = finance::FinanceVariables::new();

    //balance label for displaying balance
    let balance_label: gtk4::Label = builder
    .object("balance_label")
    .expect("Failed to load balance_label");


    // Calculate total budget
    match finance.total_budget(&conn) {
        Ok(budget) => {
            println!("--- Total Budget ---");
            println!("Your total budget is: €{:.2}", budget);
            balance_label.set_label(&format!("€{:.2}", budget));
        }
        Err(err) => {
            eprintln!("Error calculating budget: {}", err);
        }
    };


    // Load TreeView from UI file for latest 10 transaction 
    let transactions_treeview: TreeView = builder
        .object("transactions_treeview")
        .expect("Failed to find 'transactions_treeview' in UI");
   

    // Create a ListStore
    let list_store = ListStore::new(&[
        String::static_type(),
        f64::static_type(),
        String::static_type(),
        String::static_type()
        ]);


    // Set ListStore as the model for the TreeView
    transactions_treeview.set_model(Some(&list_store));



     // Retrieve last 10 transactions
     println!("--- Loading Last 10 Transactions ---");
     match finance.last_10transaction(&conn) {
         Ok(transactions) => {
             for (amount, date, status, category_id) in transactions {
                 //println!("Type: {}, Amount: ${:.2}, Date: {}", status, amount, date);
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

   

     //button for opening transaction window
    let ct_button: Button = builder
        .object("call_transactions_button")
        .expect("Failed to load call_transactions_button");
    ct_button.connect_clicked({
        print!("call_transactions_button clicked");
        let builder = Builder::from_resource("/org/my_finance_app/transactionwindow.ui");//location of transaction window ui file
        move |_| {
            println!("opening new window!!");
            let transactionwindow = transactionwindow::NewWindow::new_from_builder(&builder);
            transactionwindow.present();
        }
        
    });


    //button for viewing transactions
    let vt_button: Button = builder
        .object("view_tranasctions_button")
        .expect("Failed to load view_tranasctions_button");
    vt_button.connect_clicked({
        print!("view_transactions_button clicked");
        let builder = Builder::from_resource("/org/my_finance_app/viewtransactions.ui");//location of viw transaction window ui file
        move |_| {
            println!("opening new window!!");
            let viewtransactions = viewtransactions::NewWindow::new_from_builder(&builder);
            viewtransactions.present();
        }
        
    });

    // button for setting button
    let button: Button = builder
        .object("set_budget_button")
        .expect("Failed to load set_budget_button");
    let main_window_clone = main_window.clone();
    button.connect_clicked(move |_| {
        print!("set_budget_button clicked");
        show_note_popup(&main_window_clone);//calling popup window
    });

    //button for adding category
    let button: Button = builder
        .object("add_category_button")
        .expect("Failed to load add_category_button");
    let main_window_clone = main_window.clone();
    button.connect_clicked(move |_| {
        print!("add_category_button clicked");
        show_note_popup(&main_window_clone);//calling popup window
    });

    //button for viewing insight
    let button: Button = builder
        .object("insight_button")
        .expect("Failed to load insight_button");
    let main_window_clone = main_window.clone();
    button.connect_clicked(move |_| {
        print!("insight_button clicked");
        show_note_popup(&main_window_clone);//calling popup window
    });


    //button for viewing statement
    let st_button: Button = builder
        .object("statement_button")
        .expect("Failed to load statement_button");
    st_button.connect_clicked({
        print!("statement_button clicked");
        let builder = Builder::from_resource("/org/my_finance_app/viewtransactions.ui");//location of view transaction window ui file
        move |_| {
            println!("opening new window!!");
            let viewtransactions = viewtransactions::NewWindow::new_from_builder(&builder);
            viewtransactions.present();
        }
        
    });



    
    
    main_window.present();
}









