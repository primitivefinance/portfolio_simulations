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