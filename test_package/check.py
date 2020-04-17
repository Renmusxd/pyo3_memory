import pyo3_check
import time

nvars = 25*25
timesteps = 10000
duplicates = 16

# Make some output in rust, convert to python.
output = pyo3_check.make_output(nvars, timesteps, duplicates)
time.sleep(3)
del output

# make the same output in raw python
output = [([[False for _ in range(nvars)] for _ in range(timesteps)], 0.0) for _ in range(duplicates)]
time.sleep(3)
del output

# Make a large array of bools
# output = pyo3_check.make_simple_output(nvars * timesteps * duplicates)
# time.sleep(3)
# del output
#
# # And in pure python
# output = [False for _ in range(nvars * timesteps * duplicates)]
# time.sleep(3)
# del output

