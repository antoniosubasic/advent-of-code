#!/bin/bash

year=""
day=""
language=""
mode="run"

while getopts ":y:d:l:" opt; do
    case ${opt} in
        y)
            if [[ "$year" != "" ]]; then
                echo "year already set" >&2
                exit 1
            elif [[ ! $OPTARG =~ ^[0-9]{4}$ ]]; then
                echo "invalid year: $OPTARG" >&2
                exit 1
            elif [[ $OPTARG -lt 2015 ]]; then
                echo "year must be >= 2015" >&2
                exit 1
            elif [[ $OPTARG -gt $(date +%Y) ]]; then
                echo "year must be <= $(date +%Y)" >&2
                exit 1
            fi
            year="$OPTARG"
            ;;
        d)
            if [[ "$day" != "" ]]; then
                echo "day already set" >&2
                exit 1
            elif [[ ! $OPTARG =~ ^[0-9]{1,2}$ ]]; then
                echo "invalid day: $OPTARG" >&2
                exit 1
            elif [[ $OPTARG -lt 1 ]]; then
                echo "day must be >= 1" >&2
                exit 1
            elif [[ $OPTARG -gt 25 ]]; then
                echo "day must be <= 25" >&2
                exit 1
            fi
            day="$OPTARG"
            ;;
        l)
            language="$OPTARG"
            ;;
        \?)
            echo "invalid option: -$OPTARG" >&2
            ;;
        :)
            echo "option -$OPTARG requires an argument." >&2
            exit 1
            ;;
    esac
done

shift $((OPTIND - 1))

if [[ $# -gt 0 ]]; then
    mode="$1"
fi
