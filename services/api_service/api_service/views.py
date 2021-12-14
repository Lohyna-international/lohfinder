from api_service.pubsub import PubSubManager
from rest_framework.decorators import api_view
from rest_framework.response import Response
import random
import json
import time
import threading


SUBSCRIPTIONS_PATH = "projects/lohfinder-app/subscriptions/"
PROJECT_ID = "lohfinder-app"
pubsub_manager = PubSubManager(SUBSCRIPTIONS_PATH, PROJECT_ID)

MIN_INT = 100000
MAX_INT = 1000000


result = ""

def ack_message(message):
    print(f"Received message : {message.data}")
    message.ack()
    global result
    result = json.loads(message.data)
    

@api_view(['GET'])
def index(request):
    context = {
        "method name" : "method url"
    }
    return Response(context) 


@api_view(['GET'])
def get_all_users(request):
    message_id = str(random.randint(MIN_INT, MAX_INT))
    pubsub_manager.publish("users_get", "", message_id)
    threading.Thread(target = pubsub_manager.subscribe, args=["user_service_result-sub", ack_message]).start()
    time.sleep(3)
    global result
    return Response(result)