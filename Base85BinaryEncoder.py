import base64


class Base85IntToBytesEncoder:

  def __init__(self):
    pass

  def encode(self, number):
    i = number
    all_bytes = []
    while i > 0:
      byte = i % 256
      all_bytes.append(byte)
      i = i // 256
    all_bytes.reverse()
    binary = bytearray(all_bytes)
    ret=base64.b85encode(binary,False)
    return ret

  def decode(self,base85text):
    binary=base64.b85decode(base85text)
    all_bytes=list(binary)
    all_bytes.reverse()
    number = 0
    for i in range(len(all_bytes)):
      number = number+(all_bytes[i]*(256**i))
    return number
