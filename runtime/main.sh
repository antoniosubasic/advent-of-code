#!/bin/bash

GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

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

# return codes
# 0: right answer
# 1: wrong answer
# 2: cooldown
submit_answer() {
    local year="$1"
    local day="$2"
    local part="$3"
    local answer="$4"
    local cookie="$5"

    local response=$(curl -s -X POST -H "Cookie: session=${cookie}" -d "level=${part}&answer=${answer}" "https://adventofcode.com/${year}/day/${day}/answer")

    if [[ "$response" =~ "That's the right answer!" ]]; then
        return 0
    elif [[ "$response" =~ "You don't seem to be solving the right level" ]]; then
        local day_response=$(curl -s -H "Cookie: session=${cookie}" "https://adventofcode.com/${year}/day/${day}")
        local correct_answer=$(echo "$day_response" | grep -oP '<p>Your puzzle answer was <code>\K(.*?)(?=</code>)' | sed -n "${part}p")
        [[ "$correct_answer" -eq "$answer" ]]
        return $?
    elif [[ "$response" =~ "You gave an answer too recently" ]]; then
        local cooldown_time=$(echo "$response" | grep -oP 'You have \K(.*?)(?= left to wait)')
        echo "cooldown left: $cooldown_time" >&2
        return 2
    else
        return 1
    fi
}

# return codes
# 0: both solutions incorrect
# 1: part 1 correct
# 2: part 2 correct
# 3: both solutions correct
evaluate_solution() {
    local solution_file="$1"
    local year="$2"
    local day="$3"
    local solution_part1="$4"
    local solution_part2="$5"
    local cookie="$6"

    if [[ -f "$solution_file" ]]; then
        local solution_content
        solution_content=$(<"$solution_file")

        local part1=$(echo "$solution_content" | sed -n '1p')
        local part2=$(echo "$solution_content" | sed -n '2p')

        local return_code=0

        if [[ "$part1" == "$solution_part1" ]]; then
            return_code=1
        fi

        if [[ "$part2" == "$solution_part2" ]]; then
            return_code=$((return_code + 2))
        fi

        return $return_code
    else
        part1_submit_output=$(submit_answer "$year" "$day" 1 "$solution_part1" "$cookie" 2>&1)
        part1_submit_result=$?
        if [[ $part1_submit_result -eq 2 ]]; then
            throw "$part1_submit_output"
        fi

        part2_submit_output=$(submit_answer "$year" "$day" 2 "$solution_part2" "$cookie" 2>&1)
        part2_submit_result=$?
        if [[ $part2_submit_result -eq 2 ]]; then
            throw "$part2_submit_output"
        fi

        if [[ $part1_submit_result -eq 0 && $part2_submit_result -eq 0 ]]; then
            echo -e "$solution_part1\n$solution_part2" > "$solution_file"
            return 3
        elif [[ $part1_submit_result -eq 0 ]]; then
            return 1
        elif [[ $part2_submit_result -eq 0 ]]; then
            return 2
        else
            return 0
        fi
    fi
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
        solution_file_path="$(dirname $path)/.solution.txt"
        cookie=$(sed 's/\n$//' "$config_path/cookie")

        if [[ ! -f "$input_file_path" ]]; then
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

        part1=$(echo "$output" | sed -n '1p')
        part2=$(echo "$output" | sed -n '2p')

        evaluate_solution "$solution_file_path" "$year" "$day" "$part1" "$part2" "$cookie"
        evaluation=$?

        part1_color=$([[ $evaluation -eq 1 || $evaluation -eq 3 ]] && echo $GREEN || echo $RED)
        part2_color=$([[ $evaluation -eq 2 || $evaluation -eq 3 ]] && echo $GREEN || echo $RED)

        echo -e "${part1_color}$part1${NC}"
        echo -e "${part2_color}$part2${NC}"
        ;;
    *)
        throw "invalid mode: $mode"
        ;;
esac
