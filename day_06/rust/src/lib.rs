use std::fs;
use track_record::TrackRecords;

mod boat;
mod track_record;

pub fn part_1(file_name: &str) -> u32 {
    let track_records = try_get_track_records(file_name);

    if let Some(track_records) = track_records {
        get_product_of_number_of_ways_to_beat_track_records(track_records, 1)
    } else {
        println!("Failed to get track records");
        0
    }
}

pub fn part_2(file_name: &str) -> u32 {
    todo!()
}

fn try_get_track_records(file_name: &str) -> Option<TrackRecords> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    TrackRecords::try_build(file_lines)
}

fn get_product_of_number_of_ways_to_beat_track_records(
    track_records: TrackRecords,
    acceleration_rate: u32,
) -> u32 {
    let number_of_ways_to_beat_track_records =
        track_records.get_number_of_ways_to_beat_track_records(acceleration_rate);

    number_of_ways_to_beat_track_records.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 288);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }
}
