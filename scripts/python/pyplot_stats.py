#!/usr/bin/python3

import matplotlib.pyplot as plt
from plot_stats_base import PlotStatsBase, parse_args

plt.style.use("dark_background")

class PyplotStats (PlotStatsBase):
    def __init__(self, parsed_args):
        super().__init__(parsed_args)

    def plot_data(self):
        plt.plot(self.data_dates, self.data)
        plt.title(self.make_title())
        plt.ylabel(self.make_ylabel())
        plt.xlabel(self.make_xlabel())
        plt.grid(True)
        plt.show(block=True)

if __name__ == "__main__":
    PyplotStats(parse_args()).run()
