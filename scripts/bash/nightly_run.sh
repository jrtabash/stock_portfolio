#!/usr/bin/bash

# --------------------------------------------------------------------------------
# Variables

NAME=""
STATS_PATH=""

for arg in $@;
do
    if [ "${arg}" == "-h" ]; then
        echo "Usage"
        echo "    $0 [-h] <name> [<spath>]"
        echo ""
        echo "Where"
        echo "   <name> : Required; implies data store name is sp_<name>"
        echo "            and stocks file is <name>.csv, except for default"
        echo "            sp_datastore the expected stocks file is stocks.csv"
        echo "  <spath> : Optional; If provided, it should point to a "
        echo "            valid directory which will be used to store symbol"
        echo "            export as well as mvwap and roc stats for all symbols"
        echo "            in sp_<name> datastore"
        echo ""
        exit 1
    elif [ "${NAME}" == "" ]; then
        NAME=${arg}
    elif [ "${STATS_PATH}" == "" ]; then
        STATS_PATH=${arg}
    else
        echo "Unknown argument ${arg}"
        exit 1
    fi
done

if [ "${NAME}" == "" ]; then
    echo "Missing name"
    exit 1
fi

if [ "${STATS_PATH}" != "" ]; then
    if [ ! -d ${STATS_PATH} ]; then
       echo "Stats path '${STATS_PATH}' does not exist"
       exit 1
    fi
fi

# --------------------------------------------------------------------------------
# Functions

function run_command() {
    command=$@
    ${command}
    if [ $? -ne 0 ]; then
        echo "Error: Failed to run command ${command}"
        exit 1
    fi
}

function process_nightly() {
    ds_name=sp_${NAME}
    ds_stocks=${NAME}.csv
    if [ "${NAME}" == "datastore" ]; then
        ds_stocks=stocks.csv
    fi

    run_command sp_dstool -n ${ds_name} -s ${ds_stocks} -o update > /dev/null

    if [ "${STATS_PATH}" != "" ]; then
        for symbol in $(cat ${ds_stocks} | cut -d , -f 1 | grep -v 'symbol');
        do
            hist_file=${STATS_PATH}/hist_${symbol}.csv
            mvwap_file=${STATS_PATH}/mvwap_${symbol}.dat
            roc_file=${STATS_PATH}/roc_${symbol}.dat

            run_command sp_dstool -n ${ds_name} -o export -e ${hist_file} -y ${symbol} > /dev/null
            run_command sp_stats -n ${ds_name} -c mvwap -w 5 -y ${symbol} > ${mvwap_file}
            run_command sp_stats -n ${ds_name} -c roc -w 4 -y ${symbol} > ${roc_file}
        done
    fi
}

# --------------------------------------------------------------------------------
# Main

pushd /home/${USER} > /dev/null
process_nightly
