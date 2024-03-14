#!/usr/bin/python3

import subprocess
import sys
from plot_stats_base import PlotStatsBase, parse_args

class PlotStats (PlotStatsBase):
    def __init__(self, parsed_args):
        super().__init__(parsed_args)
        self.tmp_gp_file = "/tmp/plot_stats.tmp"

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

    def run_command(self, command):
        ret = subprocess.run(command, capture_output=True)
        if ret.returncode != 0:
            print(ret.stderr.decode(), file=sys.stderr)
            raise Exception(f"Failed to run command: {command}")

if __name__ == "__main__":
    PlotStats(parse_args()).run()
