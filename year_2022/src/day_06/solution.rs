#[cfg(test)]
mod day_06_tests {
    use crate::day_06::datastream_buffer::DatastreamBuffer;

    #[test]
    fn test_day06_solution_part_one() {
        let datastream_buffer = DatastreamBuffer(include_str!("input/datastream_buffer.txt"));

        assert_eq!(datastream_buffer.first_start_of_packet_marker_characters_processed(), 1142);
    }

    #[test]
    fn test_day06_solution_part_two() {
        let datastream_buffer = DatastreamBuffer(include_str!("input/datastream_buffer.txt"));

        assert_eq!(datastream_buffer.first_start_of_message_marker_characters_processed(), 2803);
    }
}
