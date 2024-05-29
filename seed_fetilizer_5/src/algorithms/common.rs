use super::error::SeedsDBError;

 #[derive(Clone)]
pub struct SeedRangeInfo {
    pub start_source_index: u64,
    pub end_source_index: u64,
    pub start_destination_index: u64,
}

impl SeedRangeInfo {
    
    pub fn new(start_source:u64, end_source:u64, start_dest:u64) -> Self
    {
        SeedRangeInfo {
            start_source_index: start_source,
            end_source_index: end_source,
            start_destination_index: start_dest,
        }
    } 
}

#[derive(Debug)]
pub enum SeedsListType{
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl SeedsListType {
    
    pub fn get_all_variants() -> Vec<SeedsListType> {

        vec![SeedsListType:: SeedToSoil, SeedsListType::SoilToFertilizer, 
             SeedsListType::FertilizerToWater, SeedsListType::WaterToLight, 
             SeedsListType::LightToTemperature, SeedsListType::TemperatureToHumidity, 
             SeedsListType::HumidityToLocation]
    }
}

pub trait SeedsAlgorithm {

    fn populate_map_based(map_db: &mut Vec<SeedRangeInfo>, data: &str) -> Result<(), SeedsDBError>{
        
        // Get the position ranges to populate the seeds
        let range: Vec<u64> = data.split_whitespace().map(|num| 
            num.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
        
        // Populate the map
        if range.len() == 3 {
            let temp_range_info :SeedRangeInfo = 
                SeedRangeInfo::new(range[1], 
                        range[1] + range[2] -1, 
                                   range[0]);

            map_db.push(temp_range_info);
        }
        else {
            return Err(SeedsDBError::ListDataRangeBadSize)
        }
        
        Ok(())
    }
}