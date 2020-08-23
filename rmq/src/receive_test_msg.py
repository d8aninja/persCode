import pika

connection = pika.BlockingConnection(
    pika.ConnectionParameters('localhost')
)
channel = connection.channel()

channel.exchange_declare('jeff_test')
channel.queue_declare('hello')

def callback(ch, method, properties, body):
    print('[x] received %s', body)

channel.basic_consume(queue='hello', on_message_callback=callback, auto_ack=True)
print('[x] waiting for messages, press ctrl-c to continue...')

channel.start_consuming()
