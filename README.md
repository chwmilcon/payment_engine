# Payment Engine - Toy Payment Processor
* Project created with a cookiecutter template that provides basic Cargo.toml,
  main.rs with logger and clap (filename and debug).

## USAGE:
Usage: payment_engine [OPTIONS] <NAME>

Arguments:
  <NAME>  File to process

Options:
  -d, --debug                Turn on debug logging
      --stop-on-error        Should we stop everything when there's a processing error? (Default:false)
      --logfile <LOGFILE>    file to write log messages into.
      --statelog <STATELOG>  File to dump the internal ledger, all data
  -h, --help                 Print help
  -V, --version              Print version

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

* Assuming that code should continue with a single bad row,
  but --stop-on-error can cause it to stop on error.

* In a *real* production system I would assume that gateways would attach to
  end customer. The gateway would take whatever external protocol is being used
  and translate it into what's being used internally and validate it. The
  payment processor would be load balanced by the gateways. In this architecture
  it would be expected that the payment processors would be sharded, keeping
  their state local to each shard, which a hot standby also receiving the same
  traffic. If the processing was not high performance then the payment
  processors could have their state sync'd with a common database.

* This sample is not setup to be multithreaded currently. To maximize the
  hardware it's being run on I would expect it to be.
  
* I've done most of the testing of the systems with unit tests and
  integration tests (in tests/). There are two example data sets in a
  directory called `sample_data`. I only used to test running with
  file I/O and stdio output.


# TODO:
- [ ] More formatted data tests
