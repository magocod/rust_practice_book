use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Message(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Message(e)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

pub fn get_first_transaction_for(fname: &str, uname: &str) -> Result<Transaction, TransactionError> {
    let trans = get_transactions(fname)?;
    for t in trans {
        if t.from == uname {
            return Ok(t);
        }
    }
    Err("Could not find transaction with that name".into())
}

pub fn get_transactions(fname: &str) -> Result<Vec<Transaction>, TransactionError> {
    /*std::fs::read_to_string(fname)
    .map_err(|e| e.into())
    .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
    */

    /*Ok(
        match serde_json::from_str(&match std::fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        }) {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        },
    )*/
    Ok(serde_json::from_str(&std::fs::read_to_string(fname)?)?)
}