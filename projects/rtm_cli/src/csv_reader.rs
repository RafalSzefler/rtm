use std::{collections::HashMap, io::Read};

use rtm_core::models::{AccountingOperation, Amount, ClientId, Transaction, TransactionId, TransactionKind};

const HEADERS: &[&str] = &["type", "client", "tx", "amount"];

#[derive(Debug)]
pub enum CsvReaderError {
    HeaderNotFound,
    InvalidHeaders,
}

pub struct CsvReader<T: Read> {
    reader: csv::Reader<T>,
}

impl<T: Read> CsvReader<T> {
    pub fn new(stream: T) -> Self {
        let reader = csv::Reader::from_reader(stream);
        Self { reader }
    }

    pub fn read_iter(&mut self) -> Result<CsvReaderIterator<'_, T>, CsvReaderError> {
        let Ok(headers) = self.reader.headers() else {
            return Err(CsvReaderError::HeaderNotFound);
        };

        let mut header_map = HashMap::new();
        for (i, header) in headers.iter().enumerate() {
            header_map.insert(header.trim().to_lowercase(), i);
        }
        if header_map.len() != HEADERS.len() {
            return Err(CsvReaderError::InvalidHeaders);
        }
        for header in HEADERS {
            if !header_map.contains_key(*header) {
                return Err(CsvReaderError::InvalidHeaders);
            }
        }

        let records = self.reader.records();
        return Ok(CsvReaderIterator::new(records, header_map));
    }
}

pub struct CsvReaderIterator<'a, T: Read> {
    records: csv::StringRecordsIter<'a, T>,
    header_map: HashMap<String, usize>,
}

impl<'a, T: Read> CsvReaderIterator<'a, T> {
    pub fn new(records: csv::StringRecordsIter<'a, T>, header_map: HashMap<String, usize>) -> Self {
        Self { records, header_map }
    }
}

impl<T: Read> Iterator for CsvReaderIterator<'_, T> {
    type Item = AccountingOperation;

    fn next(&mut self) -> Option<Self::Item> {
        macro_rules! read_field {
            ($record:expr, $field:expr) => {
                ($record.get(*self.header_map.get($field).unwrap()).unwrap())
            };
        }

        loop {
            let next = self.records.next()?;

            let Ok(record) = next else {
                continue;
            };

            if record.len() != HEADERS.len() {
                continue;
            }

            let Ok(client) = ClientId::try_from(read_field!(&record, "client").trim()) else {
                continue;
            };

            let Ok(tx) = TransactionId::try_from(read_field!(&record, "tx").trim()) else {
                continue;
            };

            let amount_field = read_field!(&record, "amount").trim();

            let record_type = read_field!(&record, "type").trim();

            match record_type {
                "deposit" => {
                    let Ok(amount) = Amount::try_from(amount_field) else {
                        continue;
                    };
                    return Some(AccountingOperation::Transaction {
                        transaction: Transaction::new(client, tx, amount, TransactionKind::Deposit),
                    });
                }
                "withdrawal" => {
                    let Ok(amount) = Amount::try_from(amount_field) else {
                        continue;
                    };
                    return Some(AccountingOperation::Transaction {
                        transaction: Transaction::new(client, tx, amount, TransactionKind::Withdrawal),
                    });
                }
                "dispute" => {
                    return Some(AccountingOperation::Dispute {
                        client_id: client,
                        ref_id: tx,
                    });
                }
                "resolve" => {
                    return Some(AccountingOperation::Resolve {
                        client_id: client,
                        ref_id: tx,
                    });
                }
                "chargeback" => {
                    return Some(AccountingOperation::Chargeback {
                        client_id: client,
                        ref_id: tx,
                    });
                }
                _ => {
                    return None;
                }
            }
        }
    }
}
