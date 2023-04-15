#!/usr/bin/python3

import argparse
import subprocess
import sys

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

class PlotStats:
    def __init__(self, parsed_args):
        self.parsed_args = parsed_args
        self.from_date = None
        self.to_date = None
        self.symbol = None
        self.field = None
        self.calc = None
        self.data = []
        self.tmp_gp_file = "/tmp/plot_stats.tmp"

    def run(self):
        self.read_stats_output()
        self.plot_data()

    def read_stats_output(self):
        # Read and parse sp_stats output
        for line in sys.stdin:
            tokens = line.strip().split(' ')
            if len(tokens) > 2:
                raise Exception(f"Invalid line - {line}")

            tok1 = tokens[0].strip()
            if len(tokens) == 2:
                tok2 = tokens[1].strip()

            if tok1 == "from:":
                self.from_date = tok2
            elif tok1 == "to:":
                self.to_date = tok2
            elif tok1 == "symbol:":
                self.symbol = tok2
            elif tok1 == "field:":
                self.field = tok2
            elif tok1.startswith("20"):
                self.data.append(float(tok2))
            else:
                self.calc = tok1.split(':')[0]

    def plot_data(self):
        with open(self.tmp_gp_file, "w") as f:
            f.write("set terminal wxt background 0\n")
            f.write("set style line 101 lc rgb '#808080' lt 1 lw 1\n")
            f.write("set border 3 front ls 101 lc rgb '#808080'\n")
            f.write("set title '{}' tc rgb '#808080'\n".format(self.make_title()))
            f.write("set ylabel '{}' tc rgb '#808080'\n".format(self.make_ylabel()))
            f.write("set xlabel '{}' tc rgb '#808080'\n".format(self.parsed_args.xlabel))
            f.write("plot '-' with lines notitle\n")
            for val in self.data:
                f.write(f"{val}\n")
            f.write("EOF")
        self.run_command(["gnuplot", "-p", self.tmp_gp_file])

    def make_title(self):
        return self.augment_or_override(
            "{} {} {} - {}".format(
                self.symbol,
                self.field.capitalize() if self.field is not None else "",
                self.from_date,
                self.to_date),
            self.parsed_args.title)

    def make_ylabel(self):
        return self.augment_or_override(self.calc, self.parsed_args.ylabel)

    def augment_or_override(self, text, alt_text):
        if alt_text != "":
            if self.parsed_args.override:
                return alt_text
            else:
                return "{} ({})".format(text, alt_text)
        return text

    def run_command(self, command):
        ret = subprocess.run(command, capture_output=True)
        if ret.returncode != 0:
            print(ret.stderr.decode(), file=sys.stderr)
            raise Exception(f"Failed to run command: {command}")

if __name__ == "__main__":
    PlotStats(parse_args()).run()
