import random
from faker import Faker

images = [
    "https://www.google.com/url?sa=i&url=https%3A%2F%2Fwww.vectorstock.com%2Froyalty-free-vector%2Fflat-business-man-user-profile-avatar-icon-vector-4333097&psig=AOvVaw0WMass4DSBWEHhXOqpRWN5&ust=1637053775349000&source=images&cd=vfe&ved=0CAsQjRxqFwoTCNi7qNuCmvQCFQAAAAAdAAAAABAP",
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSHL-ELbysJgVyYIJvfLMMbCZFxuLkqKe_iYBtHcvFw1VJX2RjlLA1xXina7Mn75Puo5Yw&usqp=CAU",
    "https://www.seekpng.com/png/full/356-3562377_personal-user.png",
    "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSx7FfWrqJ5ro7SdxlBsnmCo_mwrnRly5mHUg&usqp=CAU"
]

def get_mocked_users(N = 1):
    fake = Faker()
    users = list()
    for i in range(N):
        users.append(
            {
                "name" : fake.first_name(),
                "surname" : fake.last_name(),
                "password" : fake.password(),
                "email" : fake.email(),
                "phone" : fake.phone_number(),
                "photo" : random.choice(images),
                "age" : random.randint(1, 100),
                "description" : fake.text(),
                "interests" : fake.words(),
            }
        )
    return users
