from api_service.pubsub import PubSubManager
from rest_framework.decorators import api_view
from rest_framework.response import Response
import random
import json
import time
import threading
import uuid
from api_service.result_watcher import ResultWatcher


SUBSCRIPTIONS_PATH = "projects/lohfinder-app/subscriptions/"
PROJECT_ID = "lohfinder-app"
pubsub_manager = PubSubManager(SUBSCRIPTIONS_PATH, PROJECT_ID)
users_result_watcher = ResultWatcher(pubsub_manager, "user_service_result-sub", lambda m : m.attributes["message_id"])

def get_id():
    return str(uuid.uuid4().int & (1<<64)-1)

@api_view(['GET'])
def index(request):
    context = {
        "Get all users" : "/api/get-all-users",
        "Delete all users" : "/api/delete-all-users",
        "Update user" : "/api/update-user/<user_id:int>",
        "Save user" : "/api/save-user",
        "Get user by id" : "/api/get-user-by-id/<user_id:int>",
        "Get user by email" : "/api/get-user-by-email/<email:str>",
        "Delete user by id" : "/api/delete-user-by-id/<user_id:int>",
        "Delete user by email" : "/api/delete-user-by-email/<email:str>",
        "Get all events" : "/api/get-all-events",
        "Update an event" : "/api/update-event/<event_id:int>",
        "Get event by id" : "/api/get-event/<event_id:int>",
        "Delete event" : "/api/delete-event/<event_id:int>",
        "Create event" : "/api/create-event",
        "Update event application form" : "/api/update-event-application/<ea_id:int>",
        "Create event application form" : "/api/create-event-application/",
        "Delete event application form" : "/api/delete-event-application/<ea_id:int>",
    }
    return Response(context) 


@api_view(['GET'])
def get_all_users(request):
    message_id = get_id()
    pubsub_manager.publish("users_get", "", message_id)
    mutex = threading.Lock()
    users_result_watcher.add_watcher(mutex, message_id)
    with mutex:
        return Response(users_result_watcher.get_result(message_id))
