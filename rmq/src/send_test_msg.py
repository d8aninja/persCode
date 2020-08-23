import pika

connection = pika.BlockingConnection(
    pika.ConnectionParameters('localhost')
)
channel = connection.channel()

channel.exchange_declare('jeff_test')
channel.queue_declare('hello')

channel.basic_publish(exchange='jeff_test', routing_key='hello', body='Hello World!')

print('[x] Sent Hello World!')

channel.close()
