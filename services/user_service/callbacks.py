from constants import database_admin, pubsub_manager, RESULT_TOPIC
import json


def ack_message(message):
    print(f"Received message : {message}")
    print(f"Data : {message.data}")
    message.ack()
    return message.attributes["message_id"]


def get_users_callback(message):
    message_id = ack_message(message)
    users = database_admin.get_all_users()
    response = json.dumps(users)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_id_callback(message):
    message_id = ack_message(message)
    user_id = json.loads(message.data)["user_id"]
    user = database_admin.get_user_by_id(user_id)
    response = json.dumps(user)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_email_callback(message):
    message_id = ack_message(message)
    email = json.loads(message.data)["email"]
    user = database_admin.get_user_by_email(email)
    response = json.dumps(user)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def save_user_callback(message):
    message_id = ack_message(message)
    user = json.loads(message.data)
    result = database_admin.save_user(user)
    response = json.dumps({"user_id": result})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def update_user_callback(message):
    ack_message(message)
    user = json.loads(message.data)
    database_admin.update_user(user["id"], user)


def delete_user_by_id_callback(message):
    ack_message(message)
    user_id = json.loads(message.data)["id"]
    database_admin.delete_user_by_id(user_id)


def delete_user_by_email_callback(message):
    ack_message(message)
    email = json.loads(message.data)["email"]
    database_admin.delete_user_by_email(email)


def drop_users_callback(message):
    ack_message(message)
    database_admin.drop_users()
