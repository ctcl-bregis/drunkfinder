import sys
from itertools import zip_longest
userpath = sys.argv[1] 
userlength = int(sys.argv[2])


def grouper(splitlen, iterable, fillvalue = None):
    args = [iter(iterable)] * splitlen
    return zip_longest(fillvalue = fillvalue, *args)

with open(userpath) as f:
    lines = f.readlines()

    if len(lines) % userlength != 0:
        raise Exception(str(len(lines)) + " is not evenly divisible by " + userlength)


with open(userpath) as f:
    for i, g in enumerate(grouper(userlength, f, fillvalue=''), 1):
        with open("wordlist_split_{0}".format(i * userlength), "w") as out:
            out.writelines(g)

