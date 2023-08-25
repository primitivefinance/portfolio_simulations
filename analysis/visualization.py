import pandas as pd
from decimal import Decimal

# Read CSV file into a DataFrame
df = pd.read_csv('../portfolio.csv', dtype=str)


def to_decimal(x):
    return Decimal(x).scaleb(-18)


df = df.applymap(to_decimal)

print(df)