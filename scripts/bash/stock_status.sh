#!/usr/bin/bash

# --------------------------------------------------------------------------------
# Variables

CONFIG=""
SYMBOL=""
GROUPBY=
DAYCH=0
PLOT=0
CLEAR=0

PLOT_SCRIPT="plot_stats.py"

# --------------------------------------------------------------------------------
# Functions

function print_usage() {
    echo "Usage"
    echo "    stock_status.sh [-h] [-g] [-c] [-p] [-y] <config> <symbol>"
    echo ""
    echo "Options"
    echo "    -h : Print help"
    echo "    -g : Report group by"
    echo "    -d : Report day change"
    echo "    -p : Plot close history"
    echo "    -y : Plot using pyplot, requires -p"
    echo "    -c : Clear screen"
}

# --------------------------------------------------------------------------------
# Arguments

for arg in $@;
do
    if [ "${arg}" == "-h" ]; then
        print_usage
        exit 0
    elif [ "${arg}" == "-g" ]; then
        GROUPBY="-g"
    elif [ "${arg}" == "-d" ]; then
        DAYCH=1
    elif [ "${arg}" == "-p" ]; then
        PLOT=1
    elif [ "${arg}" == "-y" ]; then
        PLOT_SCRIPT="pyplot_stats.py"
    elif [ "${arg}" == "-c" ]; then
        CLEAR=1
    elif [ "${CONFIG}" == "" ]; then
        CONFIG="${arg}"
    elif [ "${SYMBOL}" == "" ]; then
        SYMBOL="${arg}"
    else
        echo "Error: Unknown option '${arg}'"
        exit 1
    fi
done

# --------------------------------------------------------------------------------
# Validate

if [ "${CONFIG}" == "" ]; then
    echo "Missing config"
    exit 1
fi

if [ "${SYMBOL}" == "" ]; then
    echo "Missing symbol"
    exit 1
fi

if [ ! -f "${CONFIG}" ]; then
    echo "Config '${CONFIG}' does not exit"
    exit 1
fi

CONSYM=$(sp_dstool -l ${CONFIG} -o consym -y ${SYMBOL} | grep -c "not in datastore")
if [ ${CONSYM} -eq 1 ]; then
    echo "Symbol ${SYMBOL} not in datastore"
    exit 1
fi

# --------------------------------------------------------------------------------
# Run

if [ ${CLEAR} -eq 1 ]; then
    clear
fi

echo "-------------------"
sp_report -l ${CONFIG} -i ${SYMBOL} ${GROUPBY}

echo ""
echo "----------------------"
sp_report -l ${CONFIG} -p divid -i ${SYMBOL}

if [ ${DAYCH} -eq 1 ]; then
    echo ""
    echo "------------------------"
    sp_report -l ${CONFIG} -p daych -i ${SYMBOL}
fi

if [ ${PLOT} -eq 1 ]; then
    sp_stats -c sma -w 1 -i close -l ${CONFIG} -y ${SYMBOL} | ${PLOT_SCRIPT}
fi
