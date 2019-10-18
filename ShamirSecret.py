import random
import functools
from MultiBaseDigitEncoder import MultiBaseEncoder


class ShamirSecret:

  _PRIME = None
  _RANDOM_INT = functools.partial(random.SystemRandom().randint, 0)
  _multi_base_encoder = None

  def __init__(self, prime=2**521-1, multi_base_encoder=MultiBaseEncoder(93)):
    self._PRIME = prime
    self._multi_base_encoder = multi_base_encoder

  def _eval_coefficient_tuple_at_x(self, poly, x):
      accum = 0
      for coeff in reversed(poly):
          accum *= x
          accum += coeff
          accum %= self._PRIME
      return accum

  def make_random_shamir_pool(self, key, minimum, shares):
      if minimum > shares:
          raise ValueError("can't have a threshold higher than the shares")
      poly = [self._RANDOM_INT(self._PRIME) for i in range(minimum)]
      poly[0] = key
      points = [(i, self._eval_coefficient_tuple_at_x(poly, i))
                for i in range(1, shares + 1)]
      return poly[0], points

  def _extended_euclidean_gcd(self, a, b):
      x = 0
      last_x = 1
      y = 1
      last_y = 0
      while b != 0:
          quot = a // b
          a, b = b, a%b
          x, last_x = last_x - quot * x, x
          y, last_y = last_y - quot * y, y
      return last_x, last_y

  def _division_modulo_prime(self,num, den, p):
      inv, _ = self._extended_euclidean_gcd(den, p)
      return num * inv

  def _lagrange_interpolation(self, x, x_s, y_s):
      k = len(x_s)
      assert k == len(set(x_s)), "points for the lagrange interpolation are not distinct"

      def compute_products(values):
          accumulator = 1
          for val in values:
              accumulator *= val
          return accumulator
      nums = []
      dens = []
      for i in range(k):
          others = list(x_s)
          cur = others.pop(i)
          nums.append(compute_products(x - o for o in others))
          dens.append(compute_products(cur - o for o in others))
      den = compute_products(dens)
      num = sum([self._division_modulo_prime(nums[i] * den * y_s[i] % self._PRIME, dens[i], self._PRIME)
                 for i in range(k)])
      return (self._division_modulo_prime(num, den, self._PRIME) + self._PRIME) % self._PRIME

  def recover_secret(self, shares):
      if len(shares) < 2:
          raise ValueError("cannot use less than two shares")
      x_s, y_s = zip(*shares)
      return self._lagrange_interpolation(0, x_s, y_s)
