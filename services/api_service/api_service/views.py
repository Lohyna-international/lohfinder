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
        "method name" : "method url"
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
