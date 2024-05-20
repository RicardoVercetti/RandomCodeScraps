from sklearn.linear_model import LogisticRegression, LinearRegression


# fn : 2x + 3x


x_train = [[8], [4]]
y_train = [40, 20]


# model = LogisticRegression()
model = LinearRegression()
model.fit(x_train, y_train)


prediction = model.predict([[115]])


print("Prediction : ", prediction)





