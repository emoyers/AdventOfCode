use crate::algorithms::{common::SeedsListType, error::SeedsDBError};

use std::u64::MAX;

#[derive(Debug)]
pub struct SeedsDBRanges {

}

impl SeedsDBRanges {
    
    pub fn new () -> Self {
        SeedsDBRanges{

        }
    }

    pub fn init_seed_vector(&mut self, seed_info: &str) -> Result<(), SeedsDBError>
    {
        Ok(())
    }

    pub fn populate(&mut self, list_type: &SeedsListType, data: &str) -> Result<(), SeedsDBError>
    {
        Ok(())
    }

    pub fn get_minimum_location(&mut self) -> u64{

        let mut min_location: u64 = MAX;

        println!("To be implemented");

        return min_location;
    }
}