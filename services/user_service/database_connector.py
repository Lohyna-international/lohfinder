import firebase_admin
from firebase_admin.exceptions import FirebaseError
from firebase_admin import credentials
from firebase_admin import db

#TODO: add returns for methods like save/update

class DatabaseAdmin:

    def __init__(self, databaseURL, credentials_path):
        cred = credentials.Certificate(credentials_path)
        firebase_admin.initialize_app(cred,
        {
            'databaseURL' : databaseURL,
        })
        self.__users_ref = db.reference("users")

    def get_all_users(self):
        try:
            if self.__users_ref.get() is None:
                return list()
            return self.__users_ref.get()
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
            return list()
    

    def get_user_by_id(self, id):
        try:
            if self.__users_ref.child(id).get() is None:
                return list()
            return self.__users_ref.child(id).get()
        except ValueError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
        return list()


    def get_user_by_email(self, email):
        try:   
            for user_id in self.get_user_ids():
                if email == self.__users_ref.child(user_id).child("email").get():          
                    return self.__users_ref.child(user_id).get()
        except ValueError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
        return list()

    def get_user_ids(self):
        try:
            if self.__users_ref.get() is None:
                return list()
            ids = [i for i in self.__users_ref.get()]
            return ids
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
        return list()


    def save_user(self, user_info):
        try:
            return self.__users_ref.push(user_info).key
        except ValueError as e:
            print("###EROR received : " + str(e))
        except TypeError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))

    
    def update_user(self, user_id, user_info):
        try:
            self.__users_ref.child(user_id).set(user_info)
        except ValueError as e:
            print("###EROR received : " + str(e))
        except TypeError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
    

    def delete_user_by_id(self, user_id):
        try:
            self.__users_ref.child(user_id).delete()
        except ValueError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))


    def delete_user_by_email(self, email):
        try:
            for i in self.get_user_ids():
                if email == self.__users_ref.child(i).child("email").get():
                    self.delete_user_by_id(i)
        except ValueError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))


    def drop_users(self):
        try:
            for user_id in self.get_user_ids():
                self.__users_ref.child(user_id).delete()
        except ValueError as e:
            print("###EROR received : " + str(e))
        except FirebaseError as e:
            print("###Connection lost. EROR received : " + str(e))
