use crate::consistency::Consistency;
use crate::frame::*;
use crate::query::QueryValues;
use crate::query::{PreparedQuery, QueryFlags};
use crate::types::*;

/// `BodyResReady`
#[derive(Debug, Clone)]
pub struct BodyReqBatch {
    pub batch_type: BatchType,
    pub queries: Vec<BatchQuery>,
    pub consistency: Consistency,
    /// **IMPORTANT NOTE:** with names flag does not work and should not be used.
    /// https://github.com/apache/cassandra/blob/trunk/doc/native_protocol_v4.spec#L413
    pub query_flags: Vec<QueryFlags>,
    pub serial_consistency: Option<Consistency>,
    pub timestamp: Option<i64>,
    pub is_idempotent: bool,
}

impl AsBytes for BodyReqBatch {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.batch_type.as_byte()];

        let len = self.queries.len() as i16;
        bytes.extend_from_slice(&len.to_be_bytes());

        bytes = self.queries.iter().fold(bytes, |mut bytes, q| {
            bytes.append(&mut q.as_bytes());
            bytes
        });

        let consistency: i16 = self.consistency.into();
        bytes.extend_from_slice(&consistency.to_be_bytes());

        let flag_byte = self
            .query_flags
            .iter()
            .fold(0, |bytes, f| bytes | f.as_byte());
        bytes.push(flag_byte);

        if let Some(serial_consistency) = self.serial_consistency {
            let serial_consistency: i16 = serial_consistency.into();
            bytes.extend_from_slice(&serial_consistency.to_be_bytes());
        }

        if let Some(timestamp) = self.timestamp {
            bytes.extend_from_slice(&timestamp.to_be_bytes());
        }

        bytes
    }
}

/// Batch type
#[derive(Debug, Clone, PartialEq)]
pub enum BatchType {
    /// The batch will be "logged". This is equivalent to a
    /// normal CQL3 batch statement.
    Logged,
    /// The batch will be "unlogged".
    Unlogged,
    /// The batch will be a "counter" batch (and non-counter
    /// statements will be rejected).
    Counter,
}

impl FromSingleByte for BatchType {
    fn from_byte(byte: u8) -> BatchType {
        match byte {
            0 => BatchType::Logged,
            1 => BatchType::Unlogged,
            2 => BatchType::Counter,
            _ => unreachable!(),
        }
    }
}

impl AsByte for BatchType {
    fn as_byte(&self) -> u8 {
        match *self {
            BatchType::Logged => 0,
            BatchType::Unlogged => 1,
            BatchType::Counter => 2,
        }
    }
}

/// The structure that represents a query to be batched.
#[derive(Debug, Clone)]
pub struct BatchQuery {
    /// It indicates if a query was prepared.
    pub is_prepared: bool,
    /// It contains either id of prepared query of a query itself.
    pub subject: BatchQuerySubj,
    /// It is the optional name of the following <value_i>. It must be present
    /// if and only if the 0x40 flag is provided for the batch.
    /// **Important note:** this feature does not work and should not be
    /// used. It is specified in a way that makes it impossible for the server
    /// to implement. This will be fixed in a future version of the native
    /// protocol. See https://issues.apache.org/jira/browse/CASSANDRA-10246 for
    /// more details
    pub values: QueryValues,
}

/// It contains either an id of prepared query or CQL string.
#[derive(Debug, Clone)]
pub enum BatchQuerySubj {
    PreparedId(PreparedQuery),
    QueryString(CStringLong),
}

impl AsBytes for BatchQuery {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + INT_LEN + SHORT_LEN);

        // kind
        if self.is_prepared {
            bytes.push(1);
        } else {
            bytes.push(0);
        }

        match &self.subject {
            BatchQuerySubj::PreparedId(s) => {
                bytes.append(
                    &mut s
                        .id
                        .read()
                        .expect("Cannot read prepared query id!")
                        .as_bytes(),
                );
            }
            BatchQuerySubj::QueryString(s) => {
                bytes.append(&mut s.as_bytes());
            }
        }

        let len = self.values.len() as i16;
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.append(&mut self.values.as_bytes());

        bytes
    }
}

impl Frame {
    /// **Note:** This function should be used internally for building query request frames.
    pub fn new_req_batch(query: BodyReqBatch, flags: Vec<Flag>) -> Frame {
        let version = Version::Request;
        let opcode = Opcode::Batch;

        Frame::new(version, flags, opcode, query.as_bytes(), None, vec![])
    }
}
