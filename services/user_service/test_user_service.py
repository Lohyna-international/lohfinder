
from database_connector import DatabaseAdmin
import mocker


databaseURL = "https://lohyna-user-service-default-rtdb.firebaseio.com/"
credentials_path = "firebase_test.json"
database_manager = DatabaseAdmin(databaseURL, credentials_path)


def test_get_all_users():
    print("Running test test_get_all_users")
    database_manager.drop_users()
    count_of_users = 3
    for i in mocker.get_mocked_users(count_of_users):
        database_manager.save_user(i)
    print(f"STEP 1: count of the users after clearing table should be equal to COUNT_OF_USERS = {count_of_users}")
    assert count_of_users == len(database_manager.get_all_users())


def test_delete_user_by_email():
    print("Running test test_delete_user_by_email")
    user = mocker.get_mocked_users()[0]
    user_email = user["email"]
    database_manager.save_user(user)
    print("STEP 1: user has been added into database")
    assert database_manager.get_user_by_email(user_email)["email"] == user_email
    database_manager.delete_user_by_email(user_email)
    print("STEP 2: After deleting by email user is not more stored")
    assert database_manager.get_user_by_email(user_email) == None


def test_delete_user_by_id():
    print("Running test test_delete_user_by_id")
    user = mocker.get_mocked_users()[0]
    user_id = database_manager.save_user(user)
    print("STEP 1: User has been saved into database")
    assert database_manager.get_user_by_id(user_id) != []
    database_manager.delete_user_by_id(user_id)
    print("STEP 2: User has been deleted from database")
    assert database_manager.get_user_by_id(user_id) == []


def test_create_user():
    print("Running test test_create_user")
    user = mocker.get_mocked_users()[0]
    id = database_manager.save_user(user)
    print("STEP 1: User has been saved into database")
    assert database_manager.get_user_by_id(id) != None


def test_update_user():
    print("Running test test_update_user")
    user = mocker.get_mocked_users()[0]
    id = database_manager.save_user(user)
    print("STEP 1: User has been saved into database")
    assert database_manager.get_user_by_id(id) != None
    user["age"] = 100
    user["email"] = "test@gmail.com"
    user["interests"].append("one more thing")
    database_manager.update_user(id, user)
    updated_user = database_manager.get_user_by_id(id)
    print("STEP 2: User age has been changed")
    assert updated_user["age"] == 100
    print("STEP 2: User email has been changed")
    assert updated_user["email"] == "test@gmail.com"
    print("STEP 4: User interests has been changed")
    assert "one more thing" in updated_user["interests"]


def test_drop_users():
    print("Running test test_drop_users")
    count_of_users = 10
    users = mocker.get_mocked_users(count_of_users)
    print("STEP 1: Database contains users")
    assert database_manager.get_all_users() != []
    for i in users: 
        database_manager.save_user(i)
    database_manager.drop_users()
    print(f"STEP 2: After populating {count_of_users} database is cleared")
    assert database_manager.get_all_users() == []
