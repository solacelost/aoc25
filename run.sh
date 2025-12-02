#!/bin/bash

set -e

cd "$(dirname "$(realpath "$0")")"

verbose=false
function build {
    if $verbose; then
        cargo build -r --bin "$1"
    else
        cargo build -r --bin "$1" >/dev/null 2>&1
    fi
}

run_test=false
desired_days=()
desired_parts=()
while [[ $# -gt 0 ]]; do
	case "$1" in
	day*)
		desired_days+=("$1")
		;;
	part*)
		desired_parts+=("$1")
		;;
    test)
        cargo fmt
        run_test=true
        ;;
	scaffold)
		shift
		day="$1"
		cp -r dayX "$day"
		sed -i 's/dayX/'"$day"'/' "$day/Cargo.toml"
		sed -i '/^\]/i \ \ \ \ \"'"$day"'\"\,' Cargo.toml
		find "$day/src/bin/" -name '*.rs' -exec rename 'dayX' "$day" {} \;
		exit 0
		;;
    -v|--verbose)
        verbose=true
        ;;
    *)
        echo "Unknown argument: $1" >&2
        exit 1
        ;;
	esac
	shift
done

while read -r dir; do
	day=$(basename "$dir")
	if [ "${desired_days[*]}" ] && ! [[ " ${desired_days[*]} " =~ " $day " ]]; then
		continue
	fi
	args=("$dir/input")
	if [ "${desired_parts[*]}" ]; then
		parts=("${desired_parts[@]}")
	else
		parts=(part{1,2})
	fi
	for part in "${parts[@]}"; do
	    correct="$(cat "$dir/$part.answer" 2>/dev/null)"
		correct="${correct:-unk}"
		bin="target/release/$day-$part"
		src="$dir/src/bin/$day-$part.rs"
		cmd=("$bin" "${args[@]}")
		if [ -f "$src" ]; then
            if $run_test; then
                cargo test --bin "$day-$part"
            else
			    build "$day-$part"
            fi
		fi
		if [ -f "$bin" ] && ! $run_test; then
			echo "$day $part:"
			start=$(date +%s%N)
            if $verbose; then
                ans=$("${cmd[@]}")
                echo "$ans"
            else
			    ans=$("${cmd[@]}" 2>/dev/null)
            fi
			end=$(date +%s%N)
			[ "${ans:-0}" = "$correct" ] && correct=correct || correct="incorrect: $correct"
			echo -en "\t$ans (${correct}) time: "
            echo "$(echo $(( end - start)) 1000000 | awk '{printf "%.2f", $1/$2}')ms"
			echo
		fi
	done
done < <(find . -mindepth 1 -maxdepth 1 -type d -name day\* ! -path './dayX' | sort)
