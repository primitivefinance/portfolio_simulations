import matplotlib.pyplot as plt
import seaborn as sns

def plot_prices(ax, df):
    sns.lineplot(x=df.index, y='liquid_exchange_prices', data=df, label='Liquid Exchange Prices', ax=ax)
    sns.lineplot(x=df.index, y='portfolio_prices', data=df, label='Portfolio Prices', ax=ax)
    ax.set_title('Prices Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Price')
    ax.legend()

def plot_arbitrageur_relative_balances(ax, df):
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_x', data=df, label='Relative Arbitrageur Balances X', ax=ax)
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_y', data=df, label='Relative Arbitrageur Balances Y', ax=ax)
    ax.set_title('Arbitrageur Balances Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Balance in wei')
    ax.legend()

def plot_portfolio_reserves(ax, df):
    sns.lineplot(x=df.index, y='portfolio_reserves_x', data=df, label='Portfolio Reserves X', ax=ax)
    sns.lineplot(x=df.index, y='portfolio_reserves_y', data=df, label='Portfolio Reserves Y', ax=ax)
    ax.set_title('Portfolio Reserves Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Reserve in wei')
    ax.legend()

def plot_all(df, filename):
    sns.set_theme(style="darkgrid")
    fig, axes = plt.subplots(3, 1, figsize=(10, 18))

    plot_prices(axes[0], df)
    plot_arbitrageur_relative_balances(axes[1], df)
    plot_portfolio_reserves(axes[2], df)

    plt.savefig(filename)
