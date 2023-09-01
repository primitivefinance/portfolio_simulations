pragma solidity ^0.8.14;

import "portfolio/libraries/SwapLib.sol";

interface PortfolioLike {
    function swap(Order memory order)
        external
        returns (uint64, uint256, uint256);
}

interface ExchangeLike {
    function swap(address token, uint256 amount) external;
}

interface TokenLike {
    function transferFrom(address, address, uint256) external;
    function transfer(address, uint256) external;
    function approve(address, uint256) external;
    function balanceOf(address) external view returns (uint256);
}

contract AtomicArb {
    function execute(
        address token,
        address otherToken,
        address portfolio,
        address exchange,
        Order memory order,
        uint256 exchangeInput
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(token).transferFrom(msg.sender, address(this), exchangeInput);
        // approve exchange to spend tokens
        TokenLike(token).approve(exchange, exchangeInput);
        // exchange y for x
        ExchangeLike(exchange).swap(token, exchangeInput);

        uint256 x_balance = TokenLike(otherToken).balanceOf(address(this));

        // send x to portfolio
        TokenLike(otherToken).transfer(portfolio, x_balance);

        uint256 max_iter = 10;
        uint256 i;
        bool success;
        uint256 output = order.output;
        do {
            // swap on portfolio
            try PortfolioLike(portfolio).swap(order) {
                success = true;
            } catch {
                output = output * 999 / 1000;
            }

            i++;
        } while (i < max_iter && !success);

        require(success, "Swap not successful");

        uint256 y_balance = TokenLike(token).balanceOf(address(this));

        // send y to arbitrageur if we have tokens
        if (y_balance > 0) TokenLike(token).transfer(msg.sender, y_balance);

        require(y_balance > exchangeInput, "Not profitable");
    }
}
