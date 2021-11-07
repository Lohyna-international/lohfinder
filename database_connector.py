import json
import firebase_admin
from firebase_admin import credentials
from firebase_admin import db

#implements CRUD operations interface
class DatabaseAdmin:

    def __init__(self,databaseURL, credentials_path):
        cred = credentials.Certificate(credentials_path)
        firebase_admin.initialize_app(cred,
        {
            'databaseURL' : databaseURL,
        })
        self.__users_ref = db.reference("users")


    def get_all_users(self):
        return self.__users_ref.get()
    

    def get_user_by_email(self, email):
        for user_id in self.get_user_ids():
            if email == self.__users_ref.child(user_id).child("email").get():
                return self.__users_ref.child(user_id).get()
    

    def get_user_ids(self):
        ids = [i for i in self.__users_ref.get()]
        return ids


    def save_user(self, user_info):
        self.__users_ref.push(user_info)

    
    def update_user(self, user_id, user_info):
        self.__users_ref.child(user_id).set(user_info)

    def delete_user_by_id(self, user_id):
        self.__users_ref.child(user_id).delete()
    

    def delete_user_by_email(self, email):
        for i in self.__users_ref.get_user_ids():
            if email == self.__users_ref.child(i).child("email").get():
                self.delete_user_by_id(i)


    def drop_users(self):
        for user_id in self.get_user_ids():
            self.__users_ref.child(user_id).delete()
    

databaseURL = "https://lohyna-user-service-default-rtdb.firebaseio.com"
credentials_path = "firebase.json"
database_manager = DatabaseAdmin(databaseURL, credentials_path)

with open("data.json", "r") as f:
    users = json.load(f)['users']
    for i in users:
        database_manager.save_user(i)
    f.close()

print(database_manager.get_all_records())