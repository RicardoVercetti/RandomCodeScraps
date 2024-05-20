from sklearn.linear_model import LogisticRegression

# Data is typically divided into two types: 
# Labeled data
# Unlabeled data

# data used in machine learning is typically numerical or categorical.
# Numerical data includes values that can be ordered and measured, 
# such as age or income. Categorical data includes values that represent categories, such as gender or type of fruit.


# example 1

# Imagine you’re working for a car manufacturing company and you want to build a model that can predict the fuel efficiency 
# of a car based on the weight and the engine size. In this case, the target variable (or label) is the fuel efficiency, 
# and the features (or input variables) are the weight and engine size. You will collect data from different car models, 
# with corresponding weight and engine size, and their fuel efficiency. This data is labeled and it’s in the form of 
# (weight,engine size,fuel efficiency) for each car. After having your data ready, you will then split it into two sets: 
# training set and testing set, the training set will be used to train the model and the testing set will be used to evaluate 
# the performance of the model. Preprocessing could be needed for example, to fill missing values or handle outliers that might 
# affect your model accuracy.


# training samples
x_train = [[1, 2], [2, 3], [3, 4], [4, 5], [5, 6]]
y_train = [0, 0, 1, 1, 1]

# Train the model
model = LogisticRegression()
model.fit(x_train, y_train)

prediction = model.predict([[6,7],[2,4],[1,3]])
print(prediction)


# print("Runs..") 