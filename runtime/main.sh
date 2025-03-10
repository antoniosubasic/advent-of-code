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
        [[ "$correct_answer" == "$answer" ]]
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
    OPTARG="$(echo "$OPTARG" | sed 's/^0*//')"
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
            fi
            languages=("csharp" "python" "rust" "c" "java")
            if [[ ! " ${languages[@]} " =~ " $OPTARG " ]]; then
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

path_pattern_regex=$(echo "$path_pattern" | sed 's/\//\\\//g')

year_regex=$(echo "$path_pattern_regex" | \
    # strip everything after {year}
    sed 's/\(.*{year}\).*/\1/' | \
    # replace {year} with 4-digit number regex group
    sed 's/{year}/([0-9]\{4\})/')

day_regex=$(echo "$path_pattern_regex" | \
    # strip everything after {day:n}
    sed -E 's/(.*\{day(:[0-9]*)?\}).*/\1/' | \
    # replace {year} with 4-digit number regex group
    sed 's/{year}/([0-9]\{4\})/' | \
    # replace {day:n} with a n-digit padded number regex group
    sed -E 's/\{day:([0-9]+)\}/([0-9]\{\1\})/' | \
    # replace {day} with 1 or 2-digit number regex group
    sed 's/{day}/([0-9]{1,2})/')

language_regex=$(echo "$path_pattern_regex" | \
    # replace {year} with 4-digit number regex group
    sed 's/{year}/([0-9]\{4\})/' | \
    # replace {day:n} with a n-digit padded number regex group
    sed -E 's/\{day:([0-9]+)\}/([0-9]\{\1\})/' | \
    # replace {day} with 1 or 2-digit number regex group
    sed 's/{day}/([0-9]{1,2})/' | \
    # replace {language} with characters regex group
    sed 's/{language}/([a-zA-Z]+)/')

year_regex_match=""
if [[ "$PWD" =~ $year_regex ]]; then
    year_regex_match="${BASH_REMATCH[1]}"
fi

day_regex_match=""
if [[ "$PWD" =~ $day_regex ]]; then
    day_regex_match="${BASH_REMATCH[2]}"
fi

language_regex_match=""
if [[ "$PWD" =~ $language_regex ]]; then
    language_regex_match="${BASH_REMATCH[3]}"
fi

path="$path_pattern"

current_month=$(date +%m)
current_day=$(date +%d | sed 's/^0*//')

if [[ "$current_month" == "12" && "$current_day" -ge 1 && "$current_day" -le 25 ]]; then
    detected_year="$(date +%Y)"
    detected_day="$current_day"
fi

if [[ "$year" != "" ]]; then
    path=${path/\{year\}/$year}
elif [[ "$year_regex_match" != "" ]]; then
    path=${path/\{year\}/$year_regex_match}
    year=$(echo "$year_regex_match" | sed 's/^0*//')
elif [[ "$detected_year" != "" ]]; then
    path=${path/\{year\}/$detected_year}
    year=$detected_year
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
elif [[ "$day_regex_match" != "" ]]; then
    path=$(echo "$path" | sed -E "s/\{day(:[0-9]+)?\}/$day_regex_match/")
    day=$(echo "$day_regex_match" | sed 's/^0*//')
elif [[ "$detected_day" != "" ]]; then
    day=$detected_day
    if [[ "${day:0:1}" == "0" ]]; then
        day=${day:1}
    fi
    if [[ "$path" =~ \{day:([0-9]+)\} ]]; then
        day_length="${BASH_REMATCH[1]}"
        path=${path/\{day:$day_length\}/$(printf "%0${day_length}d" $day)}
    else
        path=${path/\{day\}/$day}
    fi
else
    throw "day not set"
fi

if [[ "$mode" == "url" ]]; then
    echo "https://adventofcode.com/$year/day/$day"
    exit 0
fi

if [[ "$language" != "" ]]; then
    path=${path/\{language\}/$language}
elif [[ "$language_regex_match" != "" ]]; then
    path=${path/\{language\}/$language_regex_match}
    language=$language_regex_match
