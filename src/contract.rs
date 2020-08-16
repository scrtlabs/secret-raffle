use cosmwasm_std::{Api, Binary, CanonicalAddr, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier, StdError, StdResult, Storage, QueryResponse, log};
use crate::rand::Prng;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use sha2::{Digest, Sha256};

const WHITELISTED_ADDRESSES: &'static [&'static str] = &[
    "secret1fdrcpf7c6ha0say3r8hsxydmgzstqsg93lq9l9",
    "secret1gsfg8u9clrdqrxudlzysyyd23z6lkk6pefsxcw",
    "secret19nngv602x8ydc9fz8wq4kg9wv7we0rl2xrnygg",
    "secret1gm7gjydzetjqtvymqczrxzl8q9pe8a3e4c5pxh",
    "secret18dtujxnnknsj7l9fvmm67jnavj26f24zdetaka",
    "secret182dg8p7nshdjjt5sypcx6hw9p8vlwqpn6x4fmn",
    "secret1cw5vnc2v0g5p3yuacndhj0sjeesg8z79e7f00g",
    "secret1hkpylyrxnqrkk63j4r8g24tr7cuqlltljfffl4",
    "secret16zml94aa8amzt8hz44hp5s6ztxkqycu27yr7um",
    "secret17s9qz9pg95demg09wn3fdqv2ahvdvntzu3rh7q",
    "secret1m5tvx9qnppuw244yx4slzsd8ap93x0atx7pszx",
    "secret1wx6c2jugsnqd6cjmu2rk434qn07dkj2x588jx4",
    "secret1t4crj8yjgwe0dmv2fzvf4jlc6se0amz07afs7c",
    "secret17tnsluzm08yc72zp2xearkes5vnjg77qtutzlm",
    "secret1u4qnyv9vaaacanpmfg88dgchhrqk0rd2ztppdu",
    "secret13hzlwg42twz3jtwn768k9m4yunf74jzspxyx62",
    "secret1swe5tnya27syjfvye5qxl4u8tc66g73rk8k225",
// from validators
    "secret1zm99euzceq09k6sjltzq4mgyyxce8m4t8ujq5z",
    "secret1zuu95rhwpdyp0n3m4tl9p00crmd3ndehy84hsp",
    "secret1yurrvjtg39psgrzq7jdptelku3gvf4uee3yjyv",
    "secret19zljvxd8xn8zu33mdcxwmtytykdu0zlt4myunf",
    "secret19va9hdw97susykhmp86m6schqsfk567eserufr",
    "secret196f7kcr2c6w8gyfya5mt49vsj3qk7tpxd9gu5s",
    "secret18rxhudxdx6wen48rtnrf4jv5frf47qa9ws2ju3",
    "secret18u76un3s0wjunglgshevc9c094prjzaxmplvr4",
    "secret1gr0lrnryj29247gq42zlet0vd0ry8xf3e70clu",
    "secret1gddkdjvmkp0ytdxkks4hwyppcz2u95kuzyu7gp",
    "secret1ffnre67arhe6rc33e9znf24vnlwz7vwyevf7jn",
    "secret12svqw4d4j0gj2s4pn4zuanscdq02phytpv5pvr",
    "secret1237vq64caqmlz5scgnt8698h0ryd0xzystmeek",
    "secret124sa0c2xveqqgvghwnzkjq7xpj0dks3tum99lz",
    "secret1ty03hvhq742n8y9zj0jlaan85dqxncav7w8laz",
    "secret1tttrrpkh82r289r70ge5w5uh73j80fq7wc6g73",
    "secret1tmempyc2qqqnpe6gm9lmppamws5krnw7njd6h0",
    "secret1v3wrcgr9ndmad42llq35zhg2hgzh5z5936tk8d",
    "secret1dp8q5udnfls4yrclhckg470d7gh6gs7veervtp",
    "secret1d98c200ksp3gumktpzk3pdc46dh5xcp7ca429q",
    "secret1d6lxy8g3hzv5cjec3teefg2gwx8jlsjh8yv0ln",
    "secret1wg897ma3pgewzgq7ezyt3tfw5d2l88rhn9nc93",
    "secret10ryksn3swnul6k8kgywpxvav0s6dwpaff9p3us",
    "secret109utfwe06xf0khh4g4uxc227mr9s6duzwyh2za",
    "secret10cqa6t8uug20h5tfqkejsv9zsfdzuz53t3n8e7",
    "secret1s2lmuhlmjll78pkgwepwm0vfwvs36ymrtsvlqh",
    "secret1ss8feungngpnhetcs7gh33jwwr6ur3empmf54c",
    "secret13x0lk7t4ncuuvm3vnkgmycm54hz0e2w68gvzz8",
    "secret13glfwuj8sdta5f7ap82qtx95s5u4p4psm5unwm",
    "secret13tkhntnyg9xv3ltdy0mgfmnud4gktz9y0pqy9a",
    "secret1jzrfydf9a0v4ame8feh33k9en7mklmh9u9p30l",
    "secret1j0es0a8tucld9vz3szgewcph4ad00ezlet09vd",
    "secret1jj7z8m3zknvgse3nhd66rqw9f4c7y5ggkaxvvu",
    "secret15zu5l3nn5lf6yn9yz7u4jkt2u486fjcqu53e2d",
    "secret15f5xdvh6vfvqp0e05m79y8yvmwxym4mtsrztyn",
    "secret155j7sa7tew2gswajq3727v5s55n2aganpnawpt",
    "secret1kp0nlfw8e4m4rwkn85w2p0nxntp3cu2vzjstju",
    "secret1k3p95drqwc843nz23arzpt3f09afcpl34xur88",
    "secret1kld5t3xd8vl853vwah8vpkdrvzyfa0gx74uee8",
    "secret1hht2v8vl49ezh9vjfaehkxf3aneulmecdwn39t",
    "secret1czyuyrltpy0n9haancmp4tx4sp4thnmu6kwpta",
    "secret1ctjggljlzh3n52nawcnrgfsdewvkj4qcfxjj7g",
    "secret1cna2tedzx025zj6xcnlj6nw37gzde6yqjnrwkf",
    "secret1ch4wpcgvlm84m8d95x9lrck84dwk7jwpzhnenc",
    "secret1ezyzzs2xluw2fffdvewhh0tqdtj499zs8l8al5",
    "secret1exx0mp9jxdy86sluz4j4kwktmlawu42ygm5k97",
    "secret1ev2vlvaqcvyg6dgjrx9u3lful2wfc3z38sv0ve",
    "secret16xum37xp3pt6jxwy8hyhylhnfju7z6wwwn4jpz",
    "secret16skft0g2wlhtnacjwpkjullrge3ywqzugk49gg",
    "secret16nackgz33wl0vtmr3wdmu3ux6qpynrlhre6sq4",
    "secret1mpe3g9ml8zw0wqf9xmdx7g8y669xggp3qcfzta",
    "secret1m29qmes5p8dfkgyjy5x6ynz9qkfh5y4qmm80ca",
    "secret1mnq2e7arkk0hlp9xh7mg2d3muq95ldy62vjsdd",
    "secret1uqgazegrexhcvaf5na0fn5xnxwhusy94uc9fe6",
    "secret1uwasa9y5cu0hkx969hvjlh3mrxxeulg34a2r4g",
    "secret1u5q6jh3rjdljwjrnypfdj9qfwyfx40r0eavkwm",
    "secret1ulrcxlmfgdythq23nmwg03m7vaw8re5gatskuz",
    "secret1lr5fnce3urg5s4rjehsns2sdlwttl3hgc8xk60",
    "secret1lv3qaujtmhzwu8ra0jzplsqhlnnwwn0qe40kas",
    "secret1lsajjewzqh3ulkdzacwx09g33c5desc4rj3vy5",
    "secret1lavkrpws23gzmnryfr5kmstu7sasp9sm53nt3h",];

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let i = WHITELISTED_ADDRESSES.iter();

    let whitelist: Vec<CanonicalAddr> = i.map(|s| deps.api.canonical_address(&HumanAddr(s.to_string())).unwrap()).collect();

    // Init msg.item_count items
    let items: Vec<CanonicalAddr> = Vec::default();
    //Create state
    let state = State {
        items,
        contract_owner: env.message.sender,
        seed: msg.seed.as_bytes().to_vec(),
        entropy: Vec::default(),
        start_time: env.block.time,
        winner: CanonicalAddr::default(),
        winner1: CanonicalAddr::default(),
        winner2: CanonicalAddr::default(),
        winner3: CanonicalAddr::default(),
        whitelist
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
        HandleMsg::EndLottery { winner_to_select } => end_lottery(deps, env, winner_to_select),
        HandleMsg::Join { phrase } => {register(deps, env, phrase)},
        HandleMsg::AddToWhitelist { addresses } => {add_to_whitelist(deps, env, addresses)}
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Joined { address } => query_registered(deps, address),
        QueryMsg::Winner {} => query_winner(deps),
        QueryMsg::Whitelisted {address} => query_whitelist(deps, address),
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

    if !state.whitelist.contains(&env.message.sender) {
        return Err(throw_gen_err(format!("Address {} is not whitelisted. You may request this address to be added by asking on the phase-2-testnet rocket.chat channel", deps.api.human_address(&env.message.sender)?) ));
    }

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

    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(Binary(Vec::from("Registered successfully!")))
    })
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

