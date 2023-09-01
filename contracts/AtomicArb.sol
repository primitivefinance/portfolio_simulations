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
    function execute_x_to_y(
        address x_token,
        address y_token,
        address portfolio,
        address exchange,
        Order memory order,
        uint256 exchangeInput
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(y_token).transferFrom(msg.sender, address(this), exchangeInput);
        // approve exchange to spend tokens
        TokenLike(y_token).approve(exchange, exchangeInput);
        // exchange y for x
        ExchangeLike(exchange).swap(y_token, exchangeInput);

        uint256 x_balance = TokenLike(x_token).balanceOf(address(this));

        // send x to portfolio
        TokenLike(x_token).transfer(portfolio, x_balance);

        uint256 max_iter = 20;
        uint256 i;
        bool success;
        uint256 output = order.output;
        do {
            // swap on portfolio
            try PortfolioLike(portfolio).swap(order) {
                success = true;
            } catch {
                output = output * 9999 / 10000;
            }

            i++;
        } while (i < max_iter && !success);

        require(success, "Swap not successful");

        uint256 y_balance = TokenLike(y_token).balanceOf(address(this));

        // send y to arbitrageur if we have tokens
        if (y_balance > 0) TokenLike(y_token).transfer(msg.sender, y_balance);

        require(y_balance > exchangeInput, "Not profitable");
    }

    function execute_y_to_x(
        address x_token,
        address y_token,
        address portfolio,
        address exchange,
        Order memory order
    ) external {
        // pull in tokens from arbitrageur
        TokenLike(y_token).transferFrom(msg.sender, address(this), order.input);

        // approve portfolio to spend tokens
        TokenLike(y_token).approve(exchange, order.input);

        uint256 y_balance = TokenLike(y_token).balanceOf(address(this));

        // send y to portfolio
        TokenLike(y_token).transfer(portfolio, y_balance);

        uint256 max_iter = 20;
        uint256 i;
        bool success;
        uint256 output = order.output;
        do {
            // swap on portfolio
            try PortfolioLike(portfolio).swap(order) {
                success = true;
            } catch {
                output = output * 9999 / 10000;
            }

            i++;
        } while (i < max_iter && !success);

        require(success, "Swap not successful");

        // approve exchange to spend tokens
        TokenLike(x_token).approve(exchange, output);
        // exchange x for y
        ExchangeLike(exchange).swap(x_token, output);

        uint256 new_y_balance = TokenLike(y_token).balanceOf(address(this));

        // send y to arbitrageur if we have tokens
        if (new_y_balance > 0) TokenLike(y_token).transfer(msg.sender, y_balance);

        require(new_y_balance > y_balance, "Not profitable");
    }

}
