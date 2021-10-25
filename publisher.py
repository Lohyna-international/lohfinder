
import os
from google.cloud import pubsub_v1

os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = os.getcwd() + "/lohyna-user-service-584c234c30e6.json"

project_id = "lohyna-user-service"
topic_id = "create_user"

publisher = pubsub_v1.PublisherClient()
topic_path = publisher.topic_path(project_id, topic_id)

data = "Hello Google".encode('utf-8')

res = publisher.publish(topic_path, data)

print(f"published message: {res.result()}")