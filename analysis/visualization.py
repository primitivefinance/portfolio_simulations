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

def plot_lp_portfolio_values(ax, df):
    sns.lineplot(x=df.index, y='lp_portfolio_value', data=df, label='LP Portfolio Value', ax=ax)
    ax.set_title('LP Portfolio Values Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Value in wei')
    ax.legend()

def plot_arbitrageur_portfolio_values(ax, df):
    sns.lineplot(x=df.index, y='arbitrageur_portfolio_value', data=df, label='Arbitrageur Portfolio Value', ax=ax)
    ax.set_title('Arbitrageur Portfolio Values Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Value in wei')
    ax.legend()

def plot_lp_fees(ax, df):
    sns.lineplot(x=df.index, y='accumulated_lp_fees', data=df, label='Accumulated LP Fees', ax=ax)
    ax.set_title('Accumulated LP Fees Over Time')
    ax.set_xlabel('Index')
    ax.set_ylabel('Fees in wei')
    ax.legend()

def plot_all(df, filename):
    sns.set_theme(style="darkgrid")
    fig, axes = plt.subplots(6, 1, figsize=(16, 16))

    plot_prices(axes[0], df)
    plot_arbitrageur_relative_balances(axes[1], df)
    plot_portfolio_reserves(axes[2], df)
    plot_lp_portfolio_values(axes[3], df)
    plot_arbitrageur_portfolio_values(axes[4], df)
    plot_lp_fees(axes[5], df)

    plt.savefig(filename)
