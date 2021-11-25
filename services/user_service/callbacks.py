from pubsub import PubSubManager
from database_connector import DatabaseAdmin
import json


pubsub_manager = PubSubManager("projects/lohfinder-app/subscriptions/", "lohfinder-app")
databaseURL = "https://lohyna-user-service-default-rtdb.firebaseio.com/"
credentials_path = "firebase_test.json"
database_admin = DatabaseAdmin(databaseURL, credentials_path)

topics = [
    "users_get",
]


def get_users_callback(message):
    print(f"Received message : {message}")
    print(f"Data : {message.data}")
    message.ack()
    users = database_admin.get_all_users()
    response = json.dumps(users)
    pubsub_manager.publish(topics[0], response)


def get_user_by_id_callback(message):
    pass


def get_user_by_email_callback(message):
    pass


def save_user_callback(message):
    pass


def update_user_callback(message):
    pass


def delete_user_by_id_callback(message):
    pass


def delete_user_by_email_callback(message):
    pass


def drop_users_callback(message):
    pass
