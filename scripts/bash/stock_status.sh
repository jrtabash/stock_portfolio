#!/usr/bin/bash

# --------------------------------------------------------------------------------
# Variables

CONFIG=""
SYMBOL=""
GROUPBY=
PLOT=0
CLEAR=0

# --------------------------------------------------------------------------------
# Functions

function print_usage() {
    echo "Usage"
    echo "    stock_status.sh [-h] [-g] [-c] [-p] <config> <symbol>"
    echo ""
    echo "Options"
    echo "    -h : Print help"
    echo "    -g : Report group by"
    echo "    -p : Plot close history"
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
    elif [ "${arg}" == "-p" ]; then
        PLOT=1
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

# --------------------------------------------------------------------------------
# Run

if [ ${CLEAR} -eq 1 ]; then
    clear
fi

sp_report -l ${CONFIG} -o date -i ${SYMBOL} ${GROUPBY}

if [ ${PLOT} -eq 1 ]; then
    sp_stats -c sma -w 1 -i close -l ${CONFIG} -y ${SYMBOL} | plot_stats.py
fi
