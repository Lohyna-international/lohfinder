from constants import database_admin, pubsub_manager, RESULT_TOPIC
import json


def ack_message(message):
    print(f"Received message : {message}")
    message.ack()
    return message.attributes["message_id"]


def get_users_callback(message):
    message_id = ack_message(message)
    users = database_admin.get_all_users()
    response = json.dumps({"status_code":404, "result": {}}) if len(users) == 0 else json.dumps({"status_code" : 200, "result":users})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_id_callback(message):
    message_id = ack_message(message)
    user_id = message.attributes["user_id"]
    user = database_admin.get_user_by_id(user_id)
    response = json.dumps({"status_code":200, "result": user}) if user is not None else json.dumps({"status_code":404, "result": {}})
    print(response)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def get_user_by_email_callback(message):
    message_id = ack_message(message)
    email = message.attributes["email"]
    print("Received an email" + str(email))
    user = database_admin.get_user_by_email(email)
    response = json.dumps({"status_code":200, "result": user}) if user is not None else json.dumps({"status_code":404, "result": {}})
    print(response)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def save_user_callback(message):
    message_id = ack_message(message)
    user = json.loads(message.data.decode("utf-8"))
    result = database_admin.save_user(user)
    response = json.dumps({"status_code":200, "result":result }) if result is not None else json.dumps({"status_code":404, "result": {}})
    print(response)
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def update_user_callback(message):
    message_id = ack_message(message)
    user = json.loads(message.data.decode("utf-8"))
    user_id = message.attributes["user_id"]
    result = database_admin.update_user(user_id, user)
    response = json.dumps({"status_code":200, "result":result }) if result else json.dumps({"status_code":404, "result": {}})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def delete_user_by_id_callback(message):
    message_id = ack_message(message)
    user_id = message.attributes["user_id"]
    result = database_admin.delete_user_by_id(user_id)
    response = json.dumps({"status_code":200, "result":result }) if result else json.dumps({"status_code":404, "result": {}})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def delete_user_by_email_callback(message):
    message_id = ack_message(message)
    email = message.attributes["email"]
    result = database_admin.delete_user_by_email(email)
    response = json.dumps({"status_code":200, "result":result }) if result else json.dumps({"status_code":404, "result": {}})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)


def drop_users_callback(message):
    message_id = ack_message(message)
    result = database_admin.drop_users()
    response = json.dumps({"status_code":200, "result":result }) if result else json.dumps({"status_code":404, "result": {}})
    pubsub_manager.publish(RESULT_TOPIC, response, message_id)
