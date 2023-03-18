// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// struct VehicleData {
//     vehicleNo: String,
//     r#type: String,
//     data: Data,
// }
// #[derive(Debug, Deserialize, Serialize)]
// struct Data {
//     time: u64,
//     #[serde(default)]
//     lat: Option<f64>,
//     lng: Option<f64>,
//     alti: Option<u32>,
//     speed: Option<u32>,
//     ignstatus: Option<u32>,
//     heading: Option<u32>,
//     #[serde(default)]
//     rpm: Option<u32>,
//     #[serde(default)]
//     ect: Option<u32>,
//     #[serde(default)]
//     parameter4: Option<String>,
//     #[serde(default)]
//     parameter5: Option<String>,
//     #[serde(default)]
//     event: Option<String>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct BulkData {
//     bulkData: Vec<VehicleData>,
// }

// fn main() {
//     let json_str = r#"
//         {
//             "bulkData": [
//                 {
//                     "vehicleNo": "KAXXAB1234",
//                     "type": "gps",
//                     "data": {
//                         "time": 1612092829000,
//                         "lat": 12.964287,
//                         "lng": 77.591894,
//                         "alti": 815,
//                         "speed": 23,
//                         "ignstatus": 0,
//                         "heading": 98
//                     }
//                 },
//                 {
//                     "vehicleNo": "KAXXAB1234",
//                     "type": "can",
//                     "data": {
//                         "time": 1612092829000,
//                         "speed": 45,
//                         "rpm": 4580,
//                         "ect": 86,
//                         "parameter4": "value4",
//                         "parameter5": "value5"
//                     }
//                 },
//                 {
//                     "vehicleNo": "KAXXAB3456",
//                     "type": "event",
//                     "data": {
//                         "time": 1612092829000,
//                         "event": "power_disconnected"
//                     }
//                 },
//                 {
//                     "vehicleNo": "KAXXAB5678",
//                     "type": "gps",
//                     "data": {
//                         "time": 1612092829000,
//                         "lat": 12.964287,
//                         "lng": 77.591894,
//                         "alti": 815,
//                         "speed": 23,
//                         "ignstatus": 0,
//                         "heading": 98
//                     }
//                 }
//             ]
//         }
//     "#;

//     let bulk_data: BulkData = serde_json::from_str(json_str).unwrap();

//     // println!("{:#?}", bulk_data);
// // }
//  let gps_data: Vec<VehicleData> = bulk_data.bulkData.into_iter().filter(|data|
//         data.r#type == "gps"
//     ).collect();

//     println!("{:#?}", gps_data);
// }


// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// struct Data {
//     vehicleNo: String,
//     #[serde(rename = "type")]
//     data_type: String,
//     data: DataContent,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct DataContent {
//     time: Option<i64>,
//     lat: Option<f64>,
//     lng: Option<f64>,
//     alti: Option<i64>,
//     speed: Option<i64>,
//     ignstatus: Option<i64>,
//     heading: Option<i64>,
//     rpm: Option<i64>,
//     ect: Option<i64>,
//     parameter4: Option<String>,
//     parameter5: Option<String>,
//     event: Option<String>,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct JsonData {
//     bulkData: Vec<Data>,
// }

// fn main() {
//     let json_str = r#"
//     {
//         "bulkData": [
//             {
//                 "vehicleNo": "KAXXAB1234",
//                 "type": "gps",
//                 "data": {
//                     "time": 1612092829000,
//                     "lat": 12.964287,
//                     "lng": 77.591894,
//                     "alti": 815,
//                     "speed": 23,
//                     "ignstatus": 0,
//                     "heading": 98
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB1234",
//                 "type": "can",
//                 "data": {
//                     "time": 1612092829000,
//                     "speed": 45,
//                     "rpm": 4580,
//                     "ect": 86,
//                     "parameter4": "value4",
//                     "parameter5": "value5"
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB3456",
//                 "type": "event",
//                 "data": {
//                     "time": 1612092829000,
//                     "event": "power_disconnected"
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB5678",
//                 "type": "gps",
//                 "data": {
//                     "time": 1612092829000,
//                     "lat": 12.964287,
//                     "lng": 77.591894,
//                     "alti": 815,
//                     "speed": 23,
//                     "ignstatus": 0,
//                     "heading": 98
//                 }
//             }
//         ]
//     }
//     "#;

//     let json_data: JsonData = serde_json::from_str(json_str).unwrap();

//     for data in json_data.bulkData {
//         println!("Vehicle Number: {}", data.vehicleNo);
//         println!("Data Type: {}", data.data_type);
//         println!("Data: {:?}", data.data);
//     }
// }

