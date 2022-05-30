use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Record, Response, StdResult, Order};
use cw_storage_plus::Map;
use schemars::_serde_json::Value::Null;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GreetingsResponse, InstantiateMsg, QueryMsg};
use crate::state::{GREETINGS, State, STATE};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SayHello { greeting } => try_say_hello(deps, info, greeting),
    }
}

pub fn try_say_hello(deps: DepsMut, info: MessageInfo, greeting: String) -> Result<Response, ContractError> {
    let result = GREETINGS.save(deps.storage, &info.sender, &greeting.to_string());

    Ok(Response::default())
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGreetings {} => to_binary(&query_greetings(deps)?),
    }
}

fn query_greetings(deps: Deps) -> StdResult<GreetingsResponse> {
    // let greetings: StdResult<Vec<_>> = GREETINGS.range(deps.storage, None, None, Order::Ascending).map(|v| {
    //     let value = v.unwrap();
    //     return Record(value.0.to_string(), value.1);
    // }).collect();
    let greetings: Vec<_> = GREETINGS.range(deps.storage, None, None, Order::Ascending).map(|v| {
        let value = v.unwrap();
        (value.0.as_bytes().to_vec(), value.1)
    }).collect();
    // let greetings: StdResult<Vec<Record>> = GREETINGS.range(deps.storage, None, None, Order::Ascending).collect();
    Ok(GreetingsResponse { greetings })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGreetings {}).unwrap();
        let value: GreetingsResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.greetings.len());
    }

    #[test]
    fn say_hello() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::SayHello { greeting: "Hello Cosmos!".to_string() };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGreetings {}).unwrap();
        let value: GreetingsResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.greetings.len());
    }


    // #[test]
}
