use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)] 
#[serde(rename_all = "PascalCase")] 
pub enum Eventype { 
    PatientCreated, 
    PatientCaseSlideAttached, 
} 
 
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Payload {
    Patient {
        #[serde(rename = "PatientID")]
        patient_id: String,
        #[serde(rename = "Name")]
        name: String,
    },
    File {
        #[serde(rename = "PatientID")]
        patient_id: String,
        #[serde(rename = "File")]
        file: String,
    },
}
 
#[derive(Debug, Deserialize, PartialEq, Clone)] 
#[serde(rename_all = "PascalCase")] 
pub struct Event { 
    pub event_type: Eventype, 
    pub payload: Payload, 
}