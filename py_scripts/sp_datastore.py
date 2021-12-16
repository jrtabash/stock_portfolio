import os
import pandas as pd
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

    def validate(self):
        if len(self.root) == 0:
            raise DSException("Missing datastore root")
        if not os.path.exists(self.path):
            raise DSException(f"Datastore {self.path} does not exist")

    def read_data(self, tag :str, symbol: str) -> pd.DataFrame:
        if tag == self.history_tag():
            names = ["date", "open", "high", "low", "close", "adj_close", "volume"]
        else:
            names = ["date", "dividend"]
        return pd.read_csv(self.make_symbol_path(tag, symbol),
                           names=names,
                           header=None,
                           converters={"date": pd.Timestamp},
                           index_col="date")

    def read_history(self, symbol: str) -> pd.DataFrame:
        return self.read_data(self.history_tag(), symbol)

    def read_dividends(self, symbol: str) -> pd.DataFrame:
        return self.read_data(self.dividends_tag(), symbol)

    def make_symbol_path(self, tag: str, symbol: str) -> str:
        return os.path.join(self.path, f"{tag}_{symbol}.csv")

    @staticmethod
    def root_or_default(root: OptionalStr) -> str:
        if root is None:
            if "USERPROFILE" in os.environ:
                return os.environ["USERPROFILE"]
            elif "HOME" in os.environ:
                return os.environ["HOME"]
            else:
                return ""
        return root

    @staticmethod
    def name_or_default(name: OptionalStr) -> str:
        if name is None:
            return "sp_datastore"
        return name

    @staticmethod
    def history_tag() -> str:
        return "history"

    @staticmethod
    def dividends_tag() -> str:
        return "dividends"
