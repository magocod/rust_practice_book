use section_1;
use section_1::transaction::TransactionError;

fn main() -> Result<(), TransactionError> {
    section_1::point::serialize_point_example();

    let trans = section_1::transaction::get_transactions("test_data/transactions.json")?;
    for t in trans {
        println!("{:?}", t);
    }

    // fail
    let t = section_1::transaction::get_first_transaction_for("test_data/transactions.json", "Matt");
    match t {
        Ok(v) => println!("Found transaction: {:?}", v),
        Err(e) => {
            match e {
                TransactionError::LoadError(e) => { println!("LoadError: {:?}", e) }
                TransactionError::ParseError(e) => { println!("ParseError: {:?}", e) }
                TransactionError::Message(e) => { println!("Message: {:?}", e) }
            }
        },
    }

    Ok(())
}
