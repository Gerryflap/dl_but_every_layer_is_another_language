# This file is used to train the network and get it's parameters
import keras as ks
from keras.layers import Dense
import numpy as np

X = np.array([[0, 0], [0, 1], [1, 0], [1, 1]])
Y = np.array([[0], [1], [1], [0]])

model = ks.models.Sequential()
model.add(Dense(5, input_shape=(2,), activation='relu')) # Python
model.add(Dense(5, activation='relu'))  # Haskell
model.add(Dense(5, activation='relu'))  # Rust
model.add(Dense(5, activation='relu'))  # Java
model.add(Dense(1, activation='sigmoid'))  # C

model.compile(optimizer=ks.optimizers.Adam(0.01), loss=ks.losses.binary_crossentropy)
model.fit(X, Y, epochs=1000)
print(model.get_weights())