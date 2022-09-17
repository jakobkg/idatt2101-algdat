import seaborn as sns
import matplotlib.pyplot as plt

sns.set_theme()

data = [(5.550, 489),
(5.551, 493),
(5.552, 498),
(5.553, 495),
(5.554, 498),
(5.555, 486),
(5.556, 485),
(5.557, 501),
(5.558, 530),
(5.559, 491),
(5.560, 490),
(5.561, 495)]

x = []
y = []

for point in data:
    x.append(point[0])
    y.append(point[1])

plt.plot(x, y)
plt.xlabel('s')
plt.ylabel('Tid [ms]')
plt.savefig('plot.png')
