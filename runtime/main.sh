#!/bin/bash

throw() {
    local message="$1"
    local error_code="${2:-1}"
    echo -e "$message" >&2
    exit "$error_code"
}

fetch_input() {
    local year="$1"
    local day="$2"
    local cookie="$3"

    local response=$(curl -s -H "Cookie: session=${cookie}" "https://adventofcode.com/${year}/day/${day}/input")

    echo "$response"
}

config_path="$HOME/.config/aoc"

year=""
day=""
language=""
mode="run"

while getopts ":y:d:l:" opt; do
    case ${opt} in
        y)
            if [[ "$year" != "" ]]; then
                throw "year already set"
            elif [[ ! $OPTARG =~ ^[0-9]{4}$ ]]; then
                throw "invalid year: $OPTARG"
            elif [[ $OPTARG -lt 2015 ]]; then
                throw "year must be >= 2015"
            elif [[ $OPTARG -gt $(date +%Y) ]]; then
                throw "year must be <= $(date +%Y)"
            fi
            year="$OPTARG"
            ;;
        d)
            if [[ "$day" != "" ]]; then
                throw "day already set"
            elif [[ ! $OPTARG =~ ^[0-9]{1,2}$ ]]; then
                throw "invalid day: $OPTARG"
            elif [[ $OPTARG -lt 1 ]]; then
                throw "day must be >= 1"
            elif [[ $OPTARG -gt 25 ]]; then
                throw "day must be <= 25"
            fi
            day="$OPTARG"
            ;;
        l)
            if [[ "$language" != "" ]]; then
                throw "language already set"
            elif [[ ! $OPTARG =~ ^[a-zA-Z]+$ ]]; then
                throw "invalid language: $OPTARG"
            fi
            language="$OPTARG"
            ;;
        \?)
            echo "invalid option: -$OPTARG" >&2
            ;;
        :)
            throw "option -$OPTARG requires an argument."
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
    throw "year not set"
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
    throw "day not set"
fi

if [[ "$language" != "" ]]; then
    path=${path/\{language\}/$language}
elif [[ $path_regex_matches -eq 1 ]]; then
    path=${path/\{language\}/$language_regex_match}
    language=$language_regex_match
else
    throw "language not set"
fi

if [[ ! -d "$path" ]]; then
    throw "path does not exist: $path"
fi

case $mode in
    run)
        input_file_path="$(dirname $path)/input.txt"

        if [[ ! -f "$input_file_path" ]]; then
            cookie=$(sed 's/\n$//' "$config_path/cookie")

            if [[ "$cookie" == "" ]]; then
                throw "cookie not set"
            else
                input=$(fetch_input "$year" "$day" "$cookie")

                if [[ "$input" == "Puzzle inputs differ by user.  Please log in to get your puzzle input." ]]; then
                    throw "invalid cookie"
                else
                    echo -n "$input" > "$input_file_path"
                fi
            fi
        fi

        output=""

        case $language in
            csharp)
                cd "$path"
                command_output=$(dotnet run 2>&1)
                status=$?
                cd - > /dev/null

                if [[ $status -ne 0 ]]; then
                    throw "$command_output"
                else
                    output="$command_output"
                fi
                ;;
            *)
                throw "invalid language: $language"
                ;;
        esac

        echo "$output"
        ;;
    *)
        throw "invalid mode: $mode"
        ;;
esac
