import argparse
import sys
from abc import ABC, abstractmethod
from datetime import date

def parse_args():
    args = argparse.ArgumentParser()
    args.add_argument("-o", "--override", help="Override generated title and ylabel",
                      dest="override",
                      action="store_true",
                      default=False)
    args.add_argument("-t", "--title", help="Text to augment or override title",
                      dest="title",
                      type=str,
                      default="")
    args.add_argument("-y", "--ylabel", help="Text to augment or override ylabel",
                      dest="ylabel",
                      type=str,
                      default="")
    args.add_argument("-x", "--xlabel", help="Text to set xlabel",
                      dest="xlabel",
                      type=str,
                      default="")
    return args.parse_args()

class PlotStatsBase (ABC):
    def __init__(self, parsed_args):
        self.parsed_args = parsed_args
        self.from_date = None
        self.to_date = None
        self.symbol = None
        self.field = None
        self.window = None
        self.calc = None
        self.data = []
        self.data_dates = []

    @abstractmethod
    def plot_data(self):
        pass

    def run(self):
        self.read_stats_output()
        self.plot_data()

    def read_stats_output(self):
        # Read and parse sp_stats output
        for line in sys.stdin:
            tokens = line.strip().split(' ')
            if len(tokens) > 3:
                raise Exception(f"Invalid line - {line}")

            tok1 = tokens[0].strip()
            if len(tokens) >= 2:
                tok2 = tokens[1].strip()

            if tok1 == "from:":
                self.from_date = tok2
            elif tok1 == "to:":
                self.to_date = tok2
            elif tok1 == "symbol:":
                self.symbol = tok2
            elif tok1 == "field:":
                self.field = tok2
            elif tok1.startswith("window:"):
                self.window = tok2
            elif tok1.startswith("20"):
                self.data_dates.append(date.fromisoformat(tok1))
                self.data.append(float(tok2))
            else:
                self.calc = tok1.split(':')[0]

    def make_title(self):
        field = " ".join([t.capitalize() for t in self.field.split('_')]) if self.field is not None else ""
        return self.augment_or_override(
            "{} {} {} - {}".format(
                self.symbol,
                field,
                self.from_date,
                self.to_date),
            self.parsed_args.title)

    def make_ylabel(self):
        ucalc = self.calc.upper()
        ylbl_text = f"{self.window} Day {ucalc}" if self.window is not None else ucalc
        return self.augment_or_override(ylbl_text, self.parsed_args.ylabel)

    def make_xlabel(self):
        return self.augment_or_override("date", self.parsed_args.xlabel)

    def augment_or_override(self, text, alt_text):
        if alt_text != "":
            if self.parsed_args.override:
                return alt_text
            else:
                return "{} ({})".format(text, alt_text)
        return text
