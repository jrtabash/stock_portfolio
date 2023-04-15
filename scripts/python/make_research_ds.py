#!/usr/bin/python3

import argparse
import os
import subprocess
import sys
from datetime import date

def parse_args():
    args = argparse.ArgumentParser()
    args.add_argument("-v", "--verbose", help="Use verbose mode",
                      dest="verbose",
                      action="store_true",
                      default=False)
    args.add_argument("-u", "--update", help="Update symbols in new datastore",
                      dest="update",
                      action="store_true",
                      default=False)
    args.add_argument("--root", help="New datastore root",
                      dest="ds_root",
                      type=str,
                      default="",
                      required=True)
    args.add_argument("--name", help="New datastore name sp_<NAME>, and new csv file <NAME>.csv",
                      dest="name",
                      type=str,
                      default="",
                      required=True)
    args.add_argument("--date", help="Base date, formatted as YYYY-MM-DD",
                      dest="base_date",
                      type=str,
                      default="",
                      required=True)
    args.add_argument("--symbols", help="Comma separated list of symbols",
                      dest="symbols",
                      type=str,
                      default="",
                      required=True)

    ns = args.parse_args()
    if ns.ds_root == "":
        raise Exception("Missing root")
    if ns.name == "":
        raise Exception("Missing name")
    if ns.symbols == "":
        raise Exception("Missing symbols")
    if ns.base_date == "":
        raise Exception("Missing date")

    ns.ds_name = f"sp_{ns.name}"
    ns.base_date = date.fromisoformat(ns.base_date)
    ns.symbols = [s.strip() for s in ns.symbols.split(',')]
    ns.config_filename = os.path.join(ns.ds_root, f"{ns.name}.cfg")

    return ns

class MakeDSProcessor:
    def __init__(self, parsed_args):
        self.parsed_args = parsed_args

    def run(self):
        self.create_stocks_config()
        self.sp_dstool("create")
        if self.parsed_args.update:
            self.sp_dstool("update")
            self.sp_dstool("check")

    def create_stocks_config(self):
        filename = self.parsed_args.config_filename
        if self.parsed_args.verbose:
            print(f"Create file: {filename}")

        if os.path.exists(filename):
            raise Exception(f"File {filename} already exists")

        with open(filename, "w") as fd:
            fd.write(f"ds_root: {self.parsed_args.ds_root}\n")
            fd.write(f"ds_name: {self.parsed_args.ds_name}\n")
            fd.write("stocks: csv{\n");
            fd.write("  symbol,type,date,quantity,base_price\n")
            base_date = self.parsed_args.base_date
            for sym in self.parsed_args.symbols:
                fd.write(f"  {sym},stock,{base_date},100,0.00\n")
            fd.write("}\n")

    def sp_dstool(self, dsop):
        command = ["sp_dstool"]
        command.append(f"--config={self.parsed_args.config_filename}")
        command.append(f"--dsop={dsop}")
        self.run_command(command)

    def run_command(self, command):
        if self.parsed_args.verbose:
            print("Run command: {}".format(" ".join(command)))

        capture = not self.parsed_args.verbose
        ret = subprocess.run(command, capture_output=capture)
        if ret.returncode != 0:
            if capture:
                print(ret.stderr.decode(), file=sys.stderr)
            raise Exception(f"Failed to run command: {command}")

if __name__ == "__main__":
    MakeDSProcessor(parse_args()).run()
