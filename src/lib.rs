pub const WEBHOOK_URL: &str = "https://discord.com/api/webhooks/1013991255090401301/RDIf7DPBQKu1q5bFpya2ypXG5YIZW7mtuM-BtU8AggDZf6II4WY-psINGXzSXRLaE28T";

// finding yt channel ids for channels with official names
// https://stackoverflow.com/questions/14366648/how-can-i-get-a-channel-id-from-youtube
// https://commentpicker.com/youtube-channel-id.php
pub const CHANNEL_IDS: [&str; 7] = [
    "UCqK_GSMbpiV8spgD3ZGloSw",  // Coin Bureau
    "UCRXcryyD7dzNQzd0Zkbj3ug",  // Julien Chieze
    "UCqJ-Xo29CKyLTjn6z2XwYAw",  // Mark Brown GMT
    "UC2D2CMWXMOVWx7giW1n3LIg",  // Huberman Lab
    "UCAcAnMF0OrCtUep3Y4M-ZPw",  // Hugo Décrypte
    "UC0JB7TSe49lg56u6qH8y_MQ",  // GDC
    "UCv1DvRY5PyHHt3KN9ghunuw"   // MAsahiro Sakurai
];

pub const RUN_FREQUENCY: usize = 86400;