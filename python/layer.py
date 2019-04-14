weights = [[0.3154923, -1.13972652, 0.68031687, 0.99513912, -0.30351412],
           [0.46209338, 1.35546792, 0.40179586, -1.00812435, -0.15794396]]
biases = [4.45677906e-01, 8.93484321e-06, 4.50525820e-01, -2.59114859e-06, 0.00000000e+00]


def relu(x):
    return max(x, 0)


inp = input()
inp.replace("\n", "")
inp = inp.split(" ")
vec = list([float(f) for f in inp])

outp_vec = []
for j in range(len(biases)):
    o = 0
    for i in range(len(weights)):
        o += vec[i] * weights[i][j]
    o += biases[j]
    o = relu(o)
    outp_vec.append(o)
print(" ".join(["%.8f" % f for f in outp_vec]))
