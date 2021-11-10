import os
import json
from google.cloud import pubsub_v1

os.environ["GOOGLE_APPLICATION_CREDENTIALS"] = os.getcwd() + "/firebase.json"


project_id = "lohyna-user-service"
topic_id = "create_user"

publisher = pubsub_v1.PublisherClient()
topic_path = publisher.topic_path(project_id, topic_id)

data = {
    1: "Data for the first method",
    2: "Data for the second method",
}
message = json.dumps(data).encode("utf-8")

res = publisher.publish(topic_path, message)

print(f"published message: {res.result()}")