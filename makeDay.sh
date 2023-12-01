#!/bin/bash

# Check is session key files exists
if [[ -f "session.txt" ]]
then
    session=$(<session.txt)
else
    read -p "Please enter you session key: " session
    touch session.txt
    echo ${session} > session.txt
fi

# Read user input
read -p "Which day is it? : " rawNumber

# Remove leading zeros
rawNumber=$((10#$rawNumber))

# Add leading zero from 1 to 9
if ((rawNumber >= 1 && rawNumber <= 9)); then
    number=0${rawNumber}
else
    number=${rawNumber}
fi

# Day folder name
day="day-${number}"

# Check that directory does not exist yet
if [[ -d ${day} ]]
then
    echo "${day} already exists!"
    code ${day}
    exit 0
fi

# Create new Rust project
cargo new ${day} --vcs none

# Create input files
touch ${day}/input1.txt
touch ${day}/input2.txt
touch ${day}/official_input.txt

curl --cookie "session=${session}" https://adventofcode.com/2023/day/${rawNumber}/input > ${day}/official_input.txt

# Create main from template
rm ${day}/src/main.rs
cp template/template.rs ${day}/src/main.rs

# Test run the template
cd ${day}
cargo run
code .