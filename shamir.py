#!/usr/bin/env python3
import getopt
import getpass
import re
import sys
import clipboard
from ShamirSecret import ShamirSecret
from MersennePrimes import MersennePrimes
from Base85BinaryEncoder import Base85IntToBytesEncoder
from MultiBaseDigitEncoder import MultiBaseEncoder

_DEFAULT_MIN = 3
_DEFAULT_SHARES = 6
_base62 = MultiBaseEncoder(62)
_base93 = MultiBaseEncoder(93)
_PRIME = MersennePrimes().get_mersenne_prime(13)
_shamir_secret = ShamirSecret(_PRIME, _base93)


def print_usage_and_exit(status=1):
    print("Shamir secret sharing - V2019.1 by Luca Viola")
    print("")
    print("Usage: shamir (no args, uses defaults [-t 3 -s 6] and prompts for the key)")
    print("       shamir [-h|--help] : prints this help and exits")
    print("       shamir -t|--threshold= <minimum> -s|--shares= <shares> -k|--key= <key>")
    print("       shamir -r|--reconstruct= <minimum_shares>")
    print("")
    print(" -t|--threshold: the minimum # of shares needed to reconstruct the original key")
    print(" -s|--shares: the maximum # of shares to be generated")
    print(" -k|--key: the password to be shared in base 62 encoding [A-Za-z0-9]")
    print("")
    sys.exit(status)


def _reconstruct(minimum, prime=_PRIME):
  count = 1
  poly = []
  regex = r"""^(\d*)\-([0-9a-zA-Z!#$%&()*+-;<=>?@^_`{|}~\"\',.\/:\[\]\\]*)$"""

  while count <= minimum:
    p = getpass.getpass("Insert share #{:d}/{:d}: ".format(count, minimum))
    z = bool(re.match(regex, p))
    if not z:
      print("Invalid key")
      continue
    p0 = p.partition('-')[0].strip()
    p1 = p.partition('-')[2].strip()
    poly.append((int(p0), _base62.decode(p1)))
    count += 1
  secret = _base93.encode(_shamir_secret.recover_secret(poly))
  return secret


def main():
    minimum = None
    shares = None
    key = ""

    try:
      opts, args = getopt.getopt(sys.argv[1:], "ht:s:k:r:", ["help", "threshold=", "shares=", "key="])
    except getopt.GetoptError as err:
      print(err)
      print_usage_and_exit()

    for o, a in opts:
      if o in ("-h", "--help"):
        print_usage_and_exit(0)
      elif o in ("-t", "--threshold"):
        minimum = a
      elif o in ("-s", "--shares"):
        shares = a
      elif o in ("-k", "--key"):
        key = a
      elif o in ("-r", "--reconstruct"):
        minimum = int(a)
        secret = _reconstruct(minimum)
        clipboard.copy(secret)
        print("The secret has been copied to the clipboard")
        sys.exit(0)
      else:
        print_usage_and_exit()

    if key == '':
      try:
        p = getpass.getpass("Insert password: ")
        q = getpass.getpass("Repeat password: ")
        if p != q:
          print("The passwords don't match!")
          sys.exit(1)
        else:
          key = p
      except Exception as error:
        print('ERROR', error)

    keycode = _base93.decode(key.strip())

    if minimum is None:
      minimum = _DEFAULT_MIN
    if shares is None:
      shares = _DEFAULT_SHARES

    minimum = int(minimum)
    shares = int(shares)

    print("Generating {:d} shares with a minimum of {:d} shares required".format(shares, minimum))
    secret, shares = _shamir_secret.make_random_shamir_pool(keycode, minimum, shares)

    print('shares:')
    if shares:
        for share in shares:
            print('  ', str(share[0]) +"-" + str(_base62.encode(share[1])))

    test1 = _base62.encode(_shamir_secret.recover_secret(shares[:minimum]))
    test2 = _base62.encode(_shamir_secret.recover_secret(shares[-minimum:]))

    if test1 == test2 and test1 == _base62.encode(secret):
      print("Minimum shares reconstruction test passed, generation complete.")
    else:
      print("Minimum shares reconstruction test not passed.")


if __name__ == '__main__':
    main()

