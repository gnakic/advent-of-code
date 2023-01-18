use std::collections::HashSet;
use std::ptr;

pub struct DatastreamBuffer(pub &'static str);

enum DatastreamMarker {
    StartOfPacket = 4,
    StartOfMessage = 14,
}

impl DatastreamBuffer {
    pub fn first_start_of_packet_marker_characters_processed(&self) -> usize {
        self.first_marker_characters_processed(DatastreamMarker::StartOfPacket)
    }

    pub fn first_start_of_message_marker_characters_processed(&self) -> usize {
        self.first_marker_characters_processed(DatastreamMarker::StartOfMessage)
    }

    fn first_marker_characters_processed(&self, datastream_marker: DatastreamMarker) -> usize {
        let marker_size = datastream_marker as usize;
        let buffer_contents = self.0;
        let buffer_contents_characters = buffer_contents.chars().collect::<Vec<char>>();
        let mut processed_characters_until_marker_detection: usize = 0;

        for window in buffer_contents_characters.windows(marker_size) {
            let unique_elements = window.iter().collect::<HashSet<&char>>();

            let is_datastream_marker = unique_elements.len() == window.len();

            if is_datastream_marker {
                processed_characters_until_marker_detection = buffer_contents_characters
                    .iter()
                    .position(|x| ptr::eq(&window[marker_size - 1], x))
                    .map_or(0, |marker_ending_position| marker_ending_position + 1);

                break;
            }
        }

        processed_characters_until_marker_detection
    }
}
