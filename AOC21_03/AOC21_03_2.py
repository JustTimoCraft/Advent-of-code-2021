#!/usr/bin/python3

def binToDec(bitstring):
    bitstring = bitstring[::-1]
    value = 0
    for index, bit in enumerate(bitstring):
        if bit == "1":
            value += pow(2,index)
    return value

def filter(inputlist, filter, position):
    resultlist = []
    for thingy in inputlist:
        if thingy[position] == filter:
            resultlist.append(thingy)
    return resultlist

def bit_criteria(inputlist, common):
    bitpos = 0
    while len(inputlist) > 1:
        print("List right now: ", inputlist)
        print("bitpos", bitpos)
        balance = 0
        for bits in inputlist:
            if bits[bitpos] == "0":
                balance += 1
            else:
                balance -= 1
        if balance > 0:
            mostcommonbit = "0"
            leastcommonbit = "1"
        elif balance <= 0:
            mostcommonbit = "1"
            leastcommonbit = "0"
        print("most common", mostcommonbit)
        if common == "mc":
            inputlist = filter(inputlist, mostcommonbit, bitpos)
        else:
            inputlist = filter(inputlist, leastcommonbit, bitpos)

        bitpos += 1
        if bitpos == len(inputlist[0]):
            return inputlist[0]

    return inputlist[0]

with open("input.txt") as onesandzeroes:
    data = [line.strip() for line in onesandzeroes]
#data = ["00100","11110","10110","10111","10101","01111","00111","11100","10000","11001","00010","01010"]

o2_gen_values = bit_criteria(data, "mc")
co2_values = bit_criteria(data, "lc")

print("Oxygen generator value found: {} | Decimal value: {}".format(o2_gen_values, binToDec(o2_gen_values)))
print("CO2 scrubber rating value found: {} | Decimal value: {}".format(co2_values, binToDec(co2_values)))
print("Answer: {}".format(binToDec(o2_gen_values) * binToDec(co2_values)))
