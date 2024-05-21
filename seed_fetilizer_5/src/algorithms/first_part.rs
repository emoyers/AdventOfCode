use crate::algorithms::{
    common::SeedRangeInfo,
    common::SeedsListType, 
    error::SeedsDBError};
use std::cmp::min;
use std::u64::MAX;

pub struct SeedsDB {

    seeds: Vec<u64>,
    seed_to_soil: Vec<SeedRangeInfo>,
    soil_to_fertilizer: Vec<SeedRangeInfo>,
    fertilizer_to_water: Vec<SeedRangeInfo>,
    water_to_light: Vec<SeedRangeInfo>,
    light_to_temperature: Vec<SeedRangeInfo>,
    temperature_to_humidity: Vec<SeedRangeInfo>,
    humidity_to_location: Vec<SeedRangeInfo>,
}

impl SeedsDB {

    pub fn new () -> Self {
        SeedsDB {
            seeds : Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }

    pub fn init_seed_vector(&mut self, seed_info: &str) -> Result<(), SeedsDBError>
    {
        let seeds_string: Vec<&str> = seed_info.split(':').collect();
        self.seeds = seeds_string[1].split_whitespace().map(|num| 
            num.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    pub fn populate_map_based_on_type(&mut self, list_type: &SeedsListType, data: &str) -> Result<(), SeedsDBError>
    {
        let list_temp: &mut Vec<SeedRangeInfo>;

        match list_type {
            SeedsListType::SeedToSoil => list_temp = &mut self.seed_to_soil,
            SeedsListType::SoilToFertilizer => list_temp = &mut self.soil_to_fertilizer,
            SeedsListType::FertilizerToWater => list_temp = &mut self.fertilizer_to_water,
            SeedsListType::WaterToLight => list_temp = & mut self.water_to_light,
            SeedsListType::LightToTemperature => list_temp = &mut self.light_to_temperature,
            SeedsListType::TemperatureToHumidity => list_temp = &mut self.temperature_to_humidity,
            SeedsListType::HumidityToLocation => list_temp = &mut self.humidity_to_location,
        }

        // Get the position ranges to populate the seeds
        let range: Vec<u64> = data.split_whitespace().map(|num| 
            num.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
        
        // Populate the map
        if range.len() == 3 {
            let temp_range_info :SeedRangeInfo = 
                SeedRangeInfo::new(range[1], 
                        range[1] + range[2] -1, 
                                   range[0]);

            list_temp.push(temp_range_info);
        }
        else {
            return Err(SeedsDBError::ListDataRangeBadSize)
        }
        
        Ok(())
    }

    pub fn get_minimum_location(&mut self) -> u64{

        let mut min_location: u64 = MAX;

        let mut seeds_copy = self.seeds.clone();
        for seed in &mut seeds_copy {
            let curr_location = self.get_location(*seed);

            min_location = min(min_location, curr_location);
        }

        return min_location;
    }

    fn get_location(&mut self, seed: u64) -> u64{
        let mut value: u64 = seed;
        

        for list_type in SeedsListType::get_all_variants() {

            match self.get_mapped_value_from_ranges(list_type, value) {
                Some(val) => value = val,
                None => value = value,
            }
        }

        value
    }

    fn get_mapped_value_from_ranges(&mut self, list_type: SeedsListType, value: u64) -> Option<u64> {

        let list_temp: &mut Vec<SeedRangeInfo>;

        match list_type {
            SeedsListType::SeedToSoil => list_temp = &mut self.seed_to_soil,
            SeedsListType::SoilToFertilizer => list_temp = &mut self.soil_to_fertilizer,
            SeedsListType::FertilizerToWater => list_temp = &mut self.fertilizer_to_water,
            SeedsListType::WaterToLight => list_temp = & mut self.water_to_light,
            SeedsListType::LightToTemperature => list_temp = &mut self.light_to_temperature,
            SeedsListType::TemperatureToHumidity => list_temp = &mut self.temperature_to_humidity,
            SeedsListType::HumidityToLocation => list_temp = &mut self.humidity_to_location,
        }

        for range in list_temp {
            if (range.start_source_index <= value) && (value <= range.end_source_index)
            {
                let offset_destination = value - range.start_source_index;
                return Some(range.start_destination_index + offset_destination);
            }
        }
        
        None
    }
}