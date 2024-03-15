numbers = [
    68,
    73,
    68,
    76,
    1,
    107,
    2,
    188,
    138,
    1,
    126,
    197,
    254,
    210,
    1,
    113,
    1,
    0,
    0,
    1,
]
string = "".join(chr(num) for num in numbers if 32 <= num <= 126)
print(string)