fn query_whitelist<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    address: HumanAddr,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    let addr = deps.api.canonical_address(&address)?;

    if state.whitelist.contains(&addr) {
        Ok(Binary(Vec::from(format!("{} is whitelisted", address))))
    } else {
        Ok(Binary(Vec::from(format!("{} is not whitelisted", address))))
    }
}

fn query_winner<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<QueryResponse> {
    let state = config_read(&deps.storage).load()?;

    let w1 = if state.winner1 != CanonicalAddr::default() {
        deps.api.human_address(&state.winner1)?.to_string()
    } else {
        "not selected".to_string()
    };

    let w2 = if state.winner2 != CanonicalAddr::default() {
        deps.api.human_address(&state.winner2)?.to_string()
    } else {
        "not selected".to_string()
    };

    let w3 = if state.winner3 != CanonicalAddr::default() {
        deps.api.human_address(&state.winner3)?.to_string()
    } else {
        "not selected".to_string()
    };

    if state.winner1 != CanonicalAddr::default() || state.winner2 != CanonicalAddr::default() || state.winner3 != CanonicalAddr::default() {
        Ok(Binary(Vec::from(format!("1st place: {}\n2nd place: {}\n3rd place: {}", w1, w2, w3))))
    } else {
        Ok(Binary(Vec::from(format!("Winner not selected yet!"))))
    }
}

