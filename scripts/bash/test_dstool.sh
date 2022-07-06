#!/usr/bin/bash

# --------------------------------------------------------------------------------
# Variables

DS_TOOL=sp_dstool
DS_ROOT=/tmp/
DS_NAME=test_dstool
DS_STOCKS=/tmp/test_dstool_stocks.csv
DS_EXPORT=/tmp/test_dstool_export.csv

VERBOSE=0
if [ "$1" == "-v" ]; then
    VERBOSE=1
fi

# --------------------------------------------------------------------------------
# Functions

function run_dstool() {
    command="${DS_TOOL} -r ${DS_ROOT} -n ${DS_NAME} $@"
    if [ ${VERBOSE} -eq 1 ]; then
        echo "${command}"
    fi
    ${command}
    if [ $? -ne 0 ]; then
        echo "Error: Failed to run command ${command}"
        exit 1
    fi
}

function initialize() {
    rm -rf ${DS_ROOT}/${DS_NAME}/
    rm -f ${DS_STOCKS}
    rm -f ${DS_EXPORT}

    dt=$(date --date "last week" +"%Y-%m-%d")
    echo "symbol,type,date,quantity,base_price" > ${DS_STOCKS}
    echo "AAPL,stock,${dt},100,115.50" >> ${DS_STOCKS}
    echo "DELL,stock,${dt},100,50.25" >> ${DS_STOCKS}
}

function cleanup() {
    rm -rf ${DS_ROOT}/${DS_NAME}/
    rm -f ${DS_STOCKS}
    rm -f ${DS_EXPORT}
}

# --------------------------------------------------------------------------------
# Main

initialize

run_dstool -o create
run_dstool -o update -s ${DS_STOCKS}
run_dstool -o check
run_dstool -o check -y AAPL
run_dstool -o stat
run_dstool -o stat -y AAPL
run_dstool -o export -y AAPL -e ${DS_EXPORT}
run_dstool -o export -y DELL -e ${DS_EXPORT}
run_dstool -o showh -y DELL
run_dstool -o showd -y AAPL
run_dstool -o shows -y DELL
run_dstool -o reset -y DELL -s ${DS_STOCKS}
run_dstool -o drop -y DELL
run_dstool -o delete

cleanup
