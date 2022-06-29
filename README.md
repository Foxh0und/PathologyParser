## Pathology Parser

### Problem Description
Your task is to develop a command-line tool that processes a stream of events and outputs a summary of the event data.

#### Task Requirements
Given a file containing events like the following:

```json
[
    {
        "EventType": "PatientCreated",
        "Payload": {
            "PatientID": "a30bb29d",
            "Name": "Olivia"
        }
    },
    {
        "EventType": "PatientCaseSlideAttached",
        "Payload": {
            "PatientID": "a30bb29d",
            "File": "d9392a98.tiff"
        }
    }
]
```

Write a command-line tool that displays the following output:

```
Olivia: d9392a98.tiff, f796f432.tiff
Sophie: ccd803fd.tiff
```

### Analysis
This solution requires the ability to read in a file, most likely from a command line argument, parse it into well structured data, grouping them based on the patient (identified by their ID), and print out a list of patients along with their corresponding case slides.

#### Assumptions
* The user will enter the file name from the command line when running the program
* The data will be entered cleanly
* The data will be properly structured, with the Patient being created BEFORE they receive any cases slides

### Solution Design
The implementation will be done using Rust. 

#### Model
The domain model will be the first thing implemented

##### Input
These can be found in `model/input.rs`.

###### Event Type
A list of possible event types, here `[PatientCreated, PatientCaseSlideAttached]`

###### Payload
Depending on the Event Type, two possible payloads will arrive, either containing a file or patient name, along with a corresponding ID.

###### Event
An event will contain an Event Type and a Payload

##### Patient Data
This will model the processed data, and can be found in `model/patient_data.rs`.

###### Patient
This will contain the id and the name

###### Case
This will contain a file name for the case

#### Functionality
A way to process the Input into the Patient Data will be implemented, which will contain a map which has a patient as a key, and then a list of cases. As per the assumption, the patient data will arrive before any cases. This will then be iterated through and printed out.


### Using the Program
Simply navigate to the directory of the source files.

1. Build the project

```rust
cargo build
```
2. Run the project, the second command line argument will signify the file path to be used

```rust
cargo run -- data/events.json
```

### Lessons Learnt
* The main takeaway from this was learning how Rust's HashMap implementation finds keys, I learnt the hard way that it used Hash THEN Eq to compare items, so both must be overridden if you are not comparing all items normally. In this case, we only wanted to compare patient ID when comparing patients


### Future Improvements
This solution is by no means perfect, and a number of things could be done.

1. Error Handling. There is minimal error handling, and this could be built upon, especially around parsing events and reading the file.
2. Ability to handle Cases where the patient has not yet been created.
3. Move functionality into it's own library file. Rust's file tree setup is interesting to say the least. This is straightforward though.
4. When a payload with a File is processed, we effectively create a new patient object using that ID to find it (main.rs:29). It would be better to filter on the map to find the key to insert the new case to.



