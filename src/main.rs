use std::{env, process};
use std::error::Error;
use transactions_engine::{TransactionRecord, TransactionService};

#[macro_use]
extern crate log;


fn process_file(path : String, mut service: TransactionService) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record : TransactionRecord = result?;
        debug!("{:?}", record);
        let res = service.process(record);
        match res {
            Ok(_) => info!("Transaction went through successfully"),
            Err(err) => error!("Error while executing transaction: {:?}", err)
        }
    }
    service.generate_report();
    Ok(())
}


fn main() {
    env_logger::init();
    info!("Starting up!");
    let args: Vec<String> = env::args().collect();
    let file_name = args[1].clone();
    let service : TransactionService = Default::default();
    if let Err(err) = process_file(file_name, service) {
        // this path occurs if there any errors while parsing the csv.
        warn!("error running example: {}", err);
        process::exit(1);
    }
}
