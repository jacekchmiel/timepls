# timepls
Simple tool that converts unix timestamps to human readable date. 

## Features
* Supports second and millisecond precision timestamps
* Accepts json values (newline separated)
* Accepts data in format: `TIMESTAMP OTHER_DATA`, treats `OTHER_DATA` as potential JSON.
* In JSON data replaces all number values labeled with one of keys:
    - `"t"`
    - `"ts"`
    - `"time"`
    - `"timestamp"`
    
    
## Usage
```bash
echo '{"time": 1602052430}' | timepls