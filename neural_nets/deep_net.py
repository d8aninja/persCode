# sentdex p4
import tensorflow as tf

'''
input > weight > hidden layer 1 (activation function) > weights > hl2 (af) > output layer
^ this is feed forward

compare output to intended output > cost or loss function (eg cross-entropy: "how not close are we to intended target)
optimziation function (optimizer) > minimize cost (eg AdamOptimizer, SGD, AdaGrad)
^ this is back propogation

feed forward + back prop = epoch
'''
from tensorflow.examples.tutorials.mnist import input_data

mnist = input_data.read_data_sets("/tmp/data/", one_hot = True)
'''
10 classes 0-9
0 = [1,0,0,0,0,0,0,0,0,0]
1 = [0,1,0,0,0,0,0,0,0,0]
2 = [0,0,3,0,0,0,0,0,0,0]...
'''
n_nodes_hl1 = 500
n_nodes_hl2 = 500
n_nodes_hl3 = 500 # don't have to be identical

n_classes = 10
batch_size = 100

# height by width
x = tf.placeholder('float', [None, 784]) # data: 28x28 pixels
y = tf.placeholder('float') # label

def neural_network_model(data):
    hidden_1_layer = {
        'weights':tf.Variable(tf.random_normal([784, n_nodes_hl1])),
        'biases':tf.Variable(tf.random_normal([n_nodes_hl1]))
    }
    hidden_2_layer = {
        'weights':tf.Variable(tf.random_normal([n_nodes_hl1,n_nodes_hl2])),
        'biases':tf.Variable(tf.random_normal([n_nodes_hl2]))
    }
    hidden_3_layer = {
        'weights':tf.Variable(tf.random_normal([n_nodes_hl2,n_nodes_hl3])),
        'biases':tf.Variable(tf.random_normal([n_nodes_hl3]))
    }
    output_layer = {
        'weights':tf.Variable(tf.random_normal([n_nodes_hl3,n_classes])),
        'biases':tf.Variable(tf.random_normal([n_classes]))
    }
    # (input_data * weights) + biases
    l1 = tf.math.add(tf.math.multiply(data, hidden_1_layer['weights']), hidden_1_layer['biases'])
    l1 = tf.nn.relu(l1)

    l2 = tf.math.add(tf.math.multiply(l1, hidden_2_layer['weights']), hidden_2_layer['biases'])
    l2 = tf.nn.relu(l2)

    l3 = tf.math.add(tf.math.multiply(l2, hidden_3_layer['weights']), hidden_3_layer['biases'])
    l3 = tf.nn.relu(l3)

    output = tf.add(tf.math.multiply(l3, output_layer['weights']), output_layer['biases'])

    return output
# sentdex p5
def train_neural_network(x):
    prediction = neural_network_model(x)
    cost = tf.reduce_mean( tf.nn.softmax_cross_entropy_with_logits(prediction, y) )

    # learning_rate = 0.001
    optimizer = tf.train.AdamOptimize().minimize(cost) #SGD

    # cycles of ff and backprop
    hm_epochs = 10

    with tf.Session() as sess:
        sess.run(tf.initialize_all_variables())

        for epoch in range(hm_epochs):
            epoch_loss = 0
            for _ in range(int(mnist.train.num_examples/batch_size)):
                epoch_x, epoch_y = mnist.train.next_batch(batch_size) #prebuilt or mnist; you'd have to make your own batcher
                _, c = sess.run([optimizer, cost], feed_dic = {x:epoch_x, y:epoch_y})
                epch_loss += c
            print('Epoch ', epoch, ' completed out of ', hm_epochs, '. Loss: ', epoch_loss)
        
        correct = tf.equal(tf.argmax(prediction,1), tf.argmax(y,1))

        accuracy = tf.reduce_mean(tf.cast(correct, 'float'))
        print('Accuracy: ', accuracy.eval({x:mnist.test.images, y:mnist.test.labels}))

train_neural_network(x)