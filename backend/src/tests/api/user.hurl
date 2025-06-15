POST http://localhost:8000/auth/login
Content-Type: application/json
Accept-Language: en

{
  "email": "tono@gmail.com",
  "password": "Securepjjjassword123."
}

DELETE http://localhost:8000/auth/logout
Accept-Language: ja

POST http://localhost:8000/auth/register
Content-Type: application/json
Accept-Language: en

{
  "name": "user",
  "email": "yuni@gmail.com",
  "password": "Securepassword123.",
  "password_confirmation": "Securepassword123.",
  "subscription_plan": "Free"
}

GET http://localhost:8000/users/all
Accept-Language: ja

GET http://localhost:8000/users/h1@gmail.com
Accept-Language: id

DELETE http://localhost:8000/users/h1@gmail.com
Accept-Language: en

PUT http://localhost:8000/users/h1@gmail.com
Content-Type: application/json
Accept-Language: ja

{
  "name": "updatedUser1",
  "email": "h4@gmail.com",
  "subscription_plan": "Pro"
}
