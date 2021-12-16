import threading
import callbacks

from constants import pubsub_manager

subs = {
    "users_get-sub": callbacks.get_users_callback,
    "user_get_by_id-sub": callbacks.get_user_by_id_callback,
    "user_get_by_email-sub": callbacks.get_user_by_email_callback,
    "user_save-sub": callbacks.save_user_callback,
    "user_update-sub": callbacks.update_user_callback,
    "users_delete_all-sub": callbacks.drop_users_callback,
    "user_delete_by_id-sub": callbacks.delete_user_by_id_callback,
    "user_delete_by_email-sub": callbacks.delete_user_by_email_callback,
}

if __name__ == "__main__":
    for i in subs:
        threading.Thread(target=pubsub_manager.setup_subscriber, args=[i, subs[i]]).start()
