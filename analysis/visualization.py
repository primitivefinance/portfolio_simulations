import matplotlib.pyplot as plt
import seaborn as sns

# Set the style and color palette of Seaborn
sns.set_theme(style="whitegrid")
sns.set_palette("tab10")

def customize_plot(ax, title, xlabel, ylabel):
    ax.set_title(title, fontsize=16)
    ax.set_xlabel(xlabel, fontsize=14)
    ax.set_ylabel(ylabel, fontsize=14)
    ax.legend(fontsize=12)
    ax.grid(True)

def plot_prices(ax, df):
    sns.lineplot(x=df.index, y='liquid_exchange_prices', data=df, label='Liquid Exchange Prices', ax=ax, linewidth=2)
    sns.lineplot(x=df.index, y='portfolio_prices', data=df, label='Portfolio Prices', ax=ax, linewidth=2)
    customize_plot(ax, 'Prices Over Time', 'Index', 'Price')

def plot_arbitrageur_relative_balances(ax, df):
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_x', data=df, label='Relative Arbitrageur Balances X', ax=ax, linewidth=2)
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_y', data=df, label='Relative Arbitrageur Balances Y', ax=ax, linewidth=2)
    customize_plot(ax, 'Arbitrageur Balances Over Time', 'Index', 'Balance in wei')

def plot_portfolio_reserves(ax, df):
    sns.lineplot(x=df.index, y='portfolio_reserves_x', data=df, label='Portfolio Reserves X', ax=ax, linewidth=2)
    sns.lineplot(x=df.index, y='portfolio_reserves_y', data=df, label='Portfolio Reserves Y', ax=ax, linewidth=2)
    customize_plot(ax, 'Portfolio Reserves Over Time', 'Index', 'Reserve in wei')

def plot_lp_portfolio_values(ax, df):
    sns.lineplot(x=df.index, y='lp_portfolio_value', data=df, label='LP Portfolio Value', ax=ax, linewidth=2)
    customize_plot(ax, 'LP Portfolio Values Over Time', 'Index', 'Value in wei')

def plot_arbitrageur_portfolio_values(ax, df):
    sns.lineplot(x=df.index, y='arbitrageur_portfolio_value', data=df, label='Arbitrageur Portfolio Value', ax=ax, linewidth=2)
    customize_plot(ax, 'Arbitrageur Portfolio Values Over Time', 'Index', 'Value in wei')

def plot_lp_fees(ax, df):
    sns.lineplot(x=df.index, y='accumulated_lp_fees_x', data=df, label='Accumulated LP Fees X', ax=ax, linewidth=2)
    sns.lineplot(x=df.index, y='accumulated_lp_fees_y', data=df, label='Accumulated LP Fees Y', ax=ax, linewidth=2)
    customize_plot(ax, 'Accumulated LP Fees Over Time', 'Index', 'Fees in wei')

def plot_arbitrageur_balances_and_values(ax, df):
    # Plot the balances
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_x', data=df, label='Relative Arbitrageur Balances X', ax=ax, linewidth=2, alpha=0.5, legend=False)
    sns.lineplot(x=df.index, y='arbitrageur_relative_balances_y', data=df, label='Relative Arbitrageur Balances Y', ax=ax, linewidth=2, alpha=0.5, legend=False)

    # Create a second y-axis to plot the portfolio value
    ax2 = ax.twinx()
    sns.lineplot(x=df.index, y='arbitrageur_portfolio_value', data=df, label='Arbitrageur Portfolio Value', ax=ax2, linewidth=2, color='purple', legend=False)

    # Customize the plot
    ax.set_title('Arbitrageur Balances and Portfolio Values Over Time', fontsize=16)
    ax.set_xlabel('Index', fontsize=14)
    ax.set_ylabel('Balance in wei', fontsize=14)
    ax.grid(True)

    ax2.set_ylabel('Portfolio Value in wei', fontsize=14)

    # Combine legends from both axes
    lines, labels = ax.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax2.legend(lines + lines2, labels + labels2, loc=0, fontsize=12)

def plot_portfolio_balances_and_values(ax, df):
    # Plot the balances
    sns.lineplot(x=df.index, y='portfolio_reserves_x', data=df, label='Portfolio Reserves X', ax=ax, linewidth=2, alpha=0.5, legend=False)
    sns.lineplot(x=df.index, y='portfolio_reserves_y', data=df, label='Portfolio Reserves Y', ax=ax, linewidth=2, alpha=0.5, legend=False)

    # Create a second y-axis to plot the portfolio value
    ax2 = ax.twinx()
    sns.lineplot(x=df.index, y='lp_portfolio_value', data=df, label='LP Portfolio Value', ax=ax2, linewidth=2, color='purple', legend=False)

    # Customize the plot
    ax.set_title('Portfolio Balances and LP Portfolio Values Over Time', fontsize=16)
    ax.set_xlabel('Index', fontsize=14)
    ax.set_ylabel('Balance in wei', fontsize=14)
    ax.grid(True)

    ax2.set_ylabel('Portfolio Value in wei', fontsize=14)

    # Combine legends from both axes
    lines, labels = ax.get_legend_handles_labels()
    lines2, labels2 = ax2.get_legend_handles_labels()
    ax2.legend(lines + lines2, labels + labels2, loc=0, fontsize=12)


def plot_all(df, filename):
    fig, axes = plt.subplots(4, 1, figsize=(16, 20))

    plot_prices(axes[0], df)
    plot_arbitrageur_balances_and_values(axes[1], df)
    plot_portfolio_balances_and_values(axes[2], df)
    plot_lp_fees(axes[3], df)

    plt.tight_layout()  # Adjusts subplot params for better layout
    plt.savefig(filename)
