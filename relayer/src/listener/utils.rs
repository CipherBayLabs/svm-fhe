// Helper function to parse [u8; 32] arrays from log messages
pub fn parse_array_from_log(log: &str) -> Option<[u8; 32]> {
    if let Some(start_idx) = log.find('[') {
        if let Some(end_idx) = log.find(']') {
            let array_str = &log[start_idx + 1..end_idx];
            
            // Parse the comma-separated values
            let values: Vec<u8> = array_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u8>().ok())
                .collect();
            
            // Convert Vec<u8> to [u8; 32]
            if values.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&values);
                return Some(arr);
            }
        }
    }
    None
}