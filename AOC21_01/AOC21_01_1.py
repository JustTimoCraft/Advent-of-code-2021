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

increaseCounter = 0
for i in range(1,len(numbers)):
    if numbers[i] > numbers[i-1]:
        increaseCounter += 1

print("Increments: {}".format(increaseCounter))
