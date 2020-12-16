/*!
# frontend description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
frontend = "0.1"
```

*/

use actix_web_static_files;

use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));