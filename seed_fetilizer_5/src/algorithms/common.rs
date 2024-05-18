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