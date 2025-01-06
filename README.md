# SmarDex Ecosystem Open-Source Libraries

Open-source Solidity libraries used in the SmarDex ecosystem.

## Installation

To install with [**Foundry**](https://github.com/foundry-rs/foundry):

```sh
forge soldeer install @smardex-solidity-libraries~1.0.0
```

To install with [**Hardhat**](https://github.com/nomiclabs/hardhat):

```sh
npm install @smardex/solidity-libraries
```

## Contributing

If you want to contribute to this repository, create an issue explaining your problem or which improvement you would like to see added first. Mention that you are willing to open a pull request to solve the issue if that's the case.

Then, wait for maintainers to state on what to do with your issue.

If you contribute, any change of code must be thoroughly tested and requires one or more code reviews depending on its complexity.

## Contracts

```ml
src
└─ HugeUint — "Uint512 implementation with basic math operations"
```

## Safety

The smart contracts and libraries in this repository are **experimental** and are provided **as is**.

While audits were conducted on projects implementing code from this repository, we cannot guarantee that no side effects can emerge when used with other code, or that it will not break with future Solidity versions.

## Acknowledgements

This repository is inspired by or directly modified from many sources, primarily:

* [Solady](https://github.com/Vectorized/solady)
* [OpenZeppelin](https://github.com/OpenZeppelin/openzeppelin-contracts)