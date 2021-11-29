import os
import threading
import json
from google.cloud import pubsub_v1

os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = os.getcwd() + "/google_cloud.json"


class PubSubManager():
    
    def __init__(self, sub_path, project_id):
        self.__subscriber = pubsub_v1.SubscriberClient()
        self.__publisher = pubsub_v1.PublisherClient()
        self.__subscriber_path = sub_path
        self.__project_id = project_id
    

    def subscribe(self, subscription, callback):
        path = self.__subscriber_path + subscription
        
        print(f"Listening messages on {path}")

        user_pull = self.__subscriber.subscribe(path, callback = callback)

        with self.__subscriber:
            try:
                user_pull.result()
            except:
                user_pull.cancel()
                user_pull.result()


    def publish(self, topic, message, message_id):
        topic_path = self.__publisher.topic_path(self.__project_id, topic)
        res = self.__publisher.publish(topic_path, message.encode("utf-8"), message_id = message_id)
        print(f"published message: {res.result()}")


# def callback(message):
#     print(f"Received message : {message}")
#     print(f"Data : {message.data}")
#     res = json.loads(message.data)
#     print(res)
#     res = message.attributes["message_id"]
#     print(f"Atributes {res}")
#     message.ack()

# pubsub = PubSubManager("projects/lohfinder-app/subscriptions/", "lohfinder-app")
# threading.Thread(target = pubsub.subscribe, args = ["user_service_result-sub", callback]).start()
# a = {"1":"dasdasdsa", "2":"2312"}
# threading.Thread(target = pubsub.publish, args = ["user_service_result",json.dumps(a), "asdas"]).start()