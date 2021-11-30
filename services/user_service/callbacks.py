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
    response = "There are no users" if len(users) == 0 else json.dumps(users)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_id_callback(message):
    message_id = ack_message(message)
    user_id = json.loads(message.data)["user_id"]
    user = database_admin.get_user_by_id(user_id)
    response = json.dumps(user) if user is not None else "User is not found"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_email_callback(message):
    message_id = ack_message(message)
    email = json.loads(message.data)["email"]
    user = database_admin.get_user_by_email(email)
    response = json.dumps(user) if user is not None else "User is not found"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def save_user_callback(message):
    message_id = ack_message(message)
    user = json.loads(message.data)
    result = database_admin.save_user(user)
    response = json.dumps({"user_id": result}) if user is not None else "Can not save user"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def update_user_callback(message):
    message_id = ack_message(message)
    user = json.loads(message.data)
    result = database_admin.update_user(user["id"], user)
    response = "User is updated" if result else "User is not updated"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)

def delete_user_by_id_callback(message):
    message_id = ack_message(message)
    user_id = json.loads(message.data)["id"]
    result = database_admin.delete_user_by_id(user_id)
    response = "User is deleted" if result else "User is not deleted"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def delete_user_by_email_callback(message):
    message_id = ack_message(message)
    email = json.loads(message.data)["email"]
    result = database_admin.delete_user_by_email(email)
    response = "User is deleted" if result else "User is not deleted"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def drop_users_callback(message):
    message_id = ack_message(message)
    result = database_admin.drop_users()
    response = "User are deleted" if result else "Users are not deleted"
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)
