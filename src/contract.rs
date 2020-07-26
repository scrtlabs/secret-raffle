use cosmwasm_std::{Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage, QueryResponse, log};
use crate::rand::Prng;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use sha2::{Digest, Sha256};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    // Init msg.item_count items
    let items: Vec<CanonicalAddr> = Vec::default();

    //Create state
    let state = State {
        items,
        contract_owner: env.message.sender,
        seed: msg.seed.as_bytes().to_vec(),
        entropy: Vec::default(),
        start_time: env.block.time,
        winner: CanonicalAddr::default()
    };

    // Save to state
    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::EndLottery {} => end_lottery(deps, env),
        HandleMsg::Join { phrase } => {register(deps, env, phrase)}
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Joined { address } => query_registered(deps, address),
        QueryMsg::Winner {} => query_winner(deps)
    }
}

fn throw_gen_err(msg: String) -> StdError {
    StdError::GenericErr {
        msg,
        backtrace: None,
    }
}

fn register<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    phrase: String
) -> StdResult<HandleResponse> {
    let mut state = config(&mut deps.storage).load()?;

    if state.items.contains(&env.message.sender) {
        return Err(throw_gen_err(format!("Address {} is already registered", deps.api.human_address(&env.message.sender)?) ));
    }

    state.items.push(env.message.sender.clone());
    state.entropy.extend(phrase.as_bytes());
    state.entropy.extend(env.message.sender.as_slice().to_vec());
    state.entropy.extend(env.block.chain_id.as_bytes().to_vec());
    state.entropy.extend(&env.block.height.to_be_bytes());
    state.entropy.extend(&env.block.time.to_be_bytes());

    state.entropy = Sha256::digest(&state.entropy).as_slice().to_vec();

    // Save state
    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

fn query_registered<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    let addr = deps.api.canonical_address(&address)?;

    if state.items.contains(&addr) {
        Ok(Binary(Vec::from(format!("{} is registered", address))))
    } else {
        Ok(Binary(Vec::from(format!("{} is not registered", address))))
    }
}

fn query_winner<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    if state.winner != CanonicalAddr::default() {
        let winner_readable = deps.api.human_address(&state.winner)?;
        Ok(Binary(Vec::from(format!("{} was the winner", winner_readable))))
    } else {
        Ok(Binary(Vec::from(format!("Winner not selected yet!"))))
    }
}

fn end_lottery<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    // add this if you don't want to allow choosing an alternative winner
    // if state.winner != CanonicalAddr::default() {
    //     // game already ended
    //     return Ok(HandleResponse::default());
    // }

    if env.message.sender != state.contract_owner {
        // game already ended
        return Err(throw_gen_err("You cannot trigger lottery end unless you're the owner!".to_string()));
    }
    // let contract_addr: HumanAddr = deps.api.human_address(&env.contract.address)?;

    // this way every time we call the end_lottery function we will get a different result. Plus it's going to be pretty hard to
    // predict the exact time of the block, so less chance of cheating
    state.entropy.extend_from_slice(&env.block.time.to_be_bytes());

    let mut rng: Prng = Prng::new(&state.seed, &state.entropy);

    let winner = rng.select_one_of(state.items.clone().into_iter());

    if winner.is_none() {
        return Err(throw_gen_err(format!("Fucking address is empty wtf")));
    }

    state.winner =  winner.unwrap().clone();

    config(&mut deps.storage).save(&state)?;

    let winner_readable = deps.api.human_address(&state.winner)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("winner", format!("{}", winner_readable))],
        data: None,
    })
}
