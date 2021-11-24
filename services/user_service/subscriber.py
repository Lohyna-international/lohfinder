import os
import time
import json
import multiprocessing
from google.cloud import pubsub_v1

os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = os.getcwd() + "/google_cloud.json"


class PubSubManager():
    
    def __init__(self, sub_path, project_id):
        self.__subscriber = pubsub_v1.SubscriberClient()
        self.__publisher = pubsub_v1.PublisherClient()
        self.__subscriber_path = sub_path
        self.__project_id = project_id
    
    def subscribe(self, subscription):
        path = self.__subscriber_path + subscription
        
        print(f"Listening messages on {path}")
        def callback(message):
            print(f"Received message : {message}")
            print(f"Data : {message.data}")
            print(type(message))
            message.ack()

        user_pull = self.__subscriber.subscribe(path, callback = callback)

        with self.__subscriber:
            try:
                user_pull.result()
            except:
                user_pull.cancel()
                user_pull.result()

    def publish(self, topic):
        topic_path = self.__publisher.topic_path(self.__project_id, topic)
        print("asdas")
        data = {
            1: "Data for the first method",
        }

        message = json.dumps(data).encode("utf-8")

        res = self.__publisher.publish(topic_path, message)
        print(f"published message: {res.result()}")


    def list_topics(self):
        project_path = f"projects/{self.__project_id}"
        return self.__publisher.list_topics(
            request = {
                "project" : 'projects/lohfinder-app',
            }
        )
        

pubsub = PubSubManager("projects/lohfinder-app/subscriptions/", "lohfinder-app")
print(pubsub.list_topics())
# multiprocessing.Process(target=pubsub.subscribe, args=["test"]).start()
# multiprocessing.Process(target=pubsub.subscribe, args=["events"]).start()
# for i in range(3):
#     pubsub.publish("events")
#     time.sleep(2)
#     pubsub.publish("hello_topic")
#     time.sleep(2)