use serde::{Deserialize, Serialize};

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

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to && self.amount == other.amount
    }
}

pub fn get_first_transaction_for(
    fname: &str,
    uname: &str,
) -> Result<Transaction, TransactionError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn found_transaction_for() {
        let t1 = Transaction {
            from: String::from("Matt"),
            to: String::from("Andy"),
            amount: 365,
        };
        let t2 = get_first_transaction_for("test_data/transactions.json", "Matt");

        match t2 {
            Ok(t) => {
                assert_eq!(t1, t);
            }
            Err(_e) => {
                panic!("optimize test");
            }
        }
    }

    #[test]
    fn not_found_transaction_for() {
        let t2 = get_first_transaction_for("test_data/transactions.json", "NotFound");

        match t2 {
            Ok(t) => {
                panic!("error found transaction: {:?}", t);
            }
            Err(e) => {
                assert!(matches!(e, TransactionError::Message("Could not find transaction with that name")));
            }
        }
    }

    #[test]
    fn reading_invalid_json() {
        let r = get_transactions("test_data/invalid_transactions.json");

        match r {
            Ok(_t) => {
                panic!("the file can be read");
            }
            Err(e) => {
                assert!(matches!(e, TransactionError::ParseError(_e)), "must be a parse error");
            }
        }
    }
}
