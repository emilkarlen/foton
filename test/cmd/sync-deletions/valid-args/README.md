Tests uses the tool [Exactly](https://github.com/emilkarlen/exactly)

# Prerequisites

## Build SUT

First build the SUT (see top dir).

## Generate tests

Many test cases are generated using tools in cases-gen/.

Generate these tests using

    $ make all

# Run tests

    $ exactly suite .
