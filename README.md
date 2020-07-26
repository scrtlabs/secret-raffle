# Secret Raffle

## TESTNET RAFFLE

Post the account you entered with on the rocket.chat channel. We recommend using the account associated with your validator AND adding your validator name with `--memo "name"` when submitting so we can easily identify you. If a winner is selected that was not posted on the rocket.chat, did not have a memo when registering, or is not associated with a validator, a new winner will be selected.

Also, don't spam the contract with random accounts. We know we didn't limit the contract. Thanks:)

The contract address for the testnet is: `TBD`

## Description
This is a simple raffle game. The 'Raffle Host' will deploy an instance of this contract. 

Anyone can join the raffle, by submitting a transaction from their account. Each account can enter only once.

When the raffle host decides to end the raffle, a winner will be chosen at random from all the accounts that registered

## Disclaimer
This is only a usage example, and does not imply on how to correctly and safely use or write `Secret Contracts`. You should always make sure to read and understand `Secret Contract` API's disclaimers and limitations before deploying a contract in production!

## Usage

### As a participant 

#### Join the raffle

To join, you simply submit a `join` transaction, and choose a lucky phrase or number (and keep it secret!). This will be used as entropy for the required randomness.

```bash
secretcli tx compute execute <contract-address> '{ "join": { "phrase": "<write something fun here>" }}' --from account
```

`phrase` is expected to be a string, so choose whatever you want as long as you don't forget to surround it by double quotes
For example:
* right: `"5"` 
* wrong: `5`

#### Did I join?
Check if an address was successfully entered in the raffle
```
secretcli q compute contract-state smart <contract-address> '{"registered": {"address": "<your address>"}}'
```

#### See who won
See who was selected as the winner
```
secretcli q compute contract-state smart <contract-address> '{"winner": {}}'
```

### As a raffle host

### Store the contract on-chain
```bash
secretcli tx compute store contract.wasm.gz --from account --gas auto
```

#### Instantiate contract
```bash
secretcli tx compute instantiate <code_id> '{"seed": "<some long secret here>"}' --label <label> --from account
```

#### End raffle - will select a winner
```bash
secretcli tx compute execute <contract-address> '{ "end_lottery": {} }' --from account
```

For more details, check out the [messages module](https://github.com/enigmampc/secret-raffle/blob/master/src/msg.rs).
