import pika
import sys

def get_conn(host: str = 'localhost') -> pika.BlockingConnection:
    connection = pika.BlockingConnection(
        pika.ConnectionParameters(host=host))
    return connection

def get_channel(conn: pika.BlockingConnection) -> pika.BlockingConnection.channel:
    channel = conn.channel()

def declare_exchange(
    channel: pika.BlockingConnection.channel,
    name: str,
    exchange_type: str):
    channel.exchange_declare(exchange='direct_logs', exchange_type='direct')
    return channel.queue_declare(queue='', exclusive=True)
    # return result.method.queue

def callback(ch, method, properties, body):
    print(" [x] %r:%r" % (method.routing_key, body))

def main():
    channel = conn_and_channel()
    severities = sys.argv[1:]
    if not severities:
        sys.stderr.write("Usage: %s [info] [warning] [error]\n" % sys.argv[0])
        sys.exit(1)

    for severity in severities:
        channel.queue_bind(
            exchange='direct_logs', queue=queue_name, routing_key=severity)

    print(' [*] Waiting for logs. To exit press CTRL+C')

    channel.basic_consume(
        queue=queue_name, on_message_callback=callback, auto_ack=True)

    channel.start_consuming()