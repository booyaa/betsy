# betsy v0.1.3

A [AppVeyor](https://ci.appveyor.com) client.

# Install

`cargo install betsy`

# Usage

Each call requires your APPVEYOR [token](https://ci.appveyor.com/api-token). You can either invoke it using the following: `APPVEYOR=your_appveyor_token betsy list`

Or store the variable in `.env` file in your home directory.

```
USAGE:
    betsy [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    disable    Disable a project
    enable     Enable a project
    help       Prints this message or the help of the given subcommand(s)
    list       Lists projects
```

# To do

Can be found [here](https://github.com/booyaa/betsy/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement).

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
