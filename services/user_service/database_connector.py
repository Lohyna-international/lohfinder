import firebase_admin
from firebase_admin.exceptions import FirebaseError
from firebase_admin import credentials
from firebase_admin import db


class DatabaseAdmin:
    def __init__(self, databaseURL, credentials_path):
        cred = credentials.Certificate(credentials_path)
        firebase_admin.initialize_app(
            cred,
            {
                "databaseURL": databaseURL,
            },
        )
        self.__users_ref = db.reference("users")

    def get_all_users(self):
        try:
            return self.__users_ref.get()
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return list()

    def get_user_by_id(self, id):
        try:
            return self.__users_ref.child(id).get()
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return None

    def get_user_by_email(self, email):
        try:
            for user_id in self.get_user_ids():
                if email == self.__users_ref.child(user_id).child("email").get():
                    return self.__users_ref.child(user_id).get()
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return None

    def get_user_ids(self):
        try:
            ids = [i for i in self.__users_ref.get()]
            return ids
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return list()

    def save_user(self, user_info):
        try:
            return self.__users_ref.push(user_info).key
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except TypeError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return None

    def update_user(self, user_id, user_info):
        try:
            self.__users_ref.child(user_id).set(user_info)
            return True
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except TypeError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return False

    def delete_user_by_id(self, user_id):
        try:
            self.__users_ref.child(user_id).delete()
            return True
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return False

    def delete_user_by_email(self, email):
        try:
            for i in self.get_user_ids():
                if email == self.__users_ref.child(i).child("email").get():
                    self.delete_user_by_id(i)
                    return True
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return False

    def drop_users(self):
        try:
            for user_id in self.get_user_ids():
                self.__users_ref.child(user_id).delete()
            return True
        except ValueError as e:
            print("###ERROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. ERROR received : " + str(e))
        return False
