import os
import pandas as pd
import sp_util
from typing import Optional

OptionalStr = Optional[str]

class DSException (Exception):
    pass

class DataStore:
    def __init__(self, root: OptionalStr = None, name: OptionalStr = None):
        self.root: str = self.root_or_default(root)
        self.name: str = self.name_or_default(name)
        self.path: str = os.path.join(self.root, self.name)
        self.validate()

    def __str__(self) -> str:
        return f"DataStore[{self.path}]"

    def __repr__(self) -> str:
        return str(self)

    def validate(self):
        if len(self.root) == 0:
            raise DSException("Missing datastore root")
        if not os.path.exists(self.path):
            raise DSException(f"Datastore {self.path} does not exist")

    def read_data(self, tag: str, symbol: str) -> pd.DataFrame:
        if tag == sp_util.history_tag():
            names = ["date", "open", "high", "low", "close", "adj_close", "volume"]
        else:
            names = ["date", "dividend"]
        symbol_path = self.make_symbol_path(tag, symbol)
        if os.path.exists(symbol_path):
            return pd.read_csv(symbol_path,
                               names=names,
                               header=None,
                               converters={"date": pd.Timestamp},
                               index_col="date")
        else:
            return pd.DataFrame({f: [] for f in names})

    def read_history(self, symbol: str) -> pd.DataFrame:
        return self.read_data(sp_util.history_tag(), symbol)

    def read_dividends(self, symbol: str) -> pd.DataFrame:
        return self.read_data(sp_util.dividends_tag(), symbol)

    def make_symbol_path(self, tag: str, symbol: str) -> str:
        return os.path.join(self.path, f"{tag}_{symbol}.csv")

    @staticmethod
    def root_or_default(root: OptionalStr) -> str:
        if root is None:
            return sp_util.home_path()
        return root

    @staticmethod
    def name_or_default(name: OptionalStr) -> str:
        if name is None:
            return "sp_datastore"
        return name
