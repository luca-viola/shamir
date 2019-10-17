class MersennePrimes:
  _EXPONENTS_OF_TWO = [2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607, 1279, 2203, 2281, 3217, 4253]

  def __init__(self):
    pass

  def get_mersenne_prime(self, n):
    return 2 ** self._EXPONENTS_OF_TWO[n] - 1
