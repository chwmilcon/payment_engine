# Payment Engine - Toy Payment Processor
* Project created with a cookiecutter template that provides basic Cargo.toml,
  main.rs with logger and clap (filename and debug).

## Assumptions
* Amounts in transactions that are more than 4 digits of precision are
  considered invalid. This is assuming that there was a data groomer ahead
  of the processor and this shouldn't be.

  With that said full data validation would be better suited to being off the
  money loop in a gateway. The data put into a structure that doesn't need to
  be parsed and the processor engine given that. In this toy we are reading
  the ascii data and doing some basic validation on it.

## TODOs
* Clean up TODOs before uploading to github
* Make command line argument to have or not have headers