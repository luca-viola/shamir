# This class implements a two way conversion logic between base 10 and bases
# 36,62,85,93 numbers.
#
# Base 36 numbers use the symbols 0..9a..z
# Base 62 numbers use the symbols 0..9a..zA..Z
#
# for human readable, meaningful numbers.


class MultiBaseEncoder:
  CHARS = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
           82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
           112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 33, 35, 36, 37, 38, 40, 41, 42, 43, 45, 59, 60, 61,
           62, 63, 64, 94, 95, 96, 123, 124, 125,  126,  34,  39, 46, 47, 58, 91, 92, 93]
  BASE = 93

  def __init__(self, base=BASE):
    self.BASE = base
    pass

  def encode(self, number):
    key = ''

    if number == 0:
      return '0'
    while number > 0:
      mod = number % self.BASE
      try:
        key = key + chr((self.CHARS[mod]))
      except:
        print("Mod: {:d}".format(mod))
      number = number // self.BASE
    return key[::-1]

  def decode(self, alphanumeric):
    value = 0
    length = index = len(alphanumeric) - 1
    while index >= 0:
      digit = alphanumeric[index]
      value += self.CHARS.index(ord(digit)) * (self.BASE ** (length - index))
      index -= 1
    return value