else
    subpath=$(echo "$path" | sed "s|{language}.*||")
    if [[ -d "$subpath" ]]; then
        if [[ $(find "$subpath" -mindepth 1 -maxdepth 1 -type d | wc -l) -eq 1 ]]; then
            language=$(find "$subpath" -mindepth 1 -maxdepth 1 -type d -printf "%f")
            path=${path/\{language\}/$language}
        else
            throw "language not set - possibilities: $(find "$subpath" -mindepth 1 -maxdepth 1 -type d -printf "%f " | sed 's/ $//')"
        fi
    else
        throw "language not set"
    fi
fi

case $mode in
    path)
        echo "$path"
        exit 0
        ;;
    code)
        code "$path"
        exit 0
        ;;
esac

if [[ ! -f "$config_path/cookie" ]]; then
    throw "cookie not set"
fi

input_file_path="$(dirname $path)/input.txt"
cookie=$(sed 's/\n$//' "$config_path/cookie")

if [[ ! -f "$input_file_path" ]]; then
    if [[ "$cookie" == "" ]]; then
        throw "cookie not set"
    else
        input=$(fetch_input "$year" "$day" "$cookie")

        if [[ "$input" == "Puzzle inputs differ by user.  Please log in to get your puzzle input." ]]; then
            throw "invalid cookie"
        else
            mkdir -p "$(dirname "$input_file_path")"
            echo -n "$input" > "$input_file_path"
        fi
    fi
fi

case $mode in
    run)
        if [[ ! -d "$path" ]]; then
            throw "project does not exist"
        fi

        solution_file_path="$(dirname $path)/.solution.txt"

        command_output=""
        status=0

        cd "$path"
        case $language in
            csharp)
                command_output=$( { time -p dotnet run; } 2>&1 )
                status=$?
                ;;
            python)
                command_output=$( { time -p python3 main.py; } 2>&1 )
                status=$?
                ;;
            rust)
                cargo build --release --quiet
                binary_path=$(cargo metadata --format-version=1 | jq -r '.target_directory')/release/$(basename "$path")
                command_output=$( { time -p "$binary_path"; } 2>&1 )
                status=$?
                ;;
            c)
                outfile=$(mktemp)
                gcc main.c -o "$outfile"
                command_output=$( { time -p "$outfile"; } 2>&1 )
                status=$?
                ;;
            java)
                javac -d bin src/*.java
                command_output=$( { time -p java -cp bin Main; } 2>&1 )
                status=$?
                ;;
            *)
                throw "invalid language: $language"
                ;;
        esac
        cd - > /dev/null

        if [[ $status -ne 0 ]]; then
            throw "$(echo "$command_output" | head -n -3)"
        else
            output=$(echo "$command_output" | head -n 2)

            part1=$(echo "$output" | sed -n '1p')
            part2=$(echo "$output" | sed -n '2p')

            evaluate_solution "$solution_file_path" "$year" "$day" "$part1" "$part2" "$cookie"
            evaluation=$?

            part1_color=$([[ $evaluation -eq 1 || $evaluation -eq 3 ]] && echo $GREEN || echo $RED)
            part2_color=$([[ $evaluation -eq 2 || $evaluation -eq 3 ]] && echo $GREEN || echo $RED)

            echo -e "${part1_color}$part1${NC}"
            echo -e "${part2_color}$part2${NC}"
            echo -e "\n$(echo "$command_output" | tail -n 3 | head -n 1 | cut -d ' ' -f 2)s"
        fi
        ;;
    init)
        if [[ -d "$path" ]]; then
            throw "project already exists"
        fi

        base_file="$config_path/base.$language"
        
        base_file_oneline="$base_file.oneline"
        lines=$(wc -l < "$input_file_path")
        if [[ $lines -lt 1 && -f "$base_file_oneline" ]]; then
            base_file="$base_file_oneline"
        fi

        case $language in
            csharp)
                mkdir -p "$path"
                dotnet new console -o "$path"
                cp "$base_file" "$path/Program.cs"
                ;;
            python)
                mkdir -p "$path"
                cp "$base_file" "$path/main.py"
                ;;
            rust)
                mkdir -p "$(dirname "$path")"
                cargo new "$path" --bin
                cp "$base_file" "$path/src/main.rs"
                ;;
            c)
                mkdir -p "$path"
                cp "$base_file" "$path/main.c"
                ;;
            *)
                throw "invalid language: $language"
                ;;
        esac
        ;;
    *)
        throw "invalid mode: $mode"
        ;;
esac
