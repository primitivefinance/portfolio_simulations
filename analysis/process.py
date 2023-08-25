import pandas as pd
from decimal import Decimal

def import_wad_csv(path):
    df = pd.read_csv(path, dtype=str)
    df = df.applymap(to_decimal)
    return df

def to_decimal(x):
    return Decimal(x).scaleb(-18)