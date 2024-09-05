#!/bin/bash

config_path="$HOME/.config/aoc"

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
            if [[ "$language" != "" ]]; then
                echo "language already set" >&2
                exit 1
            elif [[ ! $OPTARG =~ ^[a-zA-Z]+$ ]]; then
                echo "invalid language: $OPTARG" >&2
                exit 1
            fi
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

path_pattern=$(sed "s|^~|$HOME|" "$config_path/path")
path_pattern="${path_pattern%/}"

path_regex=$(echo "$path_pattern" | \
    # escape slashes
    sed 's/\//\\\//g' | \
    # replace {year} with 4-digit number regex group
    sed 's/{year}/([0-9]\{4\})/' | \
    # replace {day:n} with a n-digit padded number regex group
    sed -E 's/\{day:([0-9]+)\}/([0-9]\{\1\})/' | \
    # replace {day} with 1 or 2-digit number regex group
    sed 's/{day}/([0-9]{1,2})/' | \
    # replace {language} with characters regex group
    sed 's/{language}/([a-zA-Z]+)/')

path_regex_matches=0
year_regex_match=""
day_regex_match=""
language_regex_match=""
if [[ "$PWD/" =~ $path_regex ]]; then
    path_regex_matches=1
    year_regex_match="${BASH_REMATCH[1]}"
    day_regex_match="${BASH_REMATCH[2]}"
    language_regex_match="${BASH_REMATCH[3]}"
fi

path="$path_pattern"

if [[ "$year" != "" ]]; then
    path=${path/\{year\}/$year}
elif [[ $path_regex_matches -eq 1 ]]; then
    path=${path/\{year\}/$year_regex_match}
    year=$year_regex_match
else
    echo "year not set" >&2
    exit 1
fi

if [[ "$day" != "" ]]; then
    if [[ "$path" =~ \{day:([0-9]+)\} ]]; then
        day_length="${BASH_REMATCH[1]}"
        path=${path/\{day:$day_length\}/$(printf "%0${day_length}d" $day)}
    else
        path=${path/\{day\}/$day}
    fi
elif [[ $path_regex_matches -eq 1 ]]; then
    path=$(echo "$path" | sed -E "s/\{day(:[0-9]+)?\}/$day_regex_match/")
    day=$day_regex_match
else
    echo "day not set" >&2
    exit 1
fi

if [[ "$language" != "" ]]; then
    path=${path/\{language\}/$language}
elif [[ $path_regex_matches -eq 1 ]]; then
    path=${path/\{language\}/$language_regex_match}
    language=$language_regex_match
else
    echo "language not set" >&2
    exit 1
fi

if [[ ! -d "$path" ]]; then
    echo "path does not exist: $path" >&2
    exit 1
fi
