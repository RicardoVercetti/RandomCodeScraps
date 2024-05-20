from sklearn.datasets import make_blobs
from matplotlib import pyplot as plt
from matplotlib import style

style.use("fivethirtyeight")


x_samples, y_samples = make_blobs(n_samples = 100, centers = 3, cluster_std = 1, n_features = 2)
# plt.scatter(x_samples[:,0], x_samples[:,1], s = 40, color="g")

# plt.xlabel("X")
# plt.ylabel("Y")

# plt.show()
# plt.clf()


print("X samples : ")
print(x_samples)
print("Y samples : ")
print(y_samples)

