use super::*;

use arbiter_core::bindings::arbiter_token;

pub async fn deploy_initial_contracts(client: Arc<RevmMiddleware>) -> Result<(bindings::weth::WETH<RevmMiddleware>, arbiter_token::ArbiterToken<RevmMiddleware>, arbiter_token::ArbiterToken<RevmMiddleware>)> {
    // Deploy the weth contract
    let weth = bindings::weth::WETH::deploy(client.clone(), ())?
        .send()
        .await?;

    // Deploy the arbiter token x contract
    let arbx = arbiter_token::ArbiterToken::deploy(
        client.clone(),
        (
            ARBITER_TOKEN_X_NAME.to_string(),
            ARBITER_TOKEN_X_SYMBOL.to_string(),
            ARBITER_TOKEN_X_DECIMALS,
        ),
    )?
    .send()
    .await?;

    // Deploy the arbiter token y contract
    let arby = arbiter_token::ArbiterToken::deploy(
        client,
        (
            ARBITER_TOKEN_Y_NAME.to_string(),
            ARBITER_TOKEN_Y_SYMBOL.to_string(),
            ARBITER_TOKEN_Y_DECIMALS,
        ),
    )?
    .send()
    .await?;
    Ok((weth, arbx, arby))
}
