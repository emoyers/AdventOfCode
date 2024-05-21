use crate::algorithms::{
    common::SeedRangeInfo,
    common::SeedsListType, 
    error::SeedsDBError};

use std::u64::MAX;

pub struct SeedsDBRanges {
    seeds: Vec<Range>,
    seed_to_soil: Vec<SeedRangeInfo>,
    soil_to_fertilizer: Vec<SeedRangeInfo>,
    fertilizer_to_water: Vec<SeedRangeInfo>,
    water_to_light: Vec<SeedRangeInfo>,
    light_to_temperature: Vec<SeedRangeInfo>,
    temperature_to_humidity: Vec<SeedRangeInfo>,
    humidity_to_location: Vec<SeedRangeInfo>,
}

impl SeedsDBRanges {
    
    pub fn new () -> Self {
        SeedsDBRanges{
            seeds: Vec::new(),
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
        let seeds_input_numeric = seeds_string[1].split_whitespace().map(|num| 
            num.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;

        let mut i: usize= 0;

        while i < seeds_input_numeric.len() - 1 {
            self.seeds.push(Range { start_index: seeds_input_numeric[i], 
                end_index: seeds_input_numeric[i] + seeds_input_numeric[i+1] -1 });
            
            i += 2;
        }

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

        self.sort_map_ranges();

        let seeds_copy = self.seeds.clone();
        let mut ranges: Vec<Range> = seeds_copy;
        

        for list_type in SeedsListType::get_all_variants() {

            ranges = self.get_mapped_ranges_from_ranges(list_type, &ranges);
        }

        let mut min_location = MAX;
        for range in ranges{

            if range.start_index < min_location{
                min_location = range.start_index;
            }
        }
        min_location
    }

    fn get_mapped_ranges_from_ranges(&mut self, list_type: SeedsListType, ranges: &Vec<Range>) -> Vec<Range> {

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

        let mut resulting_ranges: Vec<Range> = Vec::new();

        for range in ranges {
            resulting_ranges.extend(SeedsDBRanges::get_mapped_range_from_ranges(&list_temp, range));
        }

        resulting_ranges

    }

    fn get_mapped_range_from_ranges(list_ranges: &Vec<SeedRangeInfo>, range: &Range) -> Vec<Range> {
        let mut resulting_ranges: Vec<Range> = Vec::new();

        let mut current_num = range.start_index;
        let mut temp_range_info = SeedRangeInfo::new(0, 0, 0);
        let mut offset_destination = 0;
        let mut total_number_in_range = 0;

        while current_num < range.end_index {
            temp_range_info = SeedsDBRanges::get_range_info(list_ranges, current_num, range.end_index);
            offset_destination = current_num - temp_range_info.start_source_index;

            if range.end_index > temp_range_info.end_source_index{
                total_number_in_range = temp_range_info.end_source_index - current_num;
            }
            else {
                total_number_in_range = range.end_index - current_num;
            }

            resulting_ranges.push(
                Range{
                    start_index:temp_range_info.start_destination_index + offset_destination, 
                    end_index:temp_range_info.start_destination_index + offset_destination + total_number_in_range,
                });
            current_num = temp_range_info.end_source_index + 1;
        }

        resulting_ranges

    }

    fn get_range_info(list_ranges: &Vec<SeedRangeInfo>, value: u64, end_index:u64) -> SeedRangeInfo {

        for (i,range) in list_ranges.iter().enumerate() {
            if (range.start_source_index <= value) && (value <= range.end_source_index) {
                return range.clone();
            }
            else if value < range.start_source_index {

                if i == 0{
                    return SeedRangeInfo::new(0, 
                        range.start_source_index-1, 
                        0);
                }
                else {
                    return SeedRangeInfo::new(list_ranges[i-1].end_source_index+1, 
                        range.start_source_index-1, 
                        list_ranges[i-1].end_source_index+1);
                }

                
            }
        }

        SeedRangeInfo::new(list_ranges[list_ranges.len()-1].end_source_index+1, 
            end_index, 
            list_ranges[list_ranges.len()-1].end_source_index+1)
    }

    fn sort_map_ranges(&mut self) {
        self.seed_to_soil.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
        
        self.soil_to_fertilizer.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
        
        self.fertilizer_to_water.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
        
        self.water_to_light.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
        
        self.light_to_temperature.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
        
        self.temperature_to_humidity.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));

        self.humidity_to_location.sort_by(|a, b| 
            a.start_source_index.cmp(&b.start_source_index));
    }

}

#[derive(Clone)] #[derive(Debug)]
struct Range{
    start_index: u64,
    end_index: u64,
}