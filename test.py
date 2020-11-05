from target.release import libmyrustlib

props = libmyrustlib.Properties("aabbccdd", 1)
print(libmyrustlib.count_doubles(props).n())

