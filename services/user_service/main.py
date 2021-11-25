from pubsub import PubSubManager
import multiprocessing
import callbacks

print("User Service entry point")

subs = {
    "sub_one" : callbacks.create_user_callback,
    "test" : callbacks.update_user_callback,
    "users_get-sub" : callbacks.get_users_callback,
}


if __name__ == "__main__":
    pubsub = PubSubManager("projects/lohfinder-app/subscriptions/", "lohfinder-app")
    for i in subs:
        multiprocessing.Process(target=pubsub.subscribe, args=[i, subs[i]]).start()