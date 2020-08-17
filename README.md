# Secret Raffle

## TESTNET RAFFLE

Post the account you entered with on the rocket.chat channel. We recommend using the account associated with your validator AND adding your validator name with `--memo "name"` when submitting so we can easily identify you. If a winner is selected that was not posted on the rocket.chat, did not have a memo when registering, or is not associated with a validator, a new winner will be selected.

The contract address for the testnet is: `secret1jjwx5eyaz4e4h7w0jyz0dad8e2wm73d8dwkr8c` or label `raffle`

## Description
This is a simple raffle game. The 'Raffle Host' will deploy an instance of this contract. 

Anyone can join the raffle, by submitting a transaction from their account. Each account can enter only once.

When the raffle host decides to end the raffle, a winner will be chosen at random from all the accounts that registered

## Usage

### As a participant 

#### Join the raffle

To join, you simply submit a `join` transaction, and choose a lucky phrase or number (and keep it secret!). This will be used as entropy for the required randomness.

```bash
secretcli tx compute execute '{ "join": { "phrase": "<write something fun here>" }}' --from account --label raffle
```

`phrase` is expected to be a string, so choose whatever you want as long as you don't forget to surround it by double quotes
For example:
* right: `"5"` 
* wrong: `5`

#### Am I whitelisted?
Check if an address is whitelisted for the raffle
```
secretcli q compute query <contract-address> '{"whitelisted": {"address": "<your address>"}}'
```

#### Did I join?
Check if an address was successfully entered in the raffle
```
secretcli q compute query <contract-address> '{"joined": {"address": "<your address>"}}'
```

#### See who won
See who was selected as the winner
```
secretcli q compute query <contract-address> '{"winner": {}}'
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
secretcli tx compute execute <contract-address> '{ "end_lottery": {"winner_to_select": <1-3>} }' --from account
```

For more details, check out the [messages module](https://github.com/enigmampc/secret-raffle/blob/master/src/msg.rs).

### Troubleshooting 

All transactions are encrypted, so if you want to see the error returned by a failed transaction, you need to use the command

`secretcli q compute tx <TX_HASH>`
