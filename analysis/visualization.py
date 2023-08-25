import matplotlib.pyplot as plt
import seaborn as sns

def plot_prices(df):
    # Initialize the Seaborn plot
    sns.set_theme(style="darkgrid")

    # Plot multiple line plots
    plt.figure(figsize=(10, 6))
    sns.lineplot(x=df.index, y='liquid_exchange_prices', data=df, label='Liquid Exchange Prices')
    sns.lineplot(x=df.index, y='portfolio_prices', data=df, label='Portfolio Prices')

    # Add title and labels
    plt.title('Prices Over Time')
    plt.xlabel('Index')
    plt.ylabel('Price')
    plt.legend()

    # Show the plot
    plt.show()

def plot_arbitrageur_relative_balances(df):
    # Initialize the Seaborn plot
    sns.set_theme(style="darkgrid")

    # Plot multiple line plots
    plt.figure(figsize=(10, 6))
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_x', data=df, label='Relative Arbitrageur Balances X')
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_y', data=df, label='Relative Arbitrageur Balances Y')

    # Add title and labels
    plt.title('Arbitrageur Balances Over Time')
    plt.xlabel('Index')
    plt.ylabel('Balance in wei')
    plt.legend()

    # Show the plot
    plt.show()

def plot_portfolio_reserves(df):
    # Initialize the Seaborn plot
    sns.set_theme(style="darkgrid")

    # Plot multiple line plots
    plt.figure(figsize=(10, 6))
    sns.lineplot(x=df.index, y='portfolio_reserves_x', data=df, label='Portfolio Reserves X')
    sns.lineplot(x=df.index, y='portfolio_reserves_y', data=df, label='Portfolio Reserves Y')

    # Add title and labels
    plt.title('Portfolio Reserves Over Time')
    plt.xlabel('Index')
    plt.ylabel('Reserve in wei')
    plt.legend()

    # Show the plot
    plt.show()