// use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, Serialize)]
// struct GpsData {
//     time: i64,
//     lat: f64,
//     lng: f64,
//     alti: i64,
//     speed: i64,
//     ignstatus: i64,
//     heading: i64,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct CanData {
//     time: i64,
//     speed: i64,
//     rpm: i64,
//     ect: i64,
//     parameter4: String,
//     parameter5: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct EventData {
//     time: i64,
//     event: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(tag = "type")]
// enum Data {
//     #[serde(rename = "gps")]
//     Gps(GpsData),
//     #[serde(rename = "can")]
//     Can(CanData),
//     #[serde(rename = "event")]
//     Event(EventData),
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct VehicleData {
//     vehicle_no: String,
//     #[serde(flatten)]
//     data: Data,
// }

// #[derive(Debug, Deserialize, Serialize)]
// struct JsonData {
//     bulkData: Vec<VehicleData>,
// }

// fn main() {
//     let json_str = r#"
//     {
//         "bulkData": [
//             {
//                 "vehicleNo": "KAXXAB1234",
//                 "type": "gps",
//                 "data": {
//                     "time": 1612092829000,
//                     "lat": 12.964287,
//                     "lng": 77.591894,
//                     "alti": 815,
//                     "speed": 23,
//                     "ignstatus": 0,
//                     "heading": 98
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB1234",
//                 "type": "can",
//                 "data": {
//                     "time": 1612092829000,
//                     "speed": 45,
//                     "rpm": 4580,
//                     "ect": 86,
//                     "parameter4": "value4",
//                     "parameter5": "value5"
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB3456",
//                 "type": "event",
//                 "data": {
//                     "time": 1612092829000,
//                     "event": "power_disconnected"
//                 }
//             },
//             {
//                 "vehicleNo": "KAXXAB5678",
//                 "type": "gps",
//                 "data": {
//                     "time": 1612092829000,
//                     "lat": 12.964287,
//                     "lng": 77.591894,
//                     "alti": 815,
//                     "speed": 23,
//                     "ignstatus": 0,
//                     "heading": 98
//                 }
//             }
//         ]
//     }
//     "#;

//     let json_data: JsonData = serde_json::from_str(json_str).unwrap();

//     for data in json_data.bulkData {
//         match data.data {
//             Data::Gps(gps_data) => {
//                 println!("Vehicle Number: {}", data.vehicle_no);
//                 println!("Data Type: gps");
//                 println!("Data: {:?}", gps_data);
//             }
//             Data::Can(can_data) => {
//                 println!("Vehicle Number: {}", data.vehicle_no);
//                 println!("Data Type: can");
//                 println!("Data: {:?}", can_data);
//             }
//             Data::Event(event_data) => {
//                 println!("Vehicle Number: {}", data.vehicle_no);
//                 println!("Data Type: event");
//                 println!("Data: {:?}", event_data);
//             }
//         }
//     }
// }


use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct GpsData {
    time: i64,
    lat: f64,
    lng: f64,
    alti: i64,
    speed: i64,
    ignstatus: i64,
    heading: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct CanData {
    time: i64,
    speed: i64,
    rpm: i64,
    ect: i64,
    parameter4: String,
    parameter5: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventData {
    time: i64,
    event: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum Data {
    #[serde(rename = "gps")]
    Gps(GpsData),
    #[serde(rename = "can")]
    Can(CanData),
    #[serde(rename = "event")]
    Event(EventData),
}

#[derive(Debug, Deserialize, Serialize)]
struct VehicleData {
    vehicle_no: Option<String>,
    #[serde(flatten)]
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
struct JsonData {
    bulkData: Vec<VehicleData>,
}

fn main() {
    let json_str = r#"
    {
        "bulkData": [
            {
                "vehicleNo": "KAXXAB1234",
                "type": "gps",
                "data": {
                    "time": 1612092829000,
                    "lat": 12.964287,
                    "lng": 77.591894,
                    "alti": 815,
                    "speed": 23,
                    "ignstatus": 0,
                    "heading": 98
                }
            },
            {
                "vehicleNo": "KAXXAB1234",
                "type": "can",
                "data": {
                    "time": 1612092829000,
                    "speed": 45,
                    "rpm": 4580,
                    "ect": 86,
                    "parameter4": "value4",
                    "parameter5": "value5"
                }
            },
            {
                "vehicleNo": "KAXXAB3456",
                "type": "event",
                "data": {
                    "time": 1612092829000,
                    "event": "power_disconnected"
                }
            },
            {
                "vehicleNo": "KAXXAB5678",
                "type": "gps",
                "data": {
                    "time": 1612092829000,
                    "lat": 12.964287,
                    "lng": 77.591894,
                    "alti": 815,
                    "speed": 23,
                    "ignstatus": 0,
                    "heading": 98
                }
            }
        ]
    }
    "#;

    let json_data: JsonData = serde_json::from_str(json_str).unwrap();

    for data in json_data.bulkData {
        match data.vehicle_no {
            Some(vehicle_no) => {
                match data.data{
                    Data::Gps(gps_data) => {
                        println!("Vehicle Number: {}", vehicle_no);
                        println!("Data Type: gps");
                        println!("Data: {:?}", gps_data);
                    }
                    Data::Can(can_data) => {
                        println!("Vehicle Number: {}", vehicle_no);
                        println!("Data Type: can");
                        println!("Data: {:?}", can_data);
                    }
                    Data::Event(event_data) => {
                        println!("Vehicle Number: {}", vehicle_no);
                        println!("Data Type: event");
                        println!("Data: {:?}", event_data);
                    }
                }
            }
            None => {
                println!("Error: missing vehicle number");
            }
        }
    }
}
