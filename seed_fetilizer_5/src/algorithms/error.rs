use std::num::{ParseIntError, TryFromIntError};

pub enum SeedsDBError {
    ListDataRangeBadSize,
    Parse(ParseIntError),
    TryInt(TryFromIntError),
}

impl From<ParseIntError> for SeedsDBError {

    fn from(err: ParseIntError) -> SeedsDBError {
        SeedsDBError::Parse(err)
    }
}

impl From<TryFromIntError> for SeedsDBError {

    fn from(err: TryFromIntError) -> SeedsDBError {
        SeedsDBError::TryInt(err)
    }
}
