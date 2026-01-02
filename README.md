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

* Treating "withdraw" and "withdrawl" as same enum.

* Transactions in a real application might be broken up into derived structures
  containing different information based on transaction type and/or if we
  were processing grpc messages. In this simple example not going to derive
  transactions for each transaction type.

* Assuming that code should continue with a single bad row, but will add
  command line flag to override this behavior.

## TODOs
- [ ] Clean up TODOs before uploading to github
- [ ] Make transaction history
- [ ] Test cases for keep_going = true
