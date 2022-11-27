import matplotlib.pyplot as plt
import pandas
import matplotlib
matplotlib.use('svg')

df = pandas.read_csv("2-opt.txt")
fig = plt.figure()
ax = fig.add_subplot()
df.plot(ax=ax)
plt.savefig("plot.png", dpi=200)
plt.close()