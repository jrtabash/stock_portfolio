import os
import subprocess
import sys
from datetime import date

class ParsedArgs:
    def __init__(self):
        self.verbose = False
        self.update = False
        self.ds_root = ""
        self.ds_name = ""
        self.ds_stocks = ""
        self.symbols = []
        self.base_date = None
        self.parse()
        self.validate()

    def usage(self):
        print("Create a research stock portfolio datastore")
        print("")
        print("Usage")
        print("    {} [FLAGS] [OPTIONS]".format(sys.argv[0]))
        print("")
        print("FLAGS")
        print("       --help : Print usage and exit")
        print("    --verbose : Use verbose mode")
        print("     --update : Update symbols in new datastore")
        print("")
        print("OPTIONS")
        print("       --root=<ROOT> : New datastore root")
        print("       --name=<NAME> : New datastore name sp_<NAME>, and new csv file <NAME>.csv")
        print("       --date=<DATE> : Base date, formatted as YYYY-MM-DD")
        print("    --symbols=<SYMS> : Comma separated list of symbols")
        print("")

    def parse(self):
        for arg in sys.argv[1:]:
            if arg == "--help":
                self.usage()
                sys.exit(0)
            elif arg == "--verbose":
                self.verbose = True
            elif arg == "--update":
                self.update = True
            else:
                name, value = arg.split("=")
                if name == "--root":
                    self.ds_root = value
                elif name == "--name":
                    self.ds_name = f"sp_{value}"
                    self.ds_stocks = f"{value}.csv"
                elif name == "--date":
                    self.base_date = date.fromisoformat(value)
                elif name == "--symbols":
                    self.symbols = [s.strip() for s in value.split(',')]
                else:
                    raise Exception(f"Unknown argument {arg}")

    def validate(self):
        if self.ds_root == "":
            raise Exception("Missing root")
        if self.ds_name == "" or self.ds_name == "sp_":
            raise Exception("Missing name")
        if self.ds_stocks == "" or self.ds_stocks == ".csv":
            raise Exception("Missing name")
        if self.symbols == [] or self.symbols == [""]:
            raise Exception("Missing symbols")
        if self.base_date is None:
            raise Exception("Missing date")

class MakeDSProcessor:
    def __init__(self, parsed_args):
        self.parsed_args = parsed_args

    def run(self):
        stocks_file = os.path.join(self.parsed_args.ds_root, self.parsed_args.ds_stocks)
        self.create_stocks_file(stocks_file)
        self.sp_dstool("create", None)
        if self.parsed_args.update:
            self.sp_dstool("update", stocks_file)
            self.sp_dstool("check")

    def create_stocks_file(self, filename):
        if self.parsed_args.verbose:
            print(f"Create file: {filename}")

        if os.path.exists(filename):
            raise Exception(f"File {filename} already exists")

        with open(filename, "w") as fd:
            fd.write("symbol,type,date,quantity,base_price\n")
            base_date = self.parsed_args.base_date
            for sym in self.parsed_args.symbols:
                fd.write(f"{sym},stock,{base_date},100,0.00\n")

    def sp_dstool(self, dsop, stocks=None):
        command = ["sp_dstool"]
        command.append(f"--root={self.parsed_args.ds_root}")
        command.append(f"--name={self.parsed_args.ds_name}")
        command.append(f"--dsop={dsop}")
        if stocks is not None:
            command.append(f"--stocks={stocks}")
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
    MakeDSProcessor(ParsedArgs()).run()
