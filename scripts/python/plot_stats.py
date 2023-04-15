#!/usr/bin/python3

import subprocess
import sys

class PlotStats:
    def __init__(self):
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
            f.write("set title '{} {} {} - {}' tc rgb '#808080'\n".format(
                self.symbol,
                self.field.capitalize() if self.field is not None else "",
                self.from_date,
                self.to_date))
            f.write(f"set ylabel '{self.calc}' tc rgb '#808080'\n")
            f.write("plot '-' with lines notitle\n")
            for val in self.data:
                f.write(f"{val}\n")
            f.write("EOF")
        self.run_command(["gnuplot", "-p", self.tmp_gp_file])

    def run_command(self, command):
        ret = subprocess.run(command, capture_output=True)
        if ret.returncode != 0:
            print(ret.stderr.decode(), file=sys.stderr)
            raise Exception(f"Failed to run command: {command}")

if __name__ == "__main__":
    PlotStats().run()
