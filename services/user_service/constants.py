from database_connector import DatabaseAdmin
from pubsub import PubSubManager

DATABASE_URL = "https://lohfinder-app-default-rtdb.firebaseio.com/"
FIREBASE_KEYS = "firebase.json"
database_admin = DatabaseAdmin(DATABASE_URL, FIREBASE_KEYS)

SUBSCRIPTIONS_PATH = "projects/lohfinder-app/subscriptions/"
PROJECT_ID = "lohfinder-app"
pubsub_manager = PubSubManager(SUBSCRIPTIONS_PATH, PROJECT_ID)

RESULT_TOPIC = "user_service_result"
