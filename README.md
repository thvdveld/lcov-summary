# lcov-summary

lcov-summary is a tool that summarizes the content of an lcov file.
It is also possible to show the diff of two lcov files.


## Installation

Use `cargo` to install lcov-summary:
```bash
cargo install lcov-summary
```

## Usage 

Using the following command, the summary of the lcov file is printed to stdout.
```bash
lcov-summary lcov.info
```

The output might look like:
```txt
                        Lines                  Functions
              │  Hit    Total  H/T     │  Hit   Total  H/T
 ./lcov.info  │  23662  30141  78.50%  │  2675   3630  73.69%
```

Using the `--full` flag, the coverage is shown for every file:

```bash
lcov-summary --full lcov.info
```


If two files are passed to lcov-summary, then the diff of those files is printed to stdout.
```bash
lcov-summary lcov-master.info lcov-feature.info
```

The output might look like:
```txt
                               Lines                    Functions
                   │  Hit     Total   H/T      │  Hit    Total  H/T
 lcov-master.info  │   22394   28322   79.07%  │   2524   3401   74.21%
        lcov.info  │   23662   30141   78.50%  │   2675   3630   73.69%
             diff  │  + 1268  + 1819  - 0.56%  │  + 151  + 229  - 0.52%
```

The following command is not yet implemented, but it would show the diff, 
only for the files that actually have different coverage:
```bash
lcov-summary --full lcov-master.info lcov-feature.info
```
