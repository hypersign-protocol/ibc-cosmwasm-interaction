#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, IbcMsg, to_binary, IbcTimeout};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, IbcExecuteMsg, QueryAddressResponse};
use crate::state::ADDRESS_MAP;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:controller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment { channel } => Ok(Response::new()
            .add_attribute("method", "execute_query")
            .add_attribute("channel", channel.clone())
            .add_message(IbcMsg::SendPacket { 
                channel_id: channel,
                data: to_binary(&IbcExecuteMsg::IncrementCount { tx_author: info.sender.into_string()  })?, 
                timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(120)),
            })
        ),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Checks if the input address has made any IBC increment request to Host Contract
        QueryMsg::QueryAddress { addr } => query_address(deps, addr)
    }
}

fn query_address(deps: Deps, addr: String) -> StdResult<Binary> {
    let increment_requests: u64 = ADDRESS_MAP.may_load(deps.storage, addr)?.unwrap_or_default();
    to_binary(&QueryAddressResponse { increment_requests })
}

#[cfg(test)]
mod tests {}
