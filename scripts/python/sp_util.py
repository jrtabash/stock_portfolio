import os

def history_tag() -> str:
    return "history"

def dividends_tag() -> str:
    return "dividends"

def home_path(raise_if_missing: bool = False) -> str:
    if "USERPROFILE" in os.environ:
        return os.environ["USERPROFILE"]
    elif "HOME" in os.environ:
        return os.environ["HOME"]
    else:
        if raise_if_missing:
            raise Exception("Failed to find home path")
        return ""
