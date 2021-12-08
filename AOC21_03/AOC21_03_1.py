#!/usr/bin/python3

def binToDec(bitstring):
    bitstring = bitstring[::-1]
    value = 0
    for index, bit in enumerate(bitstring):
        if bit == "1":
            value += pow(2,index)
    return value

with open("input.txt") as onesandzeroes:
    data = [line.strip() for line in onesandzeroes]

gamma = ""
for index in range(len(list(data[0]))):
    balance = 0
    for number in data:
        if number[index] == '1':
            balance += 1
        else:
            balance -= 1
    if balance > 0:
        gamma += "1"
    else:
        gamma += "0"

print("Gamma:", gamma)

epsilon = ""
for bit in list(gamma):
    if bit == "1":
        epsilon += "0"
    else:
        epsilon += "1"

print("Epsilon:", epsilon)

gamma_value = binToDec(gamma)
epsilon_value = binToDec(epsilon)
print("Gamma value:", gamma_value)
print("Epsilon value:", epsilon_value)
print("Answer:", gamma_value*epsilon_value)
