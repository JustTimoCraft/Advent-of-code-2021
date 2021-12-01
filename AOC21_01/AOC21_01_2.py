#!/usr/bin/python3
### Advent of Code Day 1 Challenge 1
### JTC, December 2021

with open("input.txt") as inputFile:
    fileContents = [line.strip() for line in inputFile]

numbers = []
for element in fileContents:
    try:
        numbers.append(int(element))
    except ValueError:
        print("Something went wrong")
        exit()

pairs = []
for i in range(2,len(numbers)):
    pairs.append(numbers[i-2] + numbers[i-1] + numbers[i])

increaseCounter = 0
for i in range(1,len(pairs)):
    if pairs[i] > pairs[i-1]:
        increaseCounter += 1

print("Increments: {}".format(increaseCounter))
