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
while :; do
  read -p "Which day is it?: " number
  [[ $number =~ ^[0-9]+$ ]] || { echo "Enter a valid number"; continue; }
  if ((number >= 1 && number <= 24)); then
    break
  fi
done

# Remove leading zeros
number=$((10#$number))

curl --cookie "session=${session}" https://adventofcode.com/2023/day/${number}/input > official_input.txt 
