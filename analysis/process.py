import pandas as pd
from decimal import Decimal

def import_wad_csv(path):
    df = pd.read_csv(path, dtype=str)
    df = df.applymap(to_decimal)
    df = create_arbitrageur_relative_balances(df)
    print(df)
    return df

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