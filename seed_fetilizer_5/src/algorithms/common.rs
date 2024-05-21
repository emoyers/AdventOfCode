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