fn add_to_whitelist<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    mut addresses: Vec<HumanAddr>,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    if env.message.sender != state.contract_owner {
        return Err(throw_gen_err("You cannot trigger lottery end unless you're the owner!".to_string()));
    }

    let i = addresses.iter_mut();
    for x in i {
        state.whitelist.push(deps.api.canonical_address(x)?)
    }

    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

fn end_lottery<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    winner_to_select: u8,
) -> StdResult<HandleResponse> {
    // TODO Check if contract has expired

    let mut state = config(&mut deps.storage).load()?;

    // add this if you don't want to allow choosing an alternative winner
    // if state.winner != CanonicalAddr::default() {
    //     // game already ended
    //     return Ok(HandleResponse::default());
    // }

    if env.message.sender != state.contract_owner {
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

    let unwrapped = winner.unwrap();

    match winner_to_select {
        1 => {
            state.winner1 =  (&unwrapped).clone();
        },
        2 => {
            state.winner2 =  (&unwrapped).clone();
        },
        3 => {
            state.winner3 =  (&unwrapped).clone();
        },
        _ => {
            return Err(throw_gen_err(format!("bad winner selection")));
        }
    }

    config(&mut deps.storage).save(&state)?;

    let winner_readable = deps.api.human_address(&unwrapped)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![log("winner", format!("{}", winner_readable))],
        data: None,
    })
}
