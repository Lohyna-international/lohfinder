import os
from google.cloud import pubsub_v1

os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = os.getcwd() + "/firebase.json"
timeout = 2.0 #secs

subscriber = pubsub_v1.SubscriberClient()

subscription_path = 'projects/lohyna-user-service/subscriptions/create_user-sub'

print(f"Listening messages on {subscription_path}")


def callback(message):
    print(f"Received message : {message}")
    print(f"Data : {message.data}")
    print(type(message))
    message.ack()

user_pull = subscriber.subscribe(subscription_path, callback = callback)

with subscriber:
    try:
        user_pull.result()
    except:
        user_pull.cancel()
        user_pull.result()
