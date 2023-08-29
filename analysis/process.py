import pandas as pd
from decimal import Decimal
import glob

def import_wad_csv(path):
    df = pd.read_csv(path, dtype=str)
    df = df.applymap(to_decimal)
    df = create_arbitrageur_relative_balances(df)
    df = create_liquid_exchange_relative_reserves(df)
    df = compute_portfolio_values(df)
    df = compute_accumulated_fees(df)
    return df

def import_all_csvs(folder_path):
    all_files = glob.glob(folder_path + "/*.csv")
    dfs = []
    for filename in all_files:
        df = pd.read_csv(filename, dtype=str)
        df = df.applymap(to_decimal)
        df = create_arbitrageur_relative_balances(df)
        df = create_liquid_exchange_relative_reserves(df)
        df = compute_portfolio_values(df)
        df = compute_accumulated_fees(df)
        dfs.append(df)
    return dfs


def to_decimal(x):
    return Decimal(x).scaleb(-18)

def create_arbitrageur_relative_balances(df):
    # Create a new column 'arbitrageur_relative_balances_x' by subtracting the initial balance
    initial_balance_x = df.loc[0, 'arbitrageur_balances_x']
    df['arbitrageur_relative_balances_x'] = df['arbitrageur_balances_x'] - initial_balance_x

    # Create a new column 'arbitrageur_relative_balances_x' by subtracting the initial balance
    initial_balance_y = df.loc[0, 'arbitrageur_balances_y']
    df['arbitrageur_relative_balances_y'] = df['arbitrageur_balances_y'] - initial_balance_y

    return df

def create_liquid_exchange_relative_reserves(df):
    # Create a new column 'liquid_exchange_relative_reserves_x' by subtracting the initial balance
    initial_reserve_x = df.loc[0, 'liquid_exchange_reserves_x']
    df['liquid_exchange_relative_reserves_x'] = df['liquid_exchange_reserves_x'] - initial_reserve_x

    # Create a new column 'liquid_exchange_relative_reserves_x' by subtracting the initial balance
    initial_reserve_y = df.loc[0, 'liquid_exchange_reserves_y']
    df['liquid_exchange_relative_reserves_y'] = df['liquid_exchange_reserves_y'] - initial_reserve_y

    return df

def compute_portfolio_values(df):
    df['lp_portfolio_value'] = df['portfolio_reserves_x'] * df['liquid_exchange_prices'] + df['portfolio_reserves_y']
    df['arbitrageur_portfolio_value'] = df['arbitrageur_relative_balances_y']

    return df

def compute_accumulated_fees(df):
    df['accumulated_lp_fees_x'] = df['lp_fees_x'].cumsum()
    df['accumulated_lp_fees_y'] = df['lp_fees_y'].cumsum()

    return df

def compute_mean_and_std(dfs):
    combined_df = pd.concat(dfs, keys=range(len(dfs)), names=['simulation'])
    mean_df = combined_df.groupby(level=1).mean()
    std_df = combined_df.groupby(level=1).std()
    return mean_df, std_